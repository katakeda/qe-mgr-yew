use actix_web::{
    get, post,
    web::{scope, Data, Json, ServiceConfig},
    Responder, Result,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{model::team::Team, repository::inmem::Inmem};

#[derive(Deserialize)]
pub struct CreateTeam {
    pub name: String,
}

#[get("/")]
pub async fn find_all(repo: Data<Inmem>) -> Result<impl Responder> {
    Ok(Json(repo.get_teams()))
}

#[post("/")]
pub async fn create(body: Json<CreateTeam>, repo: Data<Inmem>) -> Result<impl Responder> {
    let team = Team {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
    };
    repo.create_team(team.clone());
    Ok(Json([team]))
}

pub fn team_api(cfg: &mut ServiceConfig) {
    cfg.service(scope("/teams").service(find_all).service(create));
}
