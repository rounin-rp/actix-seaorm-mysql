use ::entity::{user, user::Entity as User};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            first_name: Set(form_data.first_name.to_owned()),
            last_name: Set(form_data.last_name.to_owned()),
            email: Set(form_data.email.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
    pub async fn update_user_by_id(
        db: &DbConn,
        id: i32,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Invalid User Id".to_owned()))
            .map(Into::into)?;
        user::ActiveModel {
            id: user.id,
            first_name: Set(form_data.first_name.to_owned()),
            last_name: Set(form_data.last_name.to_owned()),
            email: Set(form_data.email.to_owned()),
        }
        .update(db)
        .await
    }
    pub async fn delete_user(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Invalid User Id.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }
}
