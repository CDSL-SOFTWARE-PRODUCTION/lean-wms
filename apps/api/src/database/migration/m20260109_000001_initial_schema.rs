use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Users table
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Users::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(ColumnDef::new(Users::DeviceId).string())
                    .col(ColumnDef::new(Users::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Users::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Users::LastLoginAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // Products table
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Products::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Products::Name).string().not_null())
                    .col(ColumnDef::new(Products::MasterSku).string().not_null().unique_key())
                    .col(ColumnDef::new(Products::Unit).string())
                    .col(ColumnDef::new(Products::Status).string().not_null())
                    .col(ColumnDef::new(Products::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Products::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Barcode Mappings table
        manager
            .create_table(
                Table::create()
                    .table(BarcodeMappings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BarcodeMappings::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(BarcodeMappings::ProductId).uuid().not_null())
                    .col(ColumnDef::new(BarcodeMappings::Barcode).string().not_null().unique_key())
                    .col(ColumnDef::new(BarcodeMappings::CreatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-barcode_mappings-product_id")
                            .from(BarcodeMappings::Table, BarcodeMappings::ProductId)
                            .to(Products::Table, Products::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Locations table
        manager
            .create_table(
                Table::create()
                    .table(Locations::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Locations::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Locations::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Locations::Name).string().not_null())
                    .col(ColumnDef::new(Locations::WarehouseId).uuid())
                    .col(ColumnDef::new(Locations::ParentId).uuid())
                    .col(ColumnDef::new(Locations::LocationType).string().not_null())
                    .col(ColumnDef::new(Locations::Status).string().not_null())
                    .col(ColumnDef::new(Locations::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Locations::UpdatedAt).timestamp_with_time_zone().not_null())
                    .to_owned(),
            )
            .await?;

        // Inventory Items table
        manager
            .create_table(
                Table::create()
                    .table(InventoryItems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(InventoryItems::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(InventoryItems::ProductId).uuid().not_null())
                    .col(ColumnDef::new(InventoryItems::LocationId).uuid().not_null())
                    .col(ColumnDef::new(InventoryItems::Quantity).decimal_len(18, 4).not_null())
                    .col(ColumnDef::new(InventoryItems::BatchNumber).string())
                    .col(ColumnDef::new(InventoryItems::ExpiryDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(InventoryItems::Status).string().not_null())
                    .col(ColumnDef::new(InventoryItems::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(InventoryItems::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-inventory_items-product_id")
                            .from(InventoryItems::Table, InventoryItems::ProductId)
                            .to(Products::Table, Products::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-inventory_items-location_id")
                            .from(InventoryItems::Table, InventoryItems::LocationId)
                            .to(Locations::Table, Locations::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Orders table
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Orders::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Orders::OrderNumber).string().not_null().unique_key())
                    .col(ColumnDef::new(Orders::OrderType).string().not_null())
                    .col(ColumnDef::new(Orders::Status).string().not_null())
                    .col(ColumnDef::new(Orders::CreatedBy).uuid().not_null())
                    .col(ColumnDef::new(Orders::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Orders::UpdatedAt).timestamp_with_time_zone().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-orders-created_by")
                            .from(Orders::Table, Orders::CreatedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Orders::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(InventoryItems::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Locations::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(BarcodeMappings::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Products::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    PasswordHash,
    Name,
    Role,
    DeviceId,
    IsActive,
    CreatedAt,
    UpdatedAt,
    LastLoginAt,
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    Name,
    MasterSku,
    Unit,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum BarcodeMappings {
    Table,
    Id,
    ProductId,
    Barcode,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Locations {
    Table,
    Id,
    Code,
    Name,
    WarehouseId,
    ParentId,
    LocationType,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum InventoryItems {
    Table,
    Id,
    ProductId,
    LocationId,
    Quantity,
    BatchNumber,
    ExpiryDate,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    OrderNumber,
    OrderType,
    Status,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}
