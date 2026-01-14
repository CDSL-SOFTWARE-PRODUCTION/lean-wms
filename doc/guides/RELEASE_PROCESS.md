# Quy trình Release (Release Process)

Tài liệu này hướng dẫn cách deploy và release các phiên bản của Lean WMS.

## 1. Tổng quan CI/CD

Chúng tôi sử dụng GitHub Actions để tự động hóa quy trình deploy.

| Thành phần | Môi trường Dev (`develop`) | Môi trường Prod (`main`) | Platform |
| :--- | :--- | :--- | :--- |
| **Web** | Tự động deploy | Tự động deploy | Vercel |
| **API** | Tự động deploy | Tự động deploy | Railway |
| **Mobile (JS)** | Tự động OTA Update (`preview`) | Tự động OTA Update (`production`) | Expo EAS Update |
| **Mobile (Native)** | **Thủ công** | **Thủ công** (hoặc Tag) | Expo EAS Build |

---

## 2. Web & API (Tự động)

Chỉ cần push code:

- Push vào `develop` -> Deploy lên Dev Environment.
- Merge vào `main` -> Deploy lên Production.

> [!NOTE]
> Backend API sẽ luôn được build bằng Docker trên Railway, bất kể bạn dev bằng cách nào (Native hay Docker). Điều này đảm bảo tính nhất quán của môi trường Production.

## 3. Mobile App (Quy trình đặc biệt)

Mobile App có 2 loại update:

### A. OTA Update (Cập nhật Code JS/Logic)

- **Khi nào?**: Sửa bug nhỏ, thay đổi UI, logic React Native.
- **Cách làm**: Push code như bình thường.
- **Kết quả**: Người dùng mở app sẽ tự tải bản mới về (không cần lên Store).

### B. Native Build (Cập nhật Binary .apk / .ipa)

- **Khi nào?**:
  - Cài thêm thư viện mới có dính tới native (ví dụ: Camera, Bluetooth, Map).
  - Thay đổi icon, splash screen, cấu hình `app.json`.
  - Release phiên bản lớn lên App Store / Google Play.
- **Cách làm**:
    1. Vào tab **Actions** trên GitHub.
    2. Chọn workflow **CD**.
    3. Bấm **Run workflow** (nút màu xám bên phải).
    4. Chọn nhánh (`main` hoặc `develop`).
    5. Bấm **Run workflow**.
- **Kết quả**:
  - GitHub sẽ chạy lệnh `eas build`.
  - Sau 15-30p, bạn sẽ nhận được link tải file `.apk` (Android) hoặc `.ipa` (iOS) trong dashboard Expo hoặc email.

## 4. Release Production (Gắn Tag)

Để đánh dấu một phiên bản release chính thức:

```bash
git tag v1.0.0
git push origin v1.0.0
```

Hành động này cũng sẽ kích hoạt **Native Build** cho Mobile.
