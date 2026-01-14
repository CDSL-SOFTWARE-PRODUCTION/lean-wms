# Kế hoạch Dự án

## 🗓️ Mốc thời gian (Timeline)

| Giai đoạn | Cột mốc | Mô tả | Ngày dự kiến | Tài liệu Kỹ thuật (Blueprints) |
|---|---|---|---|---|
| 1 | Nền tảng | Thiết lập Monorepo, Logic cốt lõi, Xác thực (Auth) | Q1 2026 (Hoàn thành) | `doc/architecture/BLUEPRINT_BACKEND.md` (Ready: Auth & Products CRUD) |
| 2 | MVP Web | Quản lý kho cơ bản, Nhập/Xuất kho (Web) | Q1 2026 (Đang làm) | `doc/architecture/BLUEPRINT_FRONTEND.md` |
| 3 | Mobile | Ứng dụng cho công nhân (Chưa có code) | Q1 2026 (Đang làm) | `doc/requirements/PRD.md` (Section 5.3) |
| 4 | Tối ưu hóa | Báo cáo, Dashboard, Tối ưu hóa hệ thống | Q2 2026 | `doc/architecture/BLUEPRINT_OVERVIEW.md` |

## 📚 Tham chiếu Kiến trúc (Architecture References)

Để build đúng chuẩn, vui lòng tham khảo:

- **Backend Logic & Data Flow:** [BLUEPRINT_BACKEND.md](../architecture/BLUEPRINT_BACKEND.md)
- **Frontend UI/UX:** [BLUEPRINT_FRONTEND.md](../architecture/BLUEPRINT_FRONTEND.md)
- **Tổng quan:** [BLUEPRINT_OVERVIEW.md](../architecture/BLUEPRINT_OVERVIEW.md)

## ⚠️ Rủi ro

- **Độ khó công nghệ**: Rust và React 19 có thể làm chậm giai đoạn phát triển ban đầu.
- **Phần cứng di động**: Hiệu năng camera điện thoại so với máy quét chuyên dụng.

## 🎯 Mục tiêu (Điều lệ dự án)

- Xây dựng hệ thống WMS "Tinh gọn" tập trung vào nhu cầu SME.
- Backend hiệu năng cao (Rust).
- Hỗ trợ đa nền tảng (Web + Mobile).
