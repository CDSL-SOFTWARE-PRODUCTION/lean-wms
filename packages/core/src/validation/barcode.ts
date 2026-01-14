import { z } from "zod";

export const barcodeSchema = z
    .string()
    .min(3, "Barcode must be at least 3 characters")
    .max(50, "Barcode must be at most 50 characters")
    .regex(/^[a-zA-Z0-9-_]+$/, "Barcode can only contain alphanumeric characters, hyphens, and underscores");

export const skuSchema = z
    .string()
    .min(3, "SKU must be at least 3 characters")
    .max(30, "SKU must be at most 30 characters")
    .regex(/^[a-zA-Z0-9-_.]+$/, "SKU can only contain alphanumeric characters, hyphens, underscores, and dots");

export function validateBarcode(code: string) {
    return barcodeSchema.safeParse(code);
}

export function validateSKU(code: string) {
    return skuSchema.safeParse(code);
}
