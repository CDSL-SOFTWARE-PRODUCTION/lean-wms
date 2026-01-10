import { createClient, FetchTransport } from '@rspc/client';
import { Platform } from 'react-native';
import type { Procedures } from '@lean-wms/core';

// Cấu hình địa chỉ IP Backend cho Mobile
// - Android Emulator: dùng 10.0.2.2 để trỏ về localhost của máy host
// - iOS Simulator: dùng localhost
// - Thiết bị thật: Cần thay bằng IP LAN của máy tính (ví dụ 192.168.1.x)
// Ưu tiên dùng biến môi trường (Dev & Prod đều dùng được)
const API_URL =
  process.env.EXPO_PUBLIC_API_URL ||
  Platform.select({
    android: 'http://10.0.2.2:3000/rspc', // Fallback cho Emulator
    ios: 'http://localhost:3000/rspc',
    default: 'http://localhost:3000/rspc',
  });

export const rspc = createClient<Procedures>({
  transport: new FetchTransport(API_URL),
});
