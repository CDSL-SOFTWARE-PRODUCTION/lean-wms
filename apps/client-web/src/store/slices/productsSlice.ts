// Products slice

import { createSlice, createAsyncThunk } from '@reduxjs/toolkit';
import type { PayloadAction } from '@reduxjs/toolkit';
import { apiClient } from '../../services/api';
import type { Product } from '../../services/api';

interface ProductsState {
  items: Product[];
  currentProduct: Product | null;
  loading: boolean;
  error: string | null;
}

const initialState: ProductsState = {
  items: [],
  currentProduct: null,
  loading: false,
  error: null,
};

export const fetchProducts = createAsyncThunk(
  'products/fetchProducts',
  async (params: { page?: number; limit?: number; search?: string } | undefined, { rejectWithValue }) => {
    try {
      const response = await apiClient.getProducts(params);
      return response.items || response;
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Failed to fetch products';
      return rejectWithValue(message);
    }
  }
);

export const fetchProduct = createAsyncThunk(
  'products/fetchProduct',
  async (id: string, { rejectWithValue }) => {
    try {
      const response = await apiClient.getProduct(id);
      return response;
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Failed to fetch product';
      return rejectWithValue(message);
    }
  }
);

export const createProduct = createAsyncThunk(
  'products/createProduct',
  async (data: Partial<Product>, { rejectWithValue }) => {
    try {
      const response = await apiClient.createProduct(data);
      return response;
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Failed to create product';
      return rejectWithValue(message);
    }
  }
);

export const updateProduct = createAsyncThunk(
  'products/updateProduct',
  async ({ id, data }: { id: string; data: Partial<Product> }, { rejectWithValue }) => {
    try {
      const response = await apiClient.updateProduct(id, data);
      return response;
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Failed to update product';
      return rejectWithValue(message);
    }
  }
);

export const deleteProduct = createAsyncThunk(
  'products/deleteProduct',
  async (id: string, { rejectWithValue }) => {
    try {
      await apiClient.deleteProduct(id);
      return id;
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : 'Failed to delete product';
      return rejectWithValue(message);
    }
  }
);

const productsSlice = createSlice({
  name: 'products',
  initialState,
  reducers: {
    clearError: (state) => {
      state.error = null;
    },
    clearCurrentProduct: (state) => {
      state.currentProduct = null;
    },
  },
  extraReducers: (builder) => {
    builder
      // Fetch products
      .addCase(fetchProducts.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchProducts.fulfilled, (state, action: PayloadAction<Product[]>) => {
        state.loading = false;
        state.items = action.payload;
      })
      .addCase(fetchProducts.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      // Fetch product
      .addCase(fetchProduct.pending, (state) => {
        state.loading = true;
        state.error = null;
      })
      .addCase(fetchProduct.fulfilled, (state, action: PayloadAction<Product>) => {
        state.loading = false;
        state.currentProduct = action.payload;
      })
      .addCase(fetchProduct.rejected, (state, action) => {
        state.loading = false;
        state.error = action.payload as string;
      })
      // Create product
      .addCase(createProduct.fulfilled, (state, action: PayloadAction<Product>) => {
        state.items.push(action.payload);
      })
      // Update product
      .addCase(updateProduct.fulfilled, (state, action: PayloadAction<Product>) => {
        const index = state.items.findIndex((p) => p.id === action.payload.id);
        if (index !== -1) {
          state.items[index] = action.payload;
        }
        if (state.currentProduct?.id === action.payload.id) {
          state.currentProduct = action.payload;
        }
      })
      // Delete product
      .addCase(deleteProduct.fulfilled, (state, action: PayloadAction<string>) => {
        state.items = state.items.filter((p) => p.id !== action.payload);
        if (state.currentProduct?.id === action.payload) {
          state.currentProduct = null;
        }
      });
  },
});

export const { clearError, clearCurrentProduct } = productsSlice.actions;
export default productsSlice.reducer;

