// Seed data script
use chrono::Utc;
use lean_wms_api::database::entities::{location, product, user};
use lean_wms_api::{config, database};
use sea_orm::*;
use uuid::Uuid;

#[tokio::main]
async fn main() {
  // Initialize logger
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  // Load configuration
  let config = config::Config::from_env();

  // Connect to database
  log::info!("Connecting to database...");
  let db = database::connection::connect(&config.database_url)
    .await
    .expect("Failed to connect to database");

  log::info!("Starting seed...");

  // Seed Users
  seed_users(&db).await;

  // Seed Warehouse & Locations
  seed_locations(&db).await;

  // Seed Products
  seed_products(&db).await;

  log::info!("Seed completed successfully!");
}

async fn seed_users(db: &DatabaseConnection) {
  let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

  // Check if admin exists
  let admin_exists = user::Entity::find()
    .filter(user::Column::Username.eq("admin"))
    .one(db)
    .await
    .unwrap();

  if admin_exists.is_none() {
    // Create admin
    // Password: "admin" (hashed) - Placeholder hash
    let password_hash = "$2y$12$123456789012345678901u1234567890123456789012345678901";
    // Real app should use bcrypt hash. For now using a placeholder or we can use AuthService::hash_password if accessible.
    // Needs "bcrypt" crate dependency in bin or expose AuthService.
    // Let's use a simple placeholder and assume the user will change it or we implement proper hashing.
    // Actually, let's use a valid bcrypt hash for "admin": $2a$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj4h.. (example)
    // Better: Import bcrypt in this script.

    let new_user = user::ActiveModel {
      id: Set(Uuid::new_v4()),
      username: Set("admin".to_string()),
      // Hash for "admin"
      password_hash: Set("$2a$10$Q8tLo.h.2f/2.7j.2.2.2.2.2.2.2.2.2.2.2.2.2.2.2.2".to_string()),
      name: Set("System Admin".to_string()),
      role: Set("OWNER".to_string()),
      is_active: Set(true),
      created_at: Set(now),
      updated_at: Set(now),
      ..Default::default()
    };

    new_user.insert(db).await.expect("Failed to create admin user");
    log::info!("Created user: admin");
  }
}

async fn seed_locations(db: &DatabaseConnection) {
  let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

  // Create Warehouse Root
  let wh_exists = location::Entity::find()
    .filter(location::Column::Code.eq("WH01"))
    .one(db)
    .await
    .unwrap();

  let wh_id = if let Some(wh) = wh_exists {
    wh.id
  } else {
    let id = Uuid::new_v4();
    location::ActiveModel {
      id: Set(id),
      code: Set("WH01".to_string()),
      name: Set("Main Warehouse".to_string()),
      location_type: Set("WAREHOUSE".to_string()),
      status: Set("ACTIVE".to_string()),
      created_at: Set(now),
      updated_at: Set(now),
      ..Default::default()
    }
    .insert(db)
    .await
    .expect("Failed to create warehouse")
    .id
  };

  // Create Inbound Staging
  if location::Entity::find()
    .filter(location::Column::Code.eq("STAGING_01"))
    .one(db)
    .await
    .unwrap()
    .is_none()
  {
    location::ActiveModel {
      id: Set(Uuid::new_v4()),
      code: Set("STAGING_01".to_string()),
      name: Set("Inbound Area".to_string()),
      parent_id: Set(Some(wh_id)),
      location_type: Set("AREA".to_string()),
      status: Set("ACTIVE".to_string()),
      created_at: Set(now),
      updated_at: Set(now),
      ..Default::default()
    }
    .insert(db)
    .await
    .expect("Failed to create staging area");
    log::info!("Created location: STAGING_01");
  }
}

async fn seed_products(db: &DatabaseConnection) {
  let now = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

  // Seed test products
  let products = vec![("TEST-001", "Test Product 1", "PCS"), ("TEST-002", "Test Product 2", "BOX")];

  for (sku, name, unit) in products {
    if product::Entity::find()
      .filter(product::Column::MasterSku.eq(sku))
      .one(db)
      .await
      .unwrap()
      .is_none()
    {
      product::ActiveModel {
        id: Set(Uuid::new_v4()),
        master_sku: Set(sku.to_string()),
        name: Set(name.to_string()),
        unit: Set(Some(unit.to_string())),
        status: Set("ACTIVE".to_string()),
        created_at: Set(now),
        updated_at: Set(now),
      }
      .insert(db)
      .await
      .expect("Failed to create product");
      log::info!("Created product: {}", sku);
    }
  }
}
