// Orders slice

import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit';
import { apiClient } from '../../services/api';
import type { Order } from '../../services/api';

interface OrdersState {
  items: Order[];
  currentOrder: Order | null;
  loading: boolean;
  error: string | null;
}

const initialState: OrdersState = {
  items: [],
  currentOrder: null,
  loading: false,
  error: null,
};

export const fetchOrders = createAsyncThunk(
  'orders/fetchOrders',
  async (
    params: { page?: number; limit?: number; status?: string } | undefined,
    { rejectWithValue }
  ) => {
    try {
      const response = await apiClient.getOrders(params);
      return response.items || response;
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : 'Failed to fetch orders';
      return rejectWithValue(message);
    }
  }
);

export const fetchOrder = createAsyncThunk(
  'orders/fetchOrder',
  async (id: string, { rejectWithValue }) => {
    try {
      const response = await apiClient.getOrder(id);
      return response;
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : 'Failed to fetch order';
      return rejectWithValue(message);
    }
  }
);

export const createOrder = createAsyncThunk(
  'orders/createOrder',
  async (data: Partial<Order>, { rejectWithValue }) => {
    try {
      const response = await apiClient.createOrder(data);
      return response;
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : 'Failed to create order';
      return rejectWithValue(message);
    }
  }
);

export const updateOrder = createAsyncThunk(
  'orders/updateOrder',
  async (
    { id, data }: { id: string; data: Partial<Order> },
    { rejectWithValue }
  ) => {
    try {
      const response = await apiClient.updateOrder(id, data);
      return response;
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : 'Failed to update order';
      return rejectWithValue(message);
    }
  }
);

export const deleteOrder = createAsyncThunk(
  'orders/deleteOrder',
  async (id: string, { rejectWithValue }) => {
    try {
      await apiClient.deleteOrder(id);
      return id;
    } catch (error: unknown) {
      const message =
        error instanceof Error ? error.message : 'Failed to delete order';
      return rejectWithValue(message);
    }
  }
);

const ordersSlice = createSlice({
  name: 'orders',
  initialState,
  reducers: {
    clearError: (state) => {
      state.error = null;
    },
    clearCurrentOrder: (state) => {
      state.currentOrder = null;
    },
  },
  extraReducers: (builder) => {
    builder
      // Fetch orders
      .addCase(fetchOrders.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(
        fetchOrders.fulfilled,
        (state, action: PayloadAction<Order[]>) => {
          state.loading = false;
          state.items = action.payload;
        }
      )
      .addCase(fetchOrders.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      // Fetch order
      .addCase(fetchOrder.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchOrder.fulfilled, (state, action: PayloadAction<Order>) => {
        state.loading = false;
        state.currentOrder = action.payload;
      })
      .addCase(fetchOrder.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      // Create order
      .addCase(createOrder.fulfilled, (state, action: PayloadAction<Order>) => {
        state.items.push(action.payload);
      })
      // Update order
      .addCase(updateOrder.fulfilled, (state, action: PayloadAction<Order>) => {
        const index = state.items.findIndex((o) => o.id === action.payload.id);
        if (index !== -1) {
          state.items[index] = action.payload;
        }
        if (state.currentOrder?.id === action.payload.id) {
          state.currentOrder = action.payload;
        }
      })
      // Delete order
      .addCase(
        deleteOrder.fulfilled,
        (state, action: PayloadAction<string>) => {
          state.items = state.items.filter((o) => o.id !== action.payload);
          if (state.currentOrder?.id === action.payload) {
            state.currentOrder = null;
          }
        }
      );
  },
});

export const { clearError, clearCurrentOrder } = ordersSlice.actions;
export default ordersSlice.reducer;
