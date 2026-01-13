# Hướng dẫn Kiểm thử (Testing Guide)

## 🧪 Các loại kiểm thử

1. **Unit Tests**: Kiểm tra logic nhỏ nhất (Functions, Component).
2. **Integration Tests**: Kiểm tra sự kết hợp giữa các modules (API + Database).
3. **E2E Tests**: Kiểm tra luồng người dùng thực tế.

## ☁️ Kiểm thử Triển khai (Deployment Testing)

Với chiến lược Hybrid, bạn có thể kiểm thử trực tiếp trên các môi trường:

### 1. Frontend Preview (Vercel)

- Mỗi Pull Request mở ra sẽ tự động có một **Preview URL** từ Vercel.
- Team Lead review giao diện và chức năng trực tiếp trên URL này trước khi Merge.

### 2. Mobile Update (Expo Go)

- Cài app **Expo Go** trên điện thoại.
- Khi code push vào `develop` hoặc `staging`, mở app Expo Go và quét QR code của dự án (hoặc check kênh update `preview`) để thấy thay đổi ngay lập tức mà không cần cài lại file APK.

### 3. API Staging (Railway)

- Endpoint: `https://api-staging.lean-wms.com` (Ví dụ).
- Dùng Postman hoặc Swagger UI để test các API mới trước khi lên Production.

## 🛠️ Lệnh chạy test local

```bash
# Test Core logic
pnpm test:core

# Test Web Component
pnpm test:web

# Test Rust API
pnpm test:api
```
