import { compareAsc } from "date-fns";

export interface InventoryItem {
    id: string;
    product_id: string;
    location_id: string;
    quantity: number; // or Decimal string if handling precision on client
    status: "STAGING" | "AVAILABLE" | "RESERVED" | "DEFECT";
    expiry_date?: Date | string | null;
    created_at: Date | string;
    batch_number?: string | null;
}

/**
 * FEFO (First Expired, First Out)
 * Priority 1: Expiry Date ASC (earliest expiry first)
 * Priority 2: Created At ASC (oldest stock first)
 */
export function sortFEFO(items: InventoryItem[]): InventoryItem[] {
    return [...items].sort((a, b) => {
        // 1. Compare Expiry Date
        const expiryA = a.expiry_date ? new Date(a.expiry_date) : null;
        const expiryB = b.expiry_date ? new Date(b.expiry_date) : null;

        if (expiryA && expiryB) {
            const diff = compareAsc(expiryA, expiryB);
            if (diff !== 0) return diff;
        } else if (expiryA) {
            return -1; // A has expiry, A comes first (urgent)
        } else if (expiryB) {
            return 1; // B has expiry, B comes first
        }

        // 2. Fallback to FIFO (Created At)
        const createdA = new Date(a.created_at);
        const createdB = new Date(b.created_at);
        return compareAsc(createdA, createdB);
    });
}

/**
 * FIFO (First In, First Out)
 * Priority: Created At ASC
 */
export function sortFIFO(items: InventoryItem[]): InventoryItem[] {
    return [...items].sort((a, b) => {
        const createdA = new Date(a.created_at);
        const createdB = new Date(b.created_at);
        return compareAsc(createdA, createdB);
    });
}
