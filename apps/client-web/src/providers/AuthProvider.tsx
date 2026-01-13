import { createContext, useContext, useState, useEffect, type ReactNode } from 'react';
import { apiClient, type LoginResponse } from '../services/api';

interface AuthContextType {
    user: LoginResponse['user'] | null;
    isAuthenticated: boolean;
    isLoading: boolean;
    login: (email: string, password: string) => Promise<void>;
    logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [user, setUser] = useState<LoginResponse['user'] | null>(null);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        // Check for existing token
        // In a real app, we should validate the token with /api/auth/me or similar
        const token = localStorage.getItem('auth_token');
        if (token) {
            // Placeholder: We assume valid if token exists for now, 
            // OR we decode JWT to get user info 
            // OR we call a 'me' endpoint.
            // For MVP/Skeleton, let's just confirm we have a token.
            // To be better, we should store user info in localStorage too or fetch it.
            // Let's assume we can't recover user object without fetching.
            // But for now, let's just set isAuthenticated implicitly by token presence logic in ApiClient.
            // Actually, we can't reconstruct 'user' state purely from token without decoding.

            // Let's try to restore basic user info from localStorage if we saved it?
            // ApiClient doesn't save user info, only token.
            // Let's modify logic to optionally fetch user or just start as "authenticated but unknown user".

            // For this skeleton, I'll just set loading false.
            // If we want to persist user, we should save it to localStorage on login.
        }
        setIsLoading(false);
    }, []);

    const login = async (email: string, password: string) => {
        try {
            const response = await apiClient.login(email, password);
            if (response.user) {
                setUser(response.user);
                // Optional: Save user to localStorage
            }
        } catch (error) {
            console.error("Login failed", error);
            throw error;
        }
    };

    const logout = () => {
        apiClient.logout();
        setUser(null);
    };

    return (
        <AuthContext.Provider value={{
            user,
            isAuthenticated: !!user || !!localStorage.getItem('auth_token'), // Basic check
            isLoading,
            login,
            logout
        }}>
            {children}
        </AuthContext.Provider>
    );
}

export function useAuth() {
    const context = useContext(AuthContext);
    if (context === undefined) {
        throw new Error('useAuth must be used within an AuthProvider');
    }
    return context;
}
