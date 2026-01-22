-- Create products table with support for Legend State Sync (soft deletes)
create table products (
  id uuid primary key default gen_random_uuid(),
  sku text unique not null,
  name text,
  barcode text,
  created_at timestamptz default now(),
  updated_at timestamptz default now(),
  deleted boolean default false
);
