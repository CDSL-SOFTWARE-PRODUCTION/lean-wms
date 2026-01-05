# LEAN WMS - BLUEPRINT FOR ENGINEER
## Hướng dẫn kỹ thuật & Kiến trúc hệ thống

**Mục tiêu:** Định nghĩa "Cỗ máy" hoạt động như thế nào - Tốc độ quét và Tính chính xác của dữ liệu.

---

## 1. BUSINESS LOGIC (Quy tắc nghiệp vụ)

### 1.1. Định danh & Quản lý Đối tượng (Entity Management)

#### 1.1.1. Sản phẩm (SKU) - Đối tượng gốc

- **Mỗi SKU có:**
  - SKU Code (Mã nội bộ duy nhất)
  - Tên sản phẩm
  - Đơn vị tính (Cái, Kg, Thùng...)
  - Giá nhập/Giá xuất
  - Hạn sử dụng (nếu có)

- **Logic Alias Mapping:**
  - Một SKU có thể có nhiều Alias (Mã vạch nhà cung cấp A, Mã vạch nhà cung cấp B, Mã QR nội bộ)
  - Quét bất kỳ mã nào cũng ra đúng SKU đó
  - Mapping được tạo 1 lần, dùng mãi

- **Trạng thái SKU:**
  - `ACTIVE`: Đang sử dụng
  - `INACTIVE`: Ngừng sử dụng (nhưng vẫn lưu lịch sử)
  - `DISCONTINUED`: Không còn sản xuất/nhập

#### 1.1.2. Vị trí (Bin Location) - Đối tượng tĩnh

- **Cấu trúc phân cấp:**
  ```
  Kho (Warehouse) 
    → Kệ (Rack) 
      → Tầng (Shelf) 
        → Ô (Bin)
  ```

- **Ví dụ:** `WH01-R01-S02-B03` = Kho 01, Kệ 01, Tầng 02, Ô 03

- **Logic lưu trữ:**
  - **Mixed Bin:** Một Ô có thể chứa nhiều SKU khác nhau
  - **Fixed Bin:** Một Ô chỉ chứa 1 SKU (cho hàng giá trị cao)

- **Mỗi Bin có QR code riêng** để quét khi cất/lấy hàng

#### 1.1.3. Vật chứa (Container/LPN - License Plate Number) - Đối tượng động

- **Loại vật chứa:**
  - Thùng (Box)
  - Pallet
  - Bao tải (Bag)
  - Xe đẩy (Cart)

- **Logic Nesting:**
  - Quét mã Container (QR cha) = Quét toàn bộ hàng bên trong (QR con)
  - Container có thể chứa Container khác (Nested)
  - Khi di chuyển Container, tất cả hàng bên trong tự động di chuyển theo

- **Trạng thái Container:**
  - `EMPTY`: Rỗng, sẵn sàng sử dụng
  - `IN_USE`: Đang chứa hàng
  - `IN_TRANSIT`: Đang di chuyển
  - `ARCHIVED`: Đã hủy/không dùng nữa

### 1.2. Quy trình Kho (Warehouse Process)

#### 1.2.1. Inbound (Nhập kho)

1. **Bước 1:** Quét mã hàng (QR/Barcode)
   - Nếu hàng có mã sẵn → Quét luôn
   - Nếu hàng không có mã → In QR mới từ App

2. **Bước 2:** Mapping (Gán mã)
   - Nếu hàng lạ (chưa có trong hệ thống):
     → Quét Barcode → Hiển thị form "Gán vào SKU nào?"
     → Chọn SKU có sẵn hoặc tạo SKU mới
     → Lưu mapping (1 lần duy nhất)
   - Nếu hàng quen (đã có mapping):
     → Tự động nhận diện SKU

3. **Bước 3:** Nhập thông tin
   - Số lượng
   - Hạn sử dụng (nếu có)
   - Ngày sản xuất (nếu có)
   - Trạng thái: Hàng đạt / Hàng lỗi

4. **Bước 4:** Quét mã vị trí Nhận hàng (Receiving/Staging Location)
   - Thường là khu vực "Nhận hàng", "Bàn phân loại" hoặc "Dock Door"
   - Nếu không quét, hệ thống gán vào vị trí `DEFAULT_STAGING`

5. **Bước 5:** Xác nhận & Lưu kho
   - Nếu Hàng đạt → Trạng thái `STAGING` (Chờ cất)
   - Nếu Hàng lỗi → Trạng thái `DEFECT` (Chờ xử lý/Trả về)
   - Tạo bản ghi `Inventory_Items` mới

#### 1.2.2. Put-away (Cất hàng)

1. **Bước 1:** Quét mã hàng/Container đang ở trạng thái `STAGING`
2. **Bước 2:** Quét mã vị trí đích (Bin Location QR - Vị trí cất cuối cùng)
3. **Bước 3:** Hệ thống kiểm tra:
   
   **Fixed Bin Validation:**
   - Nếu `bin_type` = `FIXED`:
     - Kiểm tra `fixed_product_id` của Bin có khớp với `product_id` của hàng không?
     - Nếu không khớp → Lỗi: "Bin này chỉ dành cho sản phẩm [Tên sản phẩm]. Vui lòng cất vào bin khác."
     - Nếu khớp → Cho phép tiếp tục
   
   **Mixed Bin Validation:**
   - Nếu `bin_type` = `MIXED`:
     - Cho phép cất bất kỳ SKU nào
   
   **Capacity Validation:**
   - Nếu `max_capacity` được set:
     - Tính tổng `quantity` của tất cả Inventory_Items trong Bin hiện tại
     - Kiểm tra: `total_quantity + new_quantity <= max_capacity`
     - Nếu vượt quá → Lỗi: "Bin này đã đầy. Sức chứa tối đa: [max_capacity]. Hiện tại: [total_quantity]"
     - Nếu chưa đầy → Cho phép tiếp tục
   - Nếu `max_capacity` = NULL → Không kiểm tra capacity (unlimited)
   
4. **Bước 4:** Xác nhận → Chuyển trạng thái:
   - Cập nhật `location_id`: Từ Staging Loc → Final Bin Loc
   - Cập nhật `status`: `STAGING` → `AVAILABLE` (Sẵn sàng bán/xuất)
   - Ghi nhận: Ai cất, Lúc nào, Ở đâu

#### 1.2.3. Outbound (Xuất kho)

1. **Bước 1:** Chủ xưởng tạo lệnh xuất (Sales Order/Production Order)
2. **Bước 2:** App công nhân hiển thị danh sách cần lấy:
   - Tên hàng, Số lượng, Vị trí
3. **Bước 3:** Hệ thống chỉ định vị trí theo logic **FEFO/FIFO**:
   
   **FEFO Algorithm (First Expired, First Out):**
   - Áp dụng khi: Product có `expiry_date` (hạn sử dụng)
   - Logic:
     1. Tìm tất cả Inventory_Items có:
        - `product_id` = product cần xuất
        - `status` = `AVAILABLE`
        - `expiry_date` IS NOT NULL
     2. Sắp xếp theo `expiry_date` ASC (hạn sắp hết nhất trước)
     3. Nếu có nhiều items cùng `expiry_date` → Sắp xếp theo `created_at` ASC (FIFO)
     4. Chọn items từ trên xuống cho đến khi đủ `quantity_ordered`
     5. Trả về `location_id` của items được chọn
   
   **FIFO Algorithm (First In, First Out):**
   - Áp dụng khi: Product KHÔNG có `expiry_date` (không có hạn sử dụng)
   - Logic:
     1. Tìm tất cả Inventory_Items có:
        - `product_id` = product cần xuất
        - `status` = `AVAILABLE`
        - `expiry_date` IS NULL
     2. Sắp xếp theo `created_at` ASC (nhập trước nhất trước)
     3. Chọn items từ trên xuống cho đến khi đủ `quantity_ordered`
     4. Trả về `location_id` của items được chọn
   
   **Ví dụ FEFO:**
   - Đơn hàng: Cần xuất 10 cái Áo thun
   - Tồn kho:
     - Bin A1: 5 cái, hạn 2025-12-31
     - Bin A2: 3 cái, hạn 2024-06-30 (hạn sắp hết hơn)
     - Bin A3: 8 cái, hạn 2025-12-31
   - Hệ thống chọn: Bin A2 (3 cái) + Bin A1 (5 cái) + Bin A3 (2 cái)

4. **Bước 4:** Công nhân đến đúng vị trí, quét:
   - Quét mã Bin Location
   - Quét mã hàng/Container
5. **Bước 5:** Hệ thống kiểm tra:
   - Đúng hàng không? (So với đơn hàng)
   - Đủ số lượng không?
   - Đúng vị trí không? (So với vị trí đã chỉ định ở Bước 3)
6. **Bước 6:** Xác nhận → Trừ kho & Tách Record (Split Logic):
   
   **Logic Tách Record (Record Splitting):**
   - *Tình huống:* Trong Bin có 10 cái, cần lấy 4 cái.
   - *Hành động:* Không thể chỉ sửa số lượng 10 thành 6, vì cần track 4 cái kia đi đâu.
   - *Xử lý:*
     1. Tìm `Inventory_Item` gốc (Qty: 10, Status: AVAILABLE)
     2. Update `Inventory_Item` gốc: `Quantity` = 10 - 4 = 6
     3. Tạo `Inventory_Item` mới (Split):
        - `product_id`, `batch_no`, `expiry` giống gốc
        - `quantity` = 4
        - `status` = `SHIPPED` (hoặc `RESERVED` nếu chờ gom)
        - Link với `order_id`
   - *Kết quả:* Kho còn 6 cái AVAILABLE. 4 cái kia đã chuyển trạng thái.

#### 1.2.4. Counting (Kiểm kê)

- **Logic Blind Count (Kiểm kê mù):**
  - App **KHÔNG** hiển thị số lượng tồn kho hiện tại
  - Bắt buộc công nhân quét/đếm thực tế
  - Tránh tâm lý "thấy số 10 thì điền đại 10"

- **Quy trình:**
  1. Chọn khu vực cần kiểm kê (Kệ, Tầng, hoặc toàn kho)
  2. Quét từng Bin Location
  3. Quét/Đếm hàng thực tế trong Bin
  4. Nhập số lượng đếm được
  5. Hệ thống so sánh với sổ sách

- **Xử lý lệch:**
  - Nếu lệch: Flag lại để Chủ duyệt
  - Chủ có thể: Duyệt điều chỉnh hoặc Yêu cầu kiểm kê lại

### 1.3. Quy trình Sản xuất Tinh gọn (Lightweight Manufacturing)

#### 1.3.1. BOM (Bill of Materials - Định mức)

- **Định nghĩa:** 1 Thành phẩm = N Nguyên liệu
- **Ví dụ:**
  - 1 Ghế = 4 Chân + 1 Mặt + 20 Ốc vít
  - 1 Áo = 1 Vải + 5 Cúc + 1 Chỉ

- **Cấu trúc:**
  - Thành phẩm (Finished Good)
  - Nguyên liệu (Raw Material)
  - Số lượng mỗi nguyên liệu

- **Logic kiểm tra:**
  - Khi tạo lệnh sản xuất → Hệ thống kiểm tra tồn kho nguyên liệu
  - Nếu thiếu → Cảnh báo ngay, không cho tạo lệnh

#### 1.3.2. Production Execution (Thực thi Sản xuất)

- **Quy trình 2 bước (Pick & Make):**
  1. **Pick Materials (Lấy nguyên liệu):**
     - Hệ thống tạo nhiệm vụ Outbound cho Nguyên liệu dựa trên BOM.
     - Công nhân thực hiện Outbound (như mục 1.2.3) để lấy nguyên liệu.
     - Nguyên liệu chuyển trạng thái: `AVAILABLE` → `SHIPPED` (hoặc `CONSUMED`).
  2. **Finish Product (Hoàn thành):**
     - Khi sản xuất xong, công nhân quét nhập kho Thành phẩm.
     - Hệ thống tạo `Inventory_Item` mới cho Thành phẩm (Status `AVAILABLE`).
     - Link Thành phẩm với Lệnh sản xuất để truy xuất nguồn gốc (Traceability).

- **Lưu ý:** Không "tự động trừ kho" từ AVAILABLE một cách kỳ diệu. Phải thông qua bước Pick (Outbound) để đảm bảo hàng thực sự được lấy ra từ đúng vị trí.

#### 1.3.3. Defect Recording (Ghi nhận lỗi)

- Cho phép nhập số lượng hỏng ngay tại bước đóng gói
- Trừ kho phế phẩm riêng (không tính vào thành phẩm)
- Báo cáo tỷ lệ lỗi cho chủ xưởng

### 1.4. Logic Quyền hạn (Permission & Authorization)

#### Công nhân (Worker):
- Quét và nhập số lượng
- Xem đơn hàng được giao
- **Không được** sửa số lượng tồn kho (Adjust)
- **Không được** xóa lịch sử

#### Chủ xưởng (Owner/Manager):
- Tất cả quyền của Công nhân
- Sửa số lượng tồn kho (Adjust)
- Duyệt điều chỉnh kiểm kê
- Phê duyệt đơn xuất hàng quan trọng
- Xem báo cáo tài chính
- Quản lý nhân viên

---

## 2. DATAFLOW (Luồng dữ liệu)

### 2.1. Mô hình Offline-First (Local Database)

#### 2.1.1. Tại thiết bị (Edge Device)

- **Database cục bộ:** WatermelonDB
- **Khi công nhân quét:**
  - Dữ liệu được ghi **NGAY LẬP TỨC** vào Local DB (Độ trễ = 0ms)
  - UI cập nhật ngay (Optimistic UI)
  - Không cần chờ server phản hồi

- **Action Queue (Hàng đợi hành động):**
  - Mỗi thao tác tạo 1 Action record: `[INSERT_SCAN, UPDATE_QTY, MOVE_ITEM]`
  - Action có trạng thái: `PENDING`, `SYNCING`, `SYNCED`, `FAILED`
  - Queue được lưu trong Local DB

#### 2.1.2. Cơ chế Đồng bộ (Sync Engine)

- **Khi có mạng:**
  - App tự động đẩy Action Queue lên Server theo lô (Batch)
  - Batch size: 10-50 actions/lần (tùy kích thước)
  - Sync chạy background, không block UI

- **Khi mất mạng:**
  - Actions vẫn được lưu trong Local DB
  - Hiển thị badge "Đang offline" trên màn hình
  - Tự động retry khi có mạng lại

- **Conflict Resolution (Xử lý xung đột):**
  - **Chiến lược phân biệt theo loại dữ liệu:**
  
  **1. Last Write Wins (LWW) - Cho dữ liệu vị trí và metadata:**
    - Áp dụng cho: `location_id`, `status`, `batch_no`, `expiry_date`, `production_date`
    - Logic: Timestamp từ server là source of truth, giá trị cuối cùng ghi đè
    - Ví dụ: 2 công nhân cùng cập nhật vị trí hàng → Giá trị có timestamp mới nhất thắng
    - Implementation: So sánh `server_timestamp`, chọn giá trị có timestamp lớn hơn
  
  **2. CRDT (Conflict-free Replicated Data Types) - Cho số lượng tồn kho:**
    - Áp dụng cho: `quantity` trong Inventory_Items
    - Logic: Cộng dồn số lượng thay vì ghi đè để tránh mất dữ liệu
    - Ví dụ: 
      - Worker A quét +10 (timestamp: T1)
      - Worker B quét +5 (timestamp: T2, cùng inventory_item_id)
      - Kết quả: `quantity = quantity_old + 10 + 5` (không mất dữ liệu)
    - Implementation: 
      - Khi sync, server kiểm tra nếu có conflict trên cùng `inventory_item_id`
      - Thay vì ghi đè, server cộng dồn: `new_quantity = old_quantity + delta_A + delta_B`
      - Lưu log conflict để audit trail
  
  **3. First Come First Served - Cho trường hợp đặc biệt:**
    - Áp dụng cho: Outbound khi số lượng tồn kho = 0 (hết hàng)
    - Scenario: 2 người cùng xuất 1 món hàng cuối cùng
    - Request đến trước → Thành công
    - Request đến sau → Server trả về lỗi: "Hàng vừa bị người khác lấy, vui lòng kiểm tra lại"
    - App hiển thị lỗi đỏ, yêu cầu hoàn tác (Rollback)

### 2.2. Luồng xử lý giao dịch (Transaction Flow)

```
Trigger: Công nhân quét QR/Barcode
    ↓
Step 1: Validation (Local)
    • Kiểm tra định dạng mã (QR/Barcode hợp lệ không?)
    • Kiểm tra số lượng (> 0 không?)
    • Kiểm tra quyền hạn (Công nhân có quyền thao tác này không?)
    ↓
Step 2: Execution (Local)
    • Cập nhật Local DB ngay lập tức
    • Cập nhật UI (Optimistic UI) → Công nhân thấy kết quả ngay
    • Tạo Action record trong Queue
    ↓
Step 3: Sync (Background)
    • Gửi Action lên Server (nếu có mạng)
    • Hoặc lưu vào Queue chờ sync sau
    ↓
Step 4: Confirmation
    • Server xử lý và trả về kết quả
    • Nếu Success:
      - Đánh dấu Action là SYNCED
      - Xóa Action khỏi Queue (sau 7 ngày để backup)
    • Nếu Fail:
      - Đánh dấu Action là FAILED
      - Hiển thị lỗi đỏ trên UI
      - Yêu cầu hoàn tác (Rollback Local DB)
      - Cho phép retry thủ công
```

### 2.3. Luồng Nhập kho (Inbound Flow)

```
Bước 1: Quét mã hàng
    ↓
Bước 2: Hệ thống kiểm tra loại hàng
    - Có trong hệ thống chưa?
    - Có mapping chưa?
    ↓
Bước 3: Nếu hàng lạ → Mapping
    - Quét Barcode → Gán vào SKU có sẵn
    - Hoặc tạo SKU mới
    ↓
Bước 4: Nhập thông tin
    - Số lượng
    - Hạn sử dụng
    - Trạng thái (Đạt/Lỗi)
    ↓
Bước 5: Quét mã kệ (Bin Location)
    ↓
Bước 6: Cập nhật tồn kho vị trí
    - Ghi nhận: SKU, Số lượng, Vị trí, Người nhập, Thời gian
```

### 2.4. Luồng Đối soát (Matching Flow)

```
Bước 1: Lệnh xuất hàng được tạo
    - Danh sách: Hàng X, Số lượng Y, Vị trí Z
    ↓
Bước 2: Công nhân quét hàng thực tế
    - Quét mã hàng
    - Quét mã vị trí
    ↓
Bước 3: Hệ thống so sánh (Match/Mismatch)
    - Đúng hàng? → ✓
    - Đúng số lượng? → ✓
    - Đúng vị trí? → ✓
    ↓
Bước 4: Kết quả
    - Match: Màn hình xanh, cho phép tiếp tục
    - Mismatch: Màn hình đỏ, báo lỗi cụ thể, không cho tiếp tục
```

### 2.5. Luồng Scan-to-Pack (Đóng gói có kiểm soát)

```
Bước 1: Quét mã QR thùng (Container)
    - Tạo "phiên đóng gói" mới
    ↓
Bước 2: Quét từng món hàng bỏ vào thùng
    - App hiển thị checklist: "Đã bỏ 1/10"
    - Nếu quét nhầm loại → App kêu "Tít tít" màu đỏ, không cho tiếp tục
    ↓
Bước 3: Kiểm tra đủ số lượng
    - Đủ 10/10 → Cho phép hoàn thành
    - Thiếu → Không cho hoàn thành, bắt buộc quét tiếp
    ↓
Bước 4: Hoàn thành
    - Kích hoạt mã QR "Cha" dán lên thùng
    - Hoặc in tem vận chuyển
    - Ghi nhận: Thùng chứa những gì, Ai đóng, Lúc nào
```

---

## 3. CÁC YẾU TỐ KỸ THUẬT (Technical Specifications)

### 3.1. Hardware Compatibility

- **Điện thoại tối thiểu:**
  - Android: Version 8.0 (Oreo) trở lên
  - iOS: Version 12.0 trở lên
  - Camera: Tối thiểu 8MP, hỗ trợ autofocus
  - RAM: Tối thiểu 2GB
  - Dung lượng trống: Tối thiểu 500MB

- **Thiết bị ngoại vi (Optional):**
  - Máy quét barcode chuyên dụng (Bluetooth)
  - Cân điện tử (USB/Bluetooth)
  - Máy in tem (Bluetooth/Wi-Fi)

### 3.2. Performance Requirements

- **Tốc độ quét:**
  - Nhận diện mã: < 500ms
  - Phản hồi UI: < 100ms
  - Ghi Local DB: < 50ms

- **Sync:**
  - Batch sync: Mỗi 30 giây (khi có mạng)
  - Hoặc sync ngay khi có mạng (nếu đang offline)

- **Offline capacity:**
  - Lưu được tối thiểu 10,000 actions offline
  - Tương đương ~2 tuần làm việc (nếu mất mạng hoàn toàn)

---

## 3.3. Mobile App Architecture (Kiến trúc ứng dụng mobile)

Phần này giúp Frontend Engineer hiểu cách tổ chức code và implement offline-first architecture.

### 3.3.1. Technology Stack

| Thành phần | Lựa chọn | Tại sao? |
|------------|----------|----------|
| **Mobile App** | Expo | Tận dụng thư viện Camera/Scanner tốt nhất cho WMS. |
| **Local DB** | WatermelonDB | Quan trọng nhất để đạt mục tiêu "10,000+ actions offline" mà không lag UI. |
| **Logic Core** | Rust | Viết các hàm Functional xử lý tồn kho, validation để dùng chung mọi nơi. |
| **Desktop App** | Tauri (Rust) | App quản lý cho chủ xưởng mượt, nhẹ, bảo mật cao. |
| **Sync Protocol** | WebSockets/NATS | Đảm bảo tính real-time khi có mạng lại. |

**Chi tiết bổ sung:**

**Mobile App (Expo):**
- **State Management:** Redux Toolkit / Zustand
- **Navigation:** React Navigation
- **Camera/Scanner:** react-native-vision-camera + react-native-vision-camera-code-scanner
- **Network:** Axios / Fetch với retry logic
- **Storage:** AsyncStorage / SecureStore

**Local Database (WatermelonDB):**
- Reactive database với lazy loading
- Tối ưu cho offline-first architecture
- Hỗ trợ sync conflict resolution
- Performance cao với 10,000+ records

**Logic Core (Rust):**
- Shared business logic giữa Mobile và Desktop
- Compile thành native modules (FFI) cho Expo
- Type-safe và performance cao
- Validation rules, inventory calculations, FEFO/FIFO algorithms

**Desktop App (Tauri):**
- Frontend: React/Vue (web technologies)
- Backend: Rust core (shared với mobile)
- Bundle size nhỏ, bảo mật cao
- Native performance

**Sync Protocol:**
- **WebSockets:** Real-time bidirectional communication
- **NATS:** Message queue cho sync batching và reliability
- Fallback: REST API cho initial sync và compatibility

### 3.3.2. App Structure (Cấu trúc thư mục)

**Khuyến nghị cấu trúc:**
```
src/
├── screens/          # Màn hình (Dashboard, Scanner, Inbound, Outbound, etc.)
├── components/        # Components tái sử dụng (Button, ScannerView, FeedbackOverlay)
├── navigation/       # Navigation config
├── services/          # Business logic & API calls
│   ├── api/          # API client, endpoints
│   ├── sync/         # Sync engine
│   └── scanner/      # Scanner service
├── store/            # State management (Redux/Zustand)
│   ├── slices/       # Redux slices hoặc Zustand stores
│   └── actions/      # Action creators
├── database/         # Local database schema & queries
│   ├── models/       # Data models
│   └── migrations/   # Database migrations
├── utils/            # Utilities (validation, formatting, etc.)
└── constants/        # Constants (colors, sizes, API endpoints)
```

### 3.3.3. State Management Strategy

**Offline-First State Flow:**
1. **User Action** → Update Local State ngay lập tức (Optimistic UI)
2. **Write to Local DB** → Commit transaction
3. **Add to Action Queue** → Mark as PENDING
4. **Background Sync** → Process queue, update status (SYNCING → SYNCED/FAILED)

**State Structure:**
- **UI State:** Component-level state (form inputs, loading states)
- **App State:** Global state (user info, network status, sync status)
- **Database State:** Source of truth (products, locations, inventory, orders)
- **Queue State:** Actions pending sync

### 3.3.4. Navigation Flow

**Main Navigation Stack:**
```
Auth Stack
  └─ Login Screen
      ↓
Main Stack
  ├─ Dashboard (Home)
  │   ├─ Inbound Flow
  │   │   ├─ Scanner Screen
  │   │   ├─ Mapping Screen (if needed)
  │   │   ├─ Quantity Input Screen
  │   │   └─ Location Scanner Screen
  │   ├─ Outbound Flow
  │   │   ├─ Order List Screen
  │   │   ├─ Guided Workflow Screen
  │   │   └─ Scanner Screen
  │   └─ Counting Flow
  │       ├─ Area Selection Screen
  │       └─ Scanner Screen
  └─ Settings
      └─ Sync Status Screen
```

**Navigation Principles:**
- Mỗi flow là một stack riêng (không quay lại Dashboard giữa chừng)
- Back button chỉ quay lại bước trước trong cùng flow
- Không có deep linking phức tạp (Phase 1)

### 3.3.5. Offline-First Implementation

**Local Database Schema:**
- Mirror server schema (Products, Locations, Inventory_Items, Orders, Barcode_Mappings)
- Thêm bảng `action_queue` để track pending sync
- Thêm bảng `sync_metadata` để track last_sync_timestamp

**Sync Engine Flow:**
```
1. App Start → Check network status
2. If online → Pull latest data from server (initial sync hoặc incremental)
3. Background Sync Loop (mỗi 30 giây):
   - Query action_queue WHERE status = 'PENDING'
   - Batch actions (10-50 actions/batch)
   - POST /api/v1/sync/push với batch
   - Update action status: SYNCED hoặc FAILED
4. On Network Reconnect → Trigger sync immediately
```

**Conflict Resolution:**
- **Chiến lược phân biệt theo loại dữ liệu:**
  - **Location & Metadata (LWW):** Server timestamp là source of truth, giá trị cuối cùng ghi đè
  - **Inventory Quantity (CRDT):** Cộng dồn số lượng, không ghi đè
  - **Outbound khi hết hàng (FCFS):** Request đến trước thắng
- Nếu conflict → Server xử lý theo chiến lược tương ứng, update local DB
- Hiển thị notification cho user nếu có conflict nghiêm trọng (hết hàng, không thể cộng dồn)

### 3.3.6. Scanner Integration

**Camera Scanner Component:**
- Full-screen camera view
- Continuous scan mode (không cần bấm nút)
- Auto-focus khi phát hiện mã
- Flash toggle
- Zoom control
- **Multi-modal feedback khi scan:**
  - **Haptic feedback:** Rung nhẹ 100ms (thành công), rung mạnh 500ms 2 lần (lỗi)
  - **Sound feedback:** "Tít" 200ms tần số cao (thành công), "Bíp bíp" 800ms tần số thấp (lỗi)
  - **Visual feedback:** Màn hình xanh 500ms (thành công), đỏ 1000ms (lỗi)

**Barcode Recognition:**
- Support: QR Code, EAN-13, Code 128, UPC-A
- Validate format trước khi xử lý
- Debounce scan (tránh duplicate scan trong 500ms)

### 3.3.7. Error Handling & User Feedback

**Feedback Mechanisms:**
- **Visual:** Màn hình xanh (success) / đỏ (error) với animation
- **Haptic:** Rung nhẹ 100ms (success) / rung mạnh 500ms 2 lần (error) - Quan trọng khi không nhìn màn hình
- **Audio:** 
  - Thành công: "Tít" (tần số cao 2000-3000 Hz, 200ms, 1 tiếng)
  - Lỗi: "Bíp bíp" (tần số thấp 400-600 Hz, 800ms, 2 tiếng) - Quan trọng trong môi trường ồn
- **Toast/Notification:** Hiển thị message ngắn gọn

**Lưu ý:** Trong môi trường kho ồn, âm thanh và haptic feedback quan trọng hơn màu sắc màn hình. Công nhân có thể không nhìn màn hình khi đang cầm hàng, nhưng vẫn cảm nhận được rung và nghe được âm thanh.

**Error Recovery:**
- Network errors → Retry tự động
- Validation errors → Hiển thị message rõ ràng, không crash
- Database errors → Log error, yêu cầu re-login nếu corruption

### 3.3.8. Performance Optimization

**Key Optimizations:**
- **Lazy Loading:** Chỉ load data khi cần (không load tất cả products/locations lúc đầu)
- **Pagination:** Load orders/products theo batch
- **Image Caching:** Cache QR codes nếu có
- **Debouncing:** Debounce scan events, search inputs
- **Memoization:** Memoize expensive calculations
- **Background Processing:** Sync chạy background, không block UI

**Memory Management:**
- Clear old action_queue entries sau 7 ngày
- Limit local database size (nếu quá lớn → archive old data)
- Unmount camera khi không dùng (tiết kiệm battery)

---

## 4. DATABASE SCHEMA (Cấu trúc dữ liệu cốt lõi)

Phần này giúp Engineer thiết kế Database không bị lỗi logic khi scale.

**Lưu ý:** Không cần viết SQL, nhưng cần mô tả mối quan hệ (Relationships) rõ ràng.

### 4.1. Entity Relationship Diagram (ERD) mô tả bằng lời

#### 4.1.1. Users (Người dùng)

- **id:** UUID (Primary Key)
- **username:** String (Tên đăng nhập, có Index UNIQUE)
- **password_hash:** String (Mã hóa password, không lưu plain text)
- **name:** String (Tên hiển thị)
- **role:** Enum (`WORKER`, `MANAGER`, `OWNER`)
- **device_id:** String (ID thiết bị đăng ký lần đầu, nullable, có Index)
- **is_active:** Boolean (Cho phép vô hiệu hóa tài khoản)
- **created_at:** Timestamp
- **updated_at:** Timestamp
- **last_login_at:** Timestamp (nullable)

**Mối quan hệ:**
- 1 User có N Inventory_Items (One-to-Many, qua created_by)
- 1 User có N Orders (One-to-Many, qua created_by)

**Logic:**
- Username phải unique trong hệ thống
- Device_id được ghi nhận khi login lần đầu từ thiết bị mới
- Nếu device_id lạ → Yêu cầu xác thực 2 bước (OTP) - xem Security section
- Role xác định quyền hạn: WORKER chỉ quét, MANAGER/OWNER có thể tạo đơn, điều chỉnh tồn kho

**Index:**
- `username` (UNIQUE) để login nhanh
- `device_id` để check device binding

#### 4.1.2. Products (Sản phẩm)

- **id:** UUID (Primary Key)
- **name:** String (Tên sản phẩm)
- **unit:** String (Đơn vị tính: Cái, Kg, Thùng...)
- **master_sku:** String (Mã nội bộ duy nhất, có Index)
- **status:** Enum (`ACTIVE`, `INACTIVE`, `DISCONTINUED`)
- **created_at:** Timestamp
- **updated_at:** Timestamp

**Mối quan hệ:**
- 1 Product có N Barcode_Mappings (One-to-Many)
- 1 Product có N Inventory_Items (One-to-Many)

#### 4.1.3. Barcode_Mappings (Bảng quan trọng nhất)

- **barcode:** String (Mã vạch quét được - Primary Key, có Index UNIQUE)
- **product_id:** UUID (Foreign Key → Products.id)
- **source:** Enum (`INTERNAL`, `VENDOR_A`, `VENDOR_B`, `CUSTOM`)
- **created_at:** Timestamp
- **is_active:** Boolean (Có thể vô hiệu hóa mapping cũ)

**Logic:**
- 1 Product có N Barcodes (One-to-Many)
- Khi quét Barcode bất kỳ → Query ra Product (Tốc độ < 100ms nhờ Index)
- Mapping được tạo 1 lần, dùng mãi (trừ khi bị vô hiệu hóa)

#### 4.1.4. Locations (Vị trí)

- **id:** UUID (Primary Key)
- **code:** String (Ví dụ: `WH01-R01-S02-B03`, có Index)
- **name:** String (Tên hiển thị)
- **parent_id:** UUID (Foreign Key → Locations.id, nullable - để phân cấp)
- **location_type:** Enum (`WAREHOUSE`, `RACK`, `SHELF`, `BIN`)
- **bin_type:** Enum (`MIXED`, `FIXED`) - Chỉ áp dụng khi location_type = BIN
- **fixed_product_id:** UUID (Foreign Key → Products.id, nullable) - Chỉ dùng khi bin_type = FIXED
- **max_capacity:** Decimal (Sức chứa tối đa, nullable) - Đơn vị: số lượng items
- **is_active:** Boolean
- **created_at:** Timestamp

**Logic:**
- `bin_type`: 
  - `MIXED`: Bin có thể chứa nhiều SKU khác nhau
  - `FIXED`: Bin chỉ chứa 1 SKU cụ thể (cho hàng giá trị cao)
- `fixed_product_id`: Chỉ set khi `bin_type = FIXED`, xác định SKU duy nhất được phép cất vào bin này
- `max_capacity`: Giới hạn số lượng items có thể chứa (tính tổng quantity của tất cả Inventory_Items trong bin)
- Validation khi Put-away:
  - Nếu bin_type = FIXED: Chỉ cho phép cất hàng có product_id = fixed_product_id
  - Nếu bin_type = MIXED: Cho phép cất bất kỳ SKU nào
  - Kiểm tra max_capacity: Tổng quantity hiện tại + quantity mới <= max_capacity (nếu max_capacity được set)

**Mối quan hệ:**
- 1 Location có thể có 1 Location cha (Self-referencing, Many-to-One)
- 1 Location có N Inventory_Items (One-to-Many)

#### 4.1.5. Inventory_Items (Tồn kho thực tế)

- **id:** UUID (Primary Key)
- **product_id:** UUID (Foreign Key → Products.id, có Index)
- **location_id:** UUID (Foreign Key → Locations.id, có Index)
- **quantity:** Decimal (Số lượng tồn kho)
- **batch_no:** String (Số lô - Optional, có Index)
- **expiry_date:** Date (Hạn sử dụng - Optional, có Index)
- **production_date:** Date (Ngày sản xuất - Optional)
- **status:** Enum (`STAGING`, `AVAILABLE`, `RESERVED`, `SHIPPED`, `DEFECT`)
- **created_at:** Timestamp
- **updated_at:** Timestamp
- **created_by:** UUID (Foreign Key → Users.id)

**Logic:**
- Đây là bảng ghi nhận "Cái gì đang ở đâu với số lượng bao nhiêu"
- Một Product có thể có nhiều Inventory_Items ở nhiều Location khác nhau
- Một Location có thể chứa nhiều Inventory_Items của nhiều Product khác nhau (Mixed Bin)

**Composite Index:** `(product_id, location_id, batch_no)` để tối ưu query tồn kho theo vị trí

#### 4.1.6. Containers (Vật chứa)

- **id:** UUID (Primary Key)
- **lpn:** String (License Plate Number - Mã QR Container, có Index UNIQUE)
- **container_type:** Enum (`BOX`, `PALLET`, `BAG`, `CART`)
- **parent_container_id:** UUID (Foreign Key → Containers.id, nullable - để Nesting)
- **status:** Enum (`EMPTY`, `IN_USE`, `IN_TRANSIT`, `ARCHIVED`)
- **created_at:** Timestamp
- **updated_at:** Timestamp

**Mối quan hệ:**
- 1 Container có thể có 1 Container cha (Self-referencing, Many-to-One - Nested)
- 1 Container có N Container_Items (One-to-Many)

#### 4.1.7. Container_Items (Hàng trong Container)

- **id:** UUID (Primary Key)
- **container_id:** UUID (Foreign Key → Containers.id, có Index)
- **inventory_item_id:** UUID (Foreign Key → Inventory_Items.id)
- **quantity:** Decimal
- **created_at:** Timestamp

**Logic:**
- Quét mã Container (QR cha) = Quét toàn bộ hàng bên trong (QR con)
- Khi di chuyển Container, tất cả hàng bên trong tự động di chuyển theo

#### 4.1.8. Action_Queue (Hàng đợi đồng bộ)

- **id:** UUID (Primary Key)
- **device_id:** String (ID thiết bị, có Index)
- **action_type:** Enum (`INBOUND_SCAN`, `OUTBOUND_SCAN`, `PUT_AWAY`, `MAPPING_NEW`, `ADJUST_QTY`, `MOVE_ITEM`)
- **status:** Enum (`PENDING`, `SYNCING`, `SYNCED`, `FAILED`)
- **payload:** JSON (Dữ liệu chi tiết của action)
- **timestamp:** Timestamp (Thời gian tạo tại device)
- **server_timestamp:** Timestamp (Thời gian nhận tại server, nullable)
- **retry_count:** Integer (Số lần retry)
- **error_message:** String (Thông báo lỗi nếu FAILED)
- **created_at:** Timestamp
- **synced_at:** Timestamp (nullable)

**Mối quan hệ:**
- N Action_Queue thuộc về 1 Device (Many-to-One, không có bảng Device riêng)

**Index:**
- `(device_id, status, created_at)` để query actions cần sync
- `(status, created_at)` để cleanup actions đã sync > 7 ngày

#### 4.1.9. Orders (Đơn hàng)

- **id:** UUID (Primary Key)
- **order_number:** String (Số đơn hàng, có Index UNIQUE)
- **order_type:** Enum (`SALES_ORDER`, `PRODUCTION_ORDER`, `TRANSFER_ORDER`)
- **status:** Enum (`PENDING`, `IN_PROGRESS`, `COMPLETED`, `CANCELLED`)
- **created_by:** UUID (Foreign Key → Users.id)
- **created_at:** Timestamp
- **updated_at:** Timestamp

**Mối quan hệ:**
- 1 Order có N Order_Items (One-to-Many)

#### 4.1.10. Order_Items (Chi tiết đơn hàng)

- **id:** UUID (Primary Key)
- **order_id:** UUID (Foreign Key → Orders.id, có Index)
- **product_id:** UUID (Foreign Key → Products.id)
- **quantity_ordered:** Decimal (Số lượng yêu cầu)
- **quantity_picked:** Decimal (Số lượng đã lấy, default = 0)
- **location_hint:** String (Vị trí gợi ý - Optional)
- **priority:** Enum (`HIGH`, `MEDIUM`, `LOW`)
- **created_at:** Timestamp

**Index:**
- `(order_id, product_id)` để query nhanh items của 1 order

#### 4.1.11. BOM (Bill of Materials - Định mức) - Phase 1 (Optional)

**Lưu ý:** BOM table này chỉ cần thiết nếu Phase 1 MVP có tính năng sản xuất (Production Order). Nếu Phase 1 chỉ tập trung vào nhập/xuất kho đơn giản, có thể bỏ qua và thêm vào Phase 2.

- **id:** UUID (Primary Key)
- **finished_product_id:** UUID (Foreign Key → Products.id, có Index) - Thành phẩm
- **raw_material_id:** UUID (Foreign Key → Products.id, có Index) - Nguyên liệu
- **quantity_required:** Decimal (Số lượng nguyên liệu cần để sản xuất 1 đơn vị thành phẩm)
- **unit:** String (Đơn vị tính của quantity_required)
- **is_active:** Boolean
- **created_at:** Timestamp
- **updated_at:** Timestamp

**Mối quan hệ:**
- 1 Finished Product có N BOM entries (One-to-Many) - mỗi entry là 1 nguyên liệu
- BOM có 2 foreign keys đến Products (finished_product và raw_material)

**Logic:**
- Khi tạo Production Order:
  1. Query BOM theo `finished_product_id`
  2. Tính tổng nguyên liệu cần: `quantity_required * quantity_to_produce`
  3. Kiểm tra tồn kho nguyên liệu
  4. Nếu thiếu → Báo lỗi, không cho tạo đơn
  
- Khi hoàn thành Production Order:
  1. Query BOM theo `finished_product_id`
  2. Trừ kho nguyên liệu: `quantity_required * quantity_produced`
  3. Cộng kho thành phẩm: `quantity_produced`

**Ví dụ:**
- Thành phẩm: "Ghế" (id: product-001)
- BOM:
  - 1 Ghế = 4 Chân (raw_material_id: product-002, quantity_required: 4)
  - 1 Ghế = 1 Mặt (raw_material_id: product-003, quantity_required: 1)
  - 1 Ghế = 20 Ốc vít (raw_material_id: product-004, quantity_required: 20)

**Index:**
- `(finished_product_id, raw_material_id)` để query BOM nhanh
- `finished_product_id` để query tất cả nguyên liệu của 1 thành phẩm

### 4.2. Indexes (Tối ưu tốc độ)

**Yêu cầu đánh Index cho các cột quan trọng:**

1. **Barcode_Mappings.barcode:**
   - Index UNIQUE
   - Mục đích: Tốc độ quét < 100ms
   - Loại: B-Tree Index

2. **Products.master_sku:**
   - Index UNIQUE
   - Mục đích: Tìm kiếm SKU nhanh

3. **Locations.code:**
   - Index
   - Mục đích: Query vị trí nhanh khi quét QR Bin Location

4. **Inventory_Items (product_id, location_id, batch_no):**
   - Composite Index
   - Mục đích: Query tồn kho theo vị trí và lô nhanh

5. **Inventory_Items.expiry_date:**
   - Index
   - Mục đích: Query hàng sắp hết hạn (FEFO logic)

6. **Action_Queue (device_id, status, created_at):**
   - Composite Index
   - Mục đích: Query actions cần sync nhanh

7. **Containers.lpn:**
   - Index UNIQUE
   - Mục đích: Query container khi quét QR nhanh

8. **Users.username:**
   - Index UNIQUE
   - Mục đích: Login nhanh

9. **Users.device_id:**
   - Index
   - Mục đích: Check device binding

10. **Locations (location_type, bin_type):**
    - Composite Index (nếu cần query nhiều)
    - Mục đích: Query bins theo type nhanh

11. **BOM (finished_product_id, raw_material_id):** (Nếu Phase 1 có manufacturing)
    - Composite Index
    - Mục đích: Query BOM nhanh khi tạo Production Order

**Lưu ý:**
- Không đánh Index quá nhiều (trade-off giữa tốc độ query và tốc độ insert)
- Ưu tiên Index cho các cột thường xuyên được query trong quy trình quét

---

## 5. API CONTRACT (Giao thức đồng bộ)

Phần này giúp Mobile Dev và Backend Dev không cãi nhau xem gửi dữ liệu kiểu gì.

### 5.1. Authentication Endpoints (Đăng nhập & Xác thực)

#### 5.1.1. Login

**Endpoint:** `POST /api/v1/auth/login`

**Request Body:**
```json
{
  "username": "worker_001",
  "password": "password123",
  "device_id": "android_abc123xyz"
}
```

**Response Success (200):**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 2592000,
    "user": {
      "id": "user-uuid",
      "username": "worker_001",
      "name": "Nguyễn Văn A",
      "role": "WORKER"
    }
  }
}
```

**Response Error (401):**
```json
{
  "success": false,
  "error_code": "INVALID_CREDENTIALS",
  "error_message": "Tên đăng nhập hoặc mật khẩu không đúng"
}
```

**Response Error (403) - New Device:**
```json
{
  "success": false,
  "error_code": "NEW_DEVICE_DETECTED",
  "error_message": "Thiết bị mới được phát hiện. Vui lòng xác thực 2 bước",
  "requires_otp": true,
  "otp_sent_to": "phone_number_or_email"
}
```

#### 5.1.2. Refresh Token

**Endpoint:** `POST /api/v1/auth/refresh`

**Request Body:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response Success (200):**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 2592000
  }
}
```

**Response Error (401):**
```json
{
  "success": false,
  "error_code": "REFRESH_TOKEN_INVALID",
  "error_message": "Refresh token không hợp lệ hoặc đã hết hạn"
}
```

#### 5.1.3. Logout

**Endpoint:** `POST /api/v1/auth/logout`

**Request Headers:**
```
Authorization: Bearer {JWT_TOKEN}
```

**Response Success (200):**
```json
{
  "success": true,
  "message": "Đăng xuất thành công"
}
```

### 5.2. Data Pull Endpoints (Lấy dữ liệu từ Server)

#### 5.2.1. Get Products

**Endpoint:** `GET /api/v1/products`

**Query Parameters:**
- `status`: Enum (`ACTIVE`, `INACTIVE`, `DISCONTINUED`) - Filter theo trạng thái
- `search`: String - Tìm kiếm theo tên hoặc SKU
- `limit`: Integer (default: 100, max: 1000)
- `offset`: Integer (default: 0)

**Request Headers:**
```
Authorization: Bearer {JWT_TOKEN}
```

**Response Success (200):**
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "product-uuid",
        "master_sku": "SKU_AO_THUN",
        "name": "Áo thun cổ tròn",
        "unit": "Cái",
        "status": "ACTIVE",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z"
      }
    ],
    "total": 150,
    "limit": 100,
    "offset": 0
  }
}
```

#### 5.2.2. Get Locations

**Endpoint:** `GET /api/v1/locations`

**Query Parameters:**
- `location_type`: Enum (`WAREHOUSE`, `RACK`, `SHELF`, `BIN`) - Filter theo loại
- `parent_id`: UUID - Lấy locations con của parent
- `is_active`: Boolean (default: true)

**Request Headers:**
```
Authorization: Bearer {JWT_TOKEN}
```

**Response Success (200):**
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "location-uuid",
        "code": "WH01-R01-S02-B03",
        "name": "Kệ A1, Tầng 2, Ô 3",
        "parent_id": "parent-location-uuid",
        "location_type": "BIN",
        "bin_type": "MIXED",
        "fixed_product_id": null,
        "max_capacity": null,
        "is_active": true,
        "created_at": "2024-01-01T00:00:00Z"
      }
    ],
    "total": 50
  }
}
```

#### 5.2.3. Get Orders

**Endpoint:** `GET /api/v1/orders`

**Query Parameters:**
- `status`: Enum (`PENDING`, `IN_PROGRESS`, `COMPLETED`, `CANCELLED`) - Filter theo trạng thái
- `order_type`: Enum (`SALES_ORDER`, `PRODUCTION_ORDER`, `TRANSFER_ORDER`)
- `limit`: Integer (default: 50, max: 100)
- `offset`: Integer (default: 0)

**Request Headers:**
```
Authorization: Bearer {JWT_TOKEN}
```

**Response Success (200):**
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "order-uuid",
        "order_number": "SO-2024-001",
        "order_type": "SALES_ORDER",
        "status": "PENDING",
        "created_by": "user-uuid",
        "created_at": "2024-01-01T00:00:00Z",
        "updated_at": "2024-01-01T00:00:00Z",
        "items": [
          {
            "id": "order-item-uuid",
            "product_id": "product-uuid",
            "product_name": "Áo thun cổ tròn",
            "quantity_ordered": 10,
            "quantity_picked": 0,
            "location_hint": "A-01-02",
            "priority": "HIGH"
          }
        ]
      }
    ],
    "total": 25
  }
}
```

#### 5.2.4. Get Barcode Mappings

**Endpoint:** `GET /api/v1/barcode-mappings`

**Query Parameters:**
- `product_id`: UUID - Lấy mappings của 1 product
- `barcode`: String - Tìm mapping theo barcode
- `is_active`: Boolean (default: true)

**Request Headers:**
```
Authorization: Bearer {JWT_TOKEN}
```

**Response Success (200):**
```json
{
  "success": true,
  "data": {
    "items": [
      {
        "barcode": "8934567890123",
        "product_id": "product-uuid",
        "product_name": "Áo thun cổ tròn",
        "source": "VENDOR_A",
        "is_active": true,
        "created_at": "2024-01-01T00:00:00Z"
      }
    ],
    "total": 500
  }
}
```

### 5.3. Cấu trúc gói tin Sync (Sync Payload Structure)

Quy định rõ định dạng JSON khi App gửi dữ liệu lên Server (Batch Sync):

**Endpoint:** `POST /api/v1/sync/push`

**Request Headers:**
```
Authorization: Bearer {JWT_TOKEN}
Content-Type: application/json
X-Device-ID: {device_id}
X-App-Version: {version}
```

**Request Body:**
```json
{
  "device_id": "android_12345",
  "sync_session_id": "uuid_gen_tai_app",
  "actions": [
    {
      "action_type": "INBOUND_SCAN",
      "timestamp": 1703325000,
      "payload": {
        "barcode": "893456789",
        "quantity": 10,
        "location_code": "A-01-02",
        "batch_no": "LOT-2024-001",
        "expiry_date": "2025-12-31",
        "is_new_mapping": false
      }
    }
  ]
}
```

**Response Success (200):**
```json
{
  "success": true,
  "sync_session_id": "uuid_gen_tai_app",
  "processed": 4,
  "failed": 0,
  "results": [...]
}
```

### 5.2. Xử lý lỗi (Error Codes)

#### HTTP Status Codes:
- **200 OK:** Sync thành công hoàn toàn (Xanh lá)
- **206 Partial Content:** Một số action thành công, một số lỗi (Vàng)
- **400 Bad Request:** Dữ liệu gửi lên không hợp lệ (Đỏ)
- **401 Unauthorized:** Token hết hạn hoặc không hợp lệ (Đăng xuất, yêu cầu login lại)
- **403 Forbidden:** Không có quyền thực hiện action này (Đỏ)
- **426 Upgrade Required:** Bắt buộc cập nhật App mới cho sync (Đỏ, hiển thị link download)
- **429 Too Many Requests:** Quá nhiều request, cần chờ (Vàng, retry sau)
- **500 Internal Server Error:** Lỗi server (Đỏ, cho phép retry)

#### Business Error Codes:
- **PRODUCT_NOT_FOUND:** Sản phẩm không tồn tại
- **BARCODE_NOT_MAPPED:** Mã vạch chưa được mapping
- **INSUFFICIENT_STOCK:** Không đủ hàng trong kho
- **WRONG_LOCATION:** Quét sai vị trí
- **WRONG_PRODUCT:** Quét sai sản phẩm (so với đơn hàng)
- **DUPLICATE_SCAN:** Đã quét rồi (tránh double scan)
- **ORDER_NOT_FOUND:** Đơn hàng không tồn tại
- **ORDER_ALREADY_COMPLETED:** Đơn hàng đã hoàn thành
- **LOCATION_NOT_FOUND:** Vị trí không tồn tại
- **CONFLICT_DETECTED:** Xung đột dữ liệu (2 người cùng thao tác)

### 5.4. Pull Sync (Đồng bộ ngược từ Server - Batch)

**Endpoint:** `GET /api/v1/sync/pull`

**Query Parameters:**
- `last_sync_timestamp`: Timestamp (ISO 8601) lần sync cuối (để chỉ lấy dữ liệu mới)
- `device_id`: ID thiết bị

**Request Headers:**
```
Authorization: Bearer {JWT_TOKEN}
```

**Response Success (200):**
```json
{
  "success": true,
  "data": {
    "last_sync_timestamp": "2024-01-15T10:30:00Z",
    "updates": {
      "products": [
        {
          "id": "product-uuid",
          "master_sku": "SKU_AO_THUN",
          "name": "Áo thun cổ tròn",
          "unit": "Cái",
          "status": "ACTIVE",
          "updated_at": "2024-01-15T10:00:00Z"
        }
      ],
      "locations": [
        {
          "id": "location-uuid",
          "code": "WH01-R01-S02-B03",
          "name": "Kệ A1, Tầng 2, Ô 3",
          "parent_id": "parent-uuid",
          "location_type": "BIN",
          "bin_type": "MIXED",
          "updated_at": "2024-01-15T09:00:00Z"
        }
      ],
      "orders": [
        {
          "id": "order-uuid",
          "order_number": "SO-2024-001",
          "order_type": "SALES_ORDER",
          "status": "PENDING",
          "items": [
            {
              "id": "order-item-uuid",
              "product_id": "product-uuid",
              "quantity_ordered": 10,
              "quantity_picked": 0
            }
          ],
          "updated_at": "2024-01-15T08:00:00Z"
        }
      ],
      "barcode_mappings": [
        {
          "barcode": "8934567890123",
          "product_id": "product-uuid",
          "source": "VENDOR_A",
          "is_active": true,
          "updated_at": "2024-01-15T07:00:00Z"
        }
      ]
    }
  }
}
```

**Logic:**
- Chỉ trả về dữ liệu có `updated_at` > `last_sync_timestamp`
- Nếu `last_sync_timestamp` = null → Trả về tất cả dữ liệu (initial sync)
- App nên gọi endpoint này khi:
  - Đăng nhập lần đầu (initial sync)
  - App khởi động (để lấy updates mới)
  - Sau khi push sync thành công (để lấy dữ liệu đã thay đổi bởi server)

---

## 6. SECURITY & EDGE CASES (Bảo mật & Trường hợp ngoại lệ)

Phần này để tránh app bị crash hoặc bị hack.

### 6.1. Authentication (Đăng nhập)

#### 6.1.1. Cơ chế JWT (JSON Web Token)

- **Token Structure:**
  - Header: Algorithm (HS256)
  - Payload: `{ user_id, role, device_id, exp, iat }`
  - Signature: HMAC SHA256

- **Token Lifecycle:**
  - Access Token: Hiệu lực 30 ngày
  - Refresh Token: Hiệu lực 90 ngày
  - Tự động refresh khi còn 7 ngày trước khi hết hạn

#### 6.1.2. Offline Auth

- **Khi có mạng:**
  - Verify token signature với Server
  - Refresh token tự động khi gần hết hạn

- **Khi mất mạng:**
  - App vẫn cho phép đăng nhập nếu Token còn hạn (Verify signature cục bộ hoặc tin tưởng cache)
  - Lưu token trong Secure Storage (Keychain/Keystore)
  - Khi có mạng lại, tự động refresh token

#### 6.1.3. Device Binding

- Mỗi device có `device_id` duy nhất (UUID)
- Server ghi nhận device_id khi login lần đầu
- Nếu phát hiện device_id lạ → Yêu cầu xác thực 2 bước (OTP)

### 6.2. Edge Cases (Các tình huống "hiểm")

#### 6.2.1. Camera Permission Denied
- App **KHÔNG được crash**
- Hiển thị màn hình hướng dẫn vào Settings
- Disable nút "NHẬP", "XUẤT", "KIỂM KHO" cho đến khi có quyền

#### 6.2.2. Storage Full
- App phải cảnh báo ngay
- Chặn quét tiếp (disable camera) cho đến khi có mạng hoặc giải phóng bộ nhớ
- Hiển thị số lượng actions đang chờ sync

#### 6.2.3. Time Travel (Gian lận thời gian)
- Server sẽ ghi nhận `server_time` khi nhận gói tin
- Chỉ dùng `device_time` để tham khảo thứ tự (không tin tưởng hoàn toàn)
- Nếu phát hiện `device_time` chênh lệch > 5 phút so với `server_time`:
  - Cảnh báo: "Thời gian thiết bị không chính xác"
  - Vẫn cho phép sync, nhưng dùng `server_time` làm thời gian chính thức

#### 6.2.4. Network Interruption During Sync
- App phải retry tự động khi có mạng lại
- Nếu sync partial: Chỉ retry các action FAILED
- Hiển thị progress: "Đang đồng bộ 15/20 actions..."

#### 6.2.5. Duplicate Scan Prevention
- App kiểm tra trong Local DB: "Mã này đã quét trong 5 giây gần đây chưa?"
- Nếu có → Hiển thị cảnh báo: "Bạn vừa quét mã này. Bỏ qua hay tiếp tục?"

#### 6.2.6. Battery Low
- Hiển thị cảnh báo: "Pin yếu. Vui lòng sạc pin"
- Vẫn cho phép quét, nhưng ưu tiên sync ngay (không đợi batch)
- Tắt các tính năng không cần thiết (animations, haptic feedback)

#### 6.2.7. App Force Kill
- Actions vẫn được lưu trong Local DB (đã commit trước khi kill)
- Khi mở app lại, tự động kiểm tra và sync actions còn pending

#### 6.2.8. Database Corruption
- App phải detect corruption khi khởi động
- Nếu detect corruption:
  - Hiển thị: "Dữ liệu cục bộ bị lỗi. Vui lòng đăng nhập lại"
  - Xóa Local DB cũ
  - Yêu cầu user login lại
  - Pull toàn bộ dữ liệu từ Server về

#### 6.2.9. Invalid Barcode Format
- App phải validate format trước khi xử lý
- Nếu không hợp lệ → Hiển thị: "Mã không hợp lệ. Vui lòng quét lại"
- Không tạo action trong queue
- Không crash app

#### 6.2.10. Server Down
- App vẫn hoạt động bình thường (offline mode)
- Hiển thị badge: "Đang offline - Server không khả dụng"
- Lưu tất cả actions vào Local DB
- Tự động retry mỗi 5 phút
- Khi server online lại, sync tất cả actions đã tích lũy

---

## LƯU Ý QUAN TRỌNG CHO ENGINEER

✅ **NÊN tập trung vào:**
- Tốc độ quét (Performance requirements)
- Tính chính xác của dữ liệu (Data integrity)
- Quy trình cụ thể (Step-by-step flows)
- Technical specifications
- Offline-first architecture
- Conflict resolution

❌ **KHÔNG NÊN:**
- Bỏ qua edge cases
- Thiết kế database không có Index
- Không xử lý lỗi đầy đủ
- Bỏ qua security

