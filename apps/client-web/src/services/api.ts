// API service layer for client-web
// Provides typed API client for making requests to the backend

import axios, { type AxiosInstance, AxiosError } from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

export interface ApiError {
  message: string;
  status?: number;
  code?: string;
}

export class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string = API_BASE_URL) {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
      },
      timeout: 10000,
    });

    // Request interceptor for adding auth token
    this.client.interceptors.request.use(
      (config) => {
        const token = this.getAuthToken();
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor for error handling
    this.client.interceptors.response.use(
      (response) => response,
      (error: AxiosError) => {
        if (error.response?.status === 401) {
          // Handle unauthorized - clear token and redirect to login
          this.clearAuthToken();
          window.location.href = '/login';
        }
        return Promise.reject(this.normalizeError(error));
      }
    );
  }

  private getAuthToken(): string | null {
    return localStorage.getItem('auth_token');
  }

  private clearAuthToken(): void {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('refresh_token');
  }

  private normalizeError(error: AxiosError): ApiError {
    if (error.response) {
      const errorData = error.response.data as { message?: string } | undefined;
      return {
        message: errorData?.message || error.message,
        status: error.response.status,
        code: error.code,
      };
    }
    return {
      message: error.message || 'Network error',
      code: error.code,
    };
  }

  // Auth endpoints
  async login(email: string, password: string) {
    const response = await this.client.post('/api/auth/login', {
      email,
      password,
    });
    if (response.data.token) {
      localStorage.setItem('auth_token', response.data.token);
      if (response.data.refreshToken) {
        localStorage.setItem('refresh_token', response.data.refreshToken);
      }
    }
    return response.data;
  }

  async register(email: string, password: string, name?: string) {
    const response = await this.client.post('/api/auth/register', {
      email,
      password,
      name,
    });
    return response.data;
  }

  async refreshToken() {
    const refreshToken = localStorage.getItem('refresh_token');
    if (!refreshToken) {
      throw new Error('No refresh token available');
    }
    const response = await this.client.post('/api/auth/refresh', {
      refreshToken,
    });
    if (response.data.token) {
      localStorage.setItem('auth_token', response.data.token);
    }
    return response.data;
  }

  async logout() {
    this.clearAuthToken();
    // Optionally call logout endpoint
    try {
      await this.client.post('/api/auth/logout');
    } catch {
      // Ignore errors on logout
    }
  }

  // Products endpoints
  async getProducts(params?: {
    page?: number;
    limit?: number;
    search?: string;
  }) {
    const response = await this.client.get('/api/products', { params });
    return response.data;
  }

  async getProduct(id: string) {
    const response = await this.client.get(`/api/products/${id}`);
    return response.data;
  }

  async createProduct(data: Partial<Product>) {
    const response = await this.client.post('/api/products', data);
    return response.data;
  }

  async updateProduct(id: string, data: Partial<Product>) {
    const response = await this.client.put(`/api/products/${id}`, data);
    return response.data;
  }

  async deleteProduct(id: string) {
    const response = await this.client.delete(`/api/products/${id}`);
    return response.data;
  }

  // Orders endpoints
  async getOrders(params?: { page?: number; limit?: number; status?: string }) {
    const response = await this.client.get('/api/orders', { params });
    return response.data;
  }

  async getOrder(id: string) {
    const response = await this.client.get(`/api/orders/${id}`);
    return response.data;
  }

  async createOrder(data: Partial<Order>) {
    const response = await this.client.post('/api/orders', data);
    return response.data;
  }

  async updateOrder(id: string, data: Partial<Order>) {
    const response = await this.client.put(`/api/orders/${id}`, data);
    return response.data;
  }

  async deleteOrder(id: string) {
    const response = await this.client.delete(`/api/orders/${id}`);
    return response.data;
  }

  // Sync endpoints
  async syncPull(lastSyncTime?: string) {
    const response = await this.client.post('/api/sync/pull', {
      lastSyncTime,
    });
    return response.data;
  }

  async syncPush(data: { changes: unknown[]; lastSyncTime?: string }) {
    const response = await this.client.post('/api/sync/push', data);
    return response.data;
  }

  async getSyncStatus() {
    const response = await this.client.get('/api/sync/status');
    return response.data;
  }
}

// Export singleton instance
export const apiClient = new ApiClient();

// Export types for API responses
export interface LoginResponse {
  token: string;
  refreshToken?: string;
  user: {
    id: string;
    email: string;
    name?: string;
  };
}

export interface Product {
  id: string;
  name: string;
  sku?: string;
  description?: string;
  price?: number;
  quantity?: number;
  createdAt: string;
  updatedAt: string;
}

export interface Order {
  id: string;
  orderNumber: string;
  status: string;
  items: OrderItem[];
  total: number;
  createdAt: string;
  updatedAt: string;
}

export interface OrderItem {
  id: string;
  productId: string;
  quantity: number;
  price: number;
}
