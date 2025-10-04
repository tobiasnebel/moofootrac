use sea_orm_migration::{MigrationTrait, MigratorTrait, async_trait};

pub mod v1_001_create_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(v1_001_create_tables::Migration)]
    }
}
