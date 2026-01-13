# DevOps & Deployment Guide

## 📋 Deployment Checklist

- [ ] **Environment Variables**: Check `.env.production` is set.
- [ ] **Database**: Run migrations `sea-orm-cli migrate up`.
- [ ] **Secrets**: Ensure JWT_SECRET is strong and rotated.
- [ ] **SSL**: Certbot/SSL certificates active.
- [ ] **Health Check**: Verify `/health` endpoint returns 200.

## 🐳 Docker

### Build

```bash
docker compose build
```

### Run

```bash
docker compose up -d
```

## ☁️ Infrastructure

- **Database**: Managed PostgreSQL (AWS RDS / Supabase / DigitalOcean).
- **Backend**: Linux VPS / Container Service (ECS/DigitalOcean App).
- **Frontend**: Static hosting (Vercel / Netlify / S3).
