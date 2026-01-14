# Hướng dẫn Setup GitHub Projects (Project Management)

Để thay thế việc quản lý bằng file Markdown thủ công, chúng ta sẽ sử dụng **GitHub Projects** (được tích hợp sẵn, miễn phí cho public/private repo).

## 1. Tạo Project Mới

1. Vào tab **Projects** trên GitHub Repo của bạn.
2. Chọn **"New project"**.
3. Chọn Template:
    - **Team planning** (Nếu muốn đầy đủ tính năng).
    - Hoặc **Feature preview** (bắt đầu từ trắng).
4. Đặt tên: "Lean WMS Development".

## 2. Cấu hình View (Góc nhìn)

Chúng ta cần 3 views chính tương ứng với yêu cầu của bạn:

### A. Kanban Board (Quản lý tiến độ hàng ngày)

- **Kiểu View**: Board.
- **Cột (Columns)**:
  - 🆕 Todo (Chưa làm)
  - 🚧 In Progress (Đang làm)
  - 👀 Review (Code Review / Pull Request)
  - ✅ Done (Hoàn thành)
- **Automation**: Vào `Workflows`, bật "Auto-add to project" và "Item added to project" để khi tạo Issue mới nó tự chui vào cột Todo.

### B. Roadmap (Lộ trình dài hạn)

- **Kiểu View**: Roadmap (Gantt Chart).
- **Trục dọc (Rows)**: Group by `Milestone` hoặc `Sprint`.
- **Trục ngang (Time)**: Sử dụng field `Start Date` và `Target Date`.
- **Mục đích**: Nhìn tổng quan Phase 1, Phase 2 khi nào xong.

### C. Bug Tracker (Theo dõi lỗi)

- **Kiểu View**: Table.
- **Filter**: `Label: bug`.
- **Cột hiển thị**: Priority (P0, P1, P2), Status, Assignee.

## 3. Quy trình làm việc (Workflow)

Thay vì tích file `task.md`, quy trình mới sẽ là:

1. **Feature Release**:
    - Vào Tab **Issues** -> Tạo Milestone mới (VD: `v1.0 - Mobile MVP`).
    - Tạo Issue cho từng feature nhỏ (VD: "Mobile Login Screen", "Backend Product API").
    - Assign Milestone `v1.0` cho các issue này.

2. **Dev Hàng ngày**:
    - Mở Project Board.
    - Kéo thẻ từ **Todo** -> **In Progress**.
    - Khi commit code, thêm mã số issue (VD: `fix: login logic #12`) -> GitHub tự link commit vào thẻ.

3. **Release**:
    - Khi tất cả thẻ trong Milestone chuyển sang **Done**.
    - Tạo Release Note từ các Issue đã đóng.

## 4. Lợi ích so với File MD

- **Trực quan**: Kéo thả, có biểu đồ.
- **Tự động**: Link với Pull Request và Commit.
- **Lịch sử**: Biết ai làm gì, vào lúc nào.
