# ⛔ DEPRECATED / LƯU TRỮ

Thư mục này chứa các tài liệu thiết kế ban đầu (Blueprints) dùng để khởi động dự án.
**Hiện tại các nội dung này đã được migrate sang hệ thống tài liệu chính thức.**

Vui lòng **KHÔNG CHỈNH SỬA** các file trong thư mục này. Hãy tham chiếu các tài liệu mới nhất dưới đây:

## 🗺️ Bản đồ chuyển đổi (Migration Map)

| Tài liệu cũ (Archive) | Tài liệu mới (Source of Truth) | Ghi chú |
| :--- | :--- | :--- |
| `BLUEPRINT_BACKEND.md` | **[ADR-0002](../architecture/adr/0002-backend-architecture-blueprint.md)** | Đổi từ Rust Core sang Supabase BaaS |
| `BLUEPRINT_FRONTEND.md` | **[ADR-0003](../architecture/adr/0003-frontend-architecture-blueprint.md)** | Giữ nguyên principle, update tech stack |
| `BLUEPRINT_OVERVIEW.md` | **[ARCHITECTURE.md](../architecture/ARCHITECTURE.md)** | Tổng quan hệ thống |
| Offline Strategy | **[ADR-0004](../architecture/adr/0004-offline-sync-strategy.md)** | Chiến lược Sync & Queue |
| Security | **[ADR-0005](../architecture/adr/0005-row-level-security.md)** | RLS & Policy |
| `ROADMAP.md` | *GitHub Project* | Quản lý task trên GitHub |

## ⚠️ Lưu ý

Nếu bạn cần tìm các thông số chi tiết về UI (Mã màu, Tần số âm thanh, Haptic pattern) chưa có trong ADR, hãy tham khảo `BLUEPRINT_FRONTEND.md` nhưng **đừng implement logic cũ** từ đó.
