import { apiClient } from './api';
import * as SecureStore from 'expo-secure-store';

export interface User {
    id: string;
    username: string;
    name: string;
    role: string;
}

export interface LoginResponse {
    token: string;
    refreshToken: string;
    user: User;
}

export const AuthService = {
    async login(email: string, password: string): Promise<LoginResponse> {
        const response = await apiClient.post<LoginResponse>('/auth/login', {
            username: email, // Backend expects 'username', but UI might say 'email'
            password,
        });

        // Save tokens securely
        await SecureStore.setItemAsync('token', response.data.token);
        await SecureStore.setItemAsync('refreshToken', response.data.refreshToken);

        return response.data;
    },

    async register(email: string, password: string, name: string): Promise<LoginResponse> {
        const response = await apiClient.post<LoginResponse>('/auth/register', {
            username: email,
            password,
            name,
        });

        await SecureStore.setItemAsync('token', response.data.token);
        await SecureStore.setItemAsync('refreshToken', response.data.refreshToken);

        return response.data;
    },

    async logout() {
        await SecureStore.deleteItemAsync('token');
        await SecureStore.deleteItemAsync('refreshToken');
    },

    async getToken() {
        return await SecureStore.getItemAsync('token');
    }
};
