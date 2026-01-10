// // Unit tests for auth slice

// import authReducer, { logout, clearError } from '../authSlice';
// import type { AuthState } from '../authSlice';

// describe('authSlice', () => {
//   const initialState: AuthState = {
//     user: null,
//     token: null,
//     refreshToken: null,
//     isAuthenticated: false,
//     loading: false,
//     error: null,
//   };

//   it('should return initial state', () => {
//     expect(authReducer(undefined, { type: 'unknown' })).toEqual(initialState);
//   });

//   it('should handle logout', () => {
//     const state: AuthState = {
//       ...initialState,
//       user: { id: '1', email: 'test@example.com', name: 'Test' },
//       token: 'test-token',
//       isAuthenticated: true,
//     };

//     const newState = authReducer(state, logout());
//     expect(newState.isAuthenticated).toBe(false);
//     expect(newState.token).toBeNull();
//     expect(newState.user).toBeNull();
//   });

//   it('should handle clearError', () => {
//     const state: AuthState = {
//       ...initialState,
//       error: 'Some error',
//     };

//     const newState = authReducer(state, clearError());
//     expect(newState.error).toBeNull();
//   });
// });
