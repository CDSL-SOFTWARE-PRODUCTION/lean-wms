import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { useDispatch, useSelector } from 'react-redux';
import { logout } from '../../store/slices/authSlice';
import { AppDispatch, RootState } from '../../store';

export default function HomeScreen() {
    const dispatch = useDispatch<AppDispatch>();
    const user = useSelector((state: RootState) => state.auth.user);

    return (
        <View style={styles.container}>
            <Text style={styles.title}>Welcome, {user?.name || 'User'}!</Text>
            <Text style={styles.subtitle}>Role: {user?.role}</Text>

            <View style={styles.content}>
                <Text>Inventory Summary will appear here.</Text>
            </View>

            <TouchableOpacity
                style={styles.logoutButton}
                onPress={() => dispatch(logout())}
            >
                <Text style={styles.logoutText}>Logout</Text>
            </TouchableOpacity>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        padding: 24,
        backgroundColor: '#fff',
        justifyContent: 'center',
        alignItems: 'center',
    },
    title: {
        fontSize: 24,
        fontWeight: 'bold',
        marginBottom: 8,
    },
    subtitle: {
        fontSize: 16,
        color: '#666',
        marginBottom: 32,
    },
    content: {
        flex: 1,
        justifyContent: 'center',
    },
    logoutButton: {
        paddingVertical: 12,
        paddingHorizontal: 32,
        backgroundColor: '#ff3b30',
        borderRadius: 8,
        marginTop: 'auto',
    },
    logoutText: {
        color: '#fff',
        fontSize: 16,
        fontWeight: '600',
    },
});
