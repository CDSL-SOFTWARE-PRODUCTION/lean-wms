// Test setup file for mobile
// Note: @testing-library/react-native v13.3.3+ includes built-in Jest matchers
// No need to import @testing-library/jest-native

// Mock expo-secure-store
jest.mock('expo-secure-store', () => ({
  getItemAsync: jest.fn(),
  setItemAsync: jest.fn(),
  deleteItemAsync: jest.fn(),
}));
