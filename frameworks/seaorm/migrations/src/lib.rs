pub use sea_orm_migration::prelude::*;

mod m20250502_083221_create_bakery_table;
mod m20250502_083902_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250502_083221_create_bakery_table::Migration),
            Box::new(m20250502_083902_create_chef_table::Migration),
        ]
    }
}
