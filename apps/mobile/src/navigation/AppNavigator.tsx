import React, { useEffect } from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import { useSelector, useDispatch } from 'react-redux';
import { RootState, AppDispatch } from '../store';
import { AuthService } from '../services/auth';
import { restoreSession } from '../store/slices/authSlice';

import AuthNavigator from './AuthNavigator';
import HomeScreen from '../screens/home/HomeScreen';
import { ActivityIndicator, View } from 'react-native';

const Stack = createStackNavigator();

export default function AppNavigator() {
    const { isAuthenticated } = useSelector((state: RootState) => state.auth);
    const dispatch = useDispatch<AppDispatch>();
    const [isRestoringToken, setRestoringToken] = React.useState(true);

    useEffect(() => {
        const bootstrapAsync = async () => {
            try {
                const token = await AuthService.getToken();
                if (token) {
                    dispatch(restoreSession({ token }));
                }
            } catch (e) {
                // failed to restore
            }
            setRestoringToken(false);
        };

        bootstrapAsync();
    }, [dispatch]);

    if (isRestoringToken) {
        return (
            <View style={{ flex: 1, justifyContent: 'center', alignItems: 'center' }}>
                <ActivityIndicator size="large" />
            </View>
        );
    }

    return (
        <NavigationContainer>
            <Stack.Navigator screenOptions={{ headerShown: false }}>
                {isAuthenticated ? (
                    <Stack.Screen name="Main" component={HomeScreen} />
                ) : (
                    <Stack.Screen name="Auth" component={AuthNavigator} />
                )}
            </Stack.Navigator>
        </NavigationContainer>
    );
}
