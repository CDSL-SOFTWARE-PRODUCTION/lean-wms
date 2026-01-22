import { observable } from '@legendapp/state';
import { syncedSupabase } from '@legendapp/state/sync-plugins/supabase';
import { configureSynced } from '@legendapp/state/sync';
import { supabase } from './supabase';
import 'react-native-get-random-values';
import { v4 as uuidv4 } from 'uuid';

// Configure global sync settings if needed
configureSynced({
    // persist: { ... } // We can add local persistence later if needed
});

export type Product = {
    id: string;
    sku: string;
    name: string;
    barcode: string;
    created_at?: string;
    updated_at?: string;
    deleted?: boolean;
};

// Define the store
export const store = observable({
    products: syncedSupabase({
        supabase,
        collection: 'products',
        select: (from) => from.select('id,sku,name,barcode,created_at,updated_at,deleted') as any,
        actions: ['read', 'create', 'update', 'delete'],
        fieldCreatedAt: 'created_at',
        fieldUpdatedAt: 'updated_at',
        fieldDeleted: 'deleted',
        changesSince: 'last-sync',

    }) as unknown as Record<string, Product>,
});

export const addProduct = (sku: string, name: string, barcode: string) => {
    const id = uuidv4();
    store.products[id].set({
        id,
        sku,
        name,
        barcode,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
    });
};
