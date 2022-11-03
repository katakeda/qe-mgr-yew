use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    Responder, Result,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{model::user::User, repository::inmem::Inmem};

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}

#[get("/")]
pub async fn find_all(repo: Data<Inmem>) -> Result<impl Responder> {
    Ok(Json(repo.get_users()))
}

#[post("/")]
pub async fn create(body: Json<CreateUser>, repo: Data<Inmem>) -> Result<impl Responder> {
    let user = User {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
    };
    repo.create_user(user.clone());
    Ok(Json([user]))
}

pub fn user_api(cfg: &mut ServiceConfig) {
    cfg.service(scope("/users").service(find_all).service(create));
}
