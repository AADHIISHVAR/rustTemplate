use sea_orm::entity::prelude::*;
use sea_orm::ActiveModelTrait;

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "register_entity")]
pub struct Model
{
    #[sea_orm(primary_key)]
    pub id: i32,
    
    #[sea_orm(unique)]
    pub username: String,
    
    #[sea_orm(unique)]
    pub email: String,
    
    pub password: String,
    
    pub name: String 
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        panic!("No RelationDef for this entity")
    }
}

