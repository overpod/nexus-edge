use sea_orm::{
    ActiveValue::Set, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, sqlx::types::chrono,
};
use std::env;

use crate::utils::encryption::hash_password;
use ::entity::{user, user::Entity as User};
use cuid::cuid2;

pub async fn seed(db: &DbConn) -> Result<(), DbErr> {
    users(db).await?;
    Ok(())
}

async fn users(db: &DbConn) -> Result<(), DbErr> {
    let username = env::var("ADMIN_USERNAME").expect("ADMIN_USERNAME env var not set");
    let password = env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD env var not set");

    let user = User::find()
        .filter(user::Column::Login.eq("admin"))
        .one(db)
        .await
        .unwrap();

    if user.is_some() {
        return Ok(());
    }

    let new_user = user::ActiveModel {
        id: Set(cuid2()),
        created_at: Set(chrono::Utc::now()),
        updated_at: Set(chrono::Utc::now()),
        login: Set(username.clone()),
        password: Set(hash_password(&password)),
        role: Set(user::Role::Admin),
        language: Set(user::Language::En),
        color_scheme: Set(user::ColorScheme::Light),
    };
    User::insert(new_user.clone()).exec(db).await?;
    log::info!("Seeded user: {:?}", username);

    Ok(())
}
