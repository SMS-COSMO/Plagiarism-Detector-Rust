use ::entity::{name, name::Entity as Name, paper};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn add_paper(
        db: &DbConn,
        id: &str,
        tf_array: &Vec<serde_json::Value>,
    ) -> Result<paper::ActiveModel, DbErr> {
        paper::ActiveModel {
            pid: Set(id.to_owned()),
            text: Set(serde_json::json!(tf_array).to_string()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn add_name(db: &DbConn, name: &str, df: i32) -> Result<name::ActiveModel, DbErr> {
        name::ActiveModel {
            name: Set(name.to_owned()),
            df: Set(df),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_name(db: &DbConn, name: &str, df: i32) -> Result<name::Model, DbErr> {
        let res: name::ActiveModel = Name::find()
            .filter(name::Column::Name.eq(name))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find name".to_owned()))
            .map(Into::into)?;

        name::ActiveModel {
            id: res.id,
            name: Set(name.to_owned()),
            df: Set(df),
        }
        .update(db)
        .await
    }
}
