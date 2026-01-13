# Hướng dẫn DevOps & Triển khai

## 📋 Danh sách kiểm tra triển khai (Deployment Checklist)

- [ ] **Biến môi trường**: Kiểm tra `.env.production` đã được thiết lập.
- [ ] **Cơ sở dữ liệu**: Chạy migrations `sea-orm-cli migrate up`.
- [ ] **Bảo mật (Secrets)**: Đảm bảo JWT_SECRET đủ mạnh và được xoay vòng.
- [ ] **SSL**: Certbot/SSL certificates đã hoạt động.
- [ ] **Kiểm tra hoạt động (Health Check)**: Xác nhận endpoint `/health` trả về 200.

## 🐳 Docker

### Build (Xây dựng)

```bash
docker compose build
```

### Run (Chạy)

```bash
docker compose up -d
```

## ☁️ Hạ tầng (Infrastructure)

- **Cơ sở dữ liệu**: Managed PostgreSQL (AWS RDS / Supabase / DigitalOcean).
- **Backend**: Linux VPS / Container Service (ECS/DigitalOcean App).
- **Frontend**: Static hosting (Vercel / Netlify / S3).
