use ::entity::{name, name::Entity as Name, paper, paper::Entity as Paper};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn list_names(db: &DbConn) -> Result<Vec<name::Model>, DbErr> {
        Name::find().all(db).await
    }

    pub async fn find_paper_by_pid(db: &DbConn, pid: &str) -> Result<Option<paper::Model>, DbErr> {
        Paper::find()
            .filter(paper::Column::Pid.eq(pid))
            .one(db)
            .await
    }

    pub async fn list_papers(db: &DbConn) -> Result<Vec<paper::Model>, DbErr> {
        Paper::find().all(db).await
    }
}
