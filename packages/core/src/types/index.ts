export interface Location {
    id: string;
    code: string; // e.g. WH01-R01-S02-B03
    type: "WAREHOUSE" | "RACK" | "SHELF" | "BIN" | "STAGING" | "DOCK";
    status: "ACTIVE" | "INACTIVE";
    max_capacity?: number;
    current_quantity?: number; // Calculated/Cached
    fixed_product_id?: string | null;
    is_full?: boolean; // Manual flag
}

export * from "../inventory"; // Re-export inventory types if they were defined there, or better, move InventoryItem here.
