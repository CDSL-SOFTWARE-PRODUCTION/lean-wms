export type AuthStackParamList = {
    Login: undefined;
    Register: undefined;
};

export type MainStackParamList = {
    Home: undefined;
    Inventory: undefined;
    Inbound: undefined;
    Outbound: undefined;
};

// Root Navigator can switch between Auth and Main (handled by conditional rendering)
// usually doesn't need a ParamList unless using a Switch Navigator which is deprecated in v5+
// But we will use conditional rendering in AppNavigator.
