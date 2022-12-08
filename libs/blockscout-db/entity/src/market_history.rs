//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "market_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub date: Option<Date>,
    pub closing_price: Option<Decimal>,
    pub opening_price: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
