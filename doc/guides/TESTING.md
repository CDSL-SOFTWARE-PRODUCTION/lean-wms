# Chiến lược Kiểm thử (Testing Strategy)

## 🧪 Các mức độ kiểm thử

### 1. Unit Tests (Kiểm thử đơn vị)

- **Phạm vi**: Các hàm riêng lẻ (Logic FEFO, Validate dữ liệu).
- **Công cụ**: `cargo test` (Rust), `jest` (TS).
- **Mục tiêu phủ (Coverage)**: >80% cho Logic cốt lõi.

### 2. Integration Tests (Kiểm thử tích hợp)

- **Phạm vi**: API endpoints + Tương tác Database.
- **Công cụ**: `cargo test` với Database test.

### 3. E2E Tests (Kiểm thử đầu cuối)

- **Phạm vi**: Luồng người dùng đầy đủ (Đăng nhập -> Tạo đơn hàng).
- **Công cụ**: Playwright / Cypress.

## 🤖 CI Pipeline

- Chạy kiểm tra cú pháp (`cargo clippy`, `eslint`).
- Chạy unit tests.
- Kiểm tra build.
