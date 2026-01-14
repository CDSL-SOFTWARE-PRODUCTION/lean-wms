import { Location } from "../types";

export const LOCATION_CODE_REGEX = /^[A-Z0-9]+(-[A-Z0-9]+)*$/;

export function validateLocationCode(code: string): boolean {
    return LOCATION_CODE_REGEX.test(code);
}

export function isStorageLocation(type: string): boolean {
    return type === "BIN" || type === "SHELF";
}

export function canAccommodate(location: Location, addedQty: number): boolean {
    // 1. Check manual full flag
    if (location.is_full) {
        return false;
    }

    // 2. Check quantitative capacity (if defined)
    if (location.max_capacity !== undefined && location.max_capacity !== null) {
        const current = location.current_quantity || 0;
        return current + addedQty <= location.max_capacity;
    }

    // 3. Unlimited or undefined capacity -> Allow
    return true;
}
