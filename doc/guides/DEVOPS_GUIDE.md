# Hướng dẫn DevOps & Triển khai

## 🏛️ Chiến lược Triển khai "Hybrid" (Enterprise MVP)

Dự án sử dụng chiến lược **Hybrid** để tối ưu hóa hiệu năng và chi phí cho giai đoạn MVP (Enterprise MVP).

| Thành phần | Công nghệ Deploy | Lý do lựa chọn |
| :--- | :--- | :--- |
| **Frontend Web** | **Vercel** | Tốc độ CDN toàn cầu, SSL miễn phí, dễ dàng preview PR. |
| **Backend API** | **Railway** | Chạy Docker container ổn định (không phải serverless), dễ scale DB. |
| **Mobile App** | **Expo EAS** | Cập nhật qua không khí (OTA Update) cho người dùng ngay lập tức. |

### 🔄 Dev/Prod Parity (Lưu ý quan trọng)

Để tối ưu tốc độ phát triển trên Windows, chúng ta sử dụng **Native Build** (chạy trực tiếp `.exe`) thay vì Docker. Tuy nhiên:

- **Frontend/Mobile**: Không ảnh hưởng (Native JS).
- **Backend**:
  - **Dev**: Chạy native bằng `scripts/dev-api.ps1` để có feedback loop nhanh nhất.
  - **Prod**: Chạy trong Docker container (Linux Base).

> [!IMPORTANT]
> **Source of Truth**: Dockerfile (`apps/api/Dockerfile`) luôn là chuẩn duy nhất cho môi trường Production. Mọi logic phụ thuộc vào OS (ví dụ: đường dẫn file, system calls) cần được kiểm tra kỹ trên CI/CD Docker.

---

## 📋 Danh sách kiểm tra trước khi Deploy

- [ ] **Secrets**: Đã config `VERCEL_TOKEN`, `RAILWAY_TOKEN`, `EXPO_TOKEN` trong GitHub Secrets.
- [ ] **Production Env**: Kiểm tra `.env` trên Railway và Vercel Dashboard.
- [ ] **Database**: Đã chạy Migration trên Railway (`sea-orm-cli migrate up`).
- [ ] **Health Check**: API Production phải trả về 200 tại `/health`.

---

## ☁️ Hướng dẫn Cấu hình Chi tiết

### 1. Frontend (Vercel)

- **Project Settings**:
  - Framework Preset: `Vite`
  - Build Command: `pnpm build:web`
  - Output Directory: `apps/client-web/dist`
  - Root Directory: `apps/client-web`
- **Environment Variables**:
  - `VITE_API_URL`: URL của Backend Railway (VD: `https://api-production.up.railway.app`)

### 2. Backend (Railway)

- **Service**: Tạo mới "Empty Service" -> Link GitHub Repo.
- **Dockerfile**: Railway sẽ tự động dùng `docker/Dockerfile` hoặc `Dockerfile` gốc.
- **Variables**: Thêm `DATABASE_URL`, `JWT_SECRET`, `PORT` (3000).
- **Domains**: Gắn Domain (VD: `api.lean-wms.com`).

### 3. Mobile (Expo)

- **EAS Update**: Dùng để push hot-fix thẳng tới máy user.
- **EAS Build**: Dùng để tạo file `.apk` / `.ipa` nộp lên store.

---

## 🚀 Migration Path (Lộ trình Nâng cấp)

Khi dự án có lượng dữ liệu lớn hoặc cần bảo mật khắt khe hơn:

1. **Giai đoạn 1 (Hiện tại)**: Vercel + Railway (Nhanh, Rẻ, Ít tốn công DevOps).
2. **Giai đoạn 2 (Scale)**: Chuyển Backend/DB sang **VPS Riêng** (DigitalOcean/AWS).
   - Dùng **Docker Compose** để chạy lại stack y hệt như trên Railway.
   - Frontend vẫn giữ ở Vercel (hoặc chuyển về VPS nếu cần VPN nội bộ).

---

## 🛡️ Best Practices & Audit (Kiểm soát chất lượng)

Để đảm bảo hệ thống vận hành ổn định trong môi trường Production, chúng ta tuân thủ các tiêu chuẩn sau:

### 1. Code Quality & Formatting

- **Rust**: Luôn chạy `cargo lint` và `cargo fmt` trước khi push. Sử dụng `thiserror` cho việc xử lý lỗi tập trung.
- **TypeScript**: Tuân thủ strict mode, không sử dụng `any`. Sử dụng Path Aliases (`@/*`) để code sạch hơn.

### 2. Security

- API phải luôn có JWT validation.
- Database URL và các Secrets quan trọng phải được quản lý qua Environment Variables (không hardcode).
- Định kỳ quét lỗi bảo mật bằng `cargo audit`.

### 3. Monitoring & Maintenance

- Kiểm tra log định kỳ (`RUST_LOG=info`).
- Đảm bảo các chỉ số Health Check tại `/health` luôn trả về 200.
