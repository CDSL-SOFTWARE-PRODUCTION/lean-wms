// SeaORM entity definitions

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

// User entity
pub mod user {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "users")]
    #[allow(dead_code)]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub username: String,
        #[serde(skip_serializing)]
        pub password_hash: String,
        pub name: String,
        pub role: String, // WORKER, MANAGER, OWNER
        pub device_id: Option<String>,
        pub is_active: bool,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
        pub last_login_at: Option<DateTimeWithTimeZone>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    #[allow(dead_code)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// Product entity
pub mod product {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "products")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub name: String,
        pub master_sku: String,
        pub unit: Option<String>,
        pub status: String, // ACTIVE, INACTIVE, DISCONTINUED
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::barcode_mapping::Entity")]
        BarcodeMapping,
    }

    impl Related<super::barcode_mapping::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::BarcodeMapping.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// Barcode Mapping entity
pub mod barcode_mapping {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "barcode_mappings")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub product_id: Uuid,
        pub barcode: String,
        pub created_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::product::Entity",
            from = "Column::ProductId",
            to = "super::product::Column::Id"
        )]
        Product,
    }

    impl Related<super::product::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Product.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

// Order entity
pub mod order {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "orders")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub order_number: String,
        pub order_type: String, // SALES_ORDER, PRODUCTION_ORDER, TRANSFER_ORDER
        pub status: String, // PENDING, IN_PROGRESS, COMPLETED, CANCELLED
        pub created_by: Uuid,
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    #[allow(dead_code)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// Location entity
pub mod location {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
    #[sea_orm(table_name = "locations")]
    #[allow(dead_code)]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub code: String,
        pub name: String,
        pub warehouse_id: Option<Uuid>,
        pub parent_id: Option<Uuid>,
        pub location_type: String, // AREA, RACK, SHELF, BIN
        pub status: String, // ACTIVE, INACTIVE, FULL
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    #[allow(dead_code)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// Inventory Item entity
pub mod inventory_item {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "inventory_items")]
    #[allow(dead_code)]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        pub product_id: Uuid,
        pub location_id: Uuid,
        #[sea_orm(column_type = "Decimal(Some((18, 4)))")]
        pub quantity: Decimal,
        pub batch_number: Option<String>,
        pub expiry_date: Option<DateTimeWithTimeZone>,
        pub status: String, // AVAILABLE, STAGING, DAMAGED
        pub created_at: DateTimeWithTimeZone,
        pub updated_at: DateTimeWithTimeZone,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    #[allow(dead_code)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::product::Entity",
            from = "Column::ProductId",
            to = "super::product::Column::Id"
        )]
        Product,
        #[sea_orm(
            belongs_to = "super::location::Entity",
            from = "Column::LocationId",
            to = "super::location::Column::Id"
        )]
        Location,
    }

    impl Related<super::product::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Product.def()
        }
    }

    impl Related<super::location::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Location.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
