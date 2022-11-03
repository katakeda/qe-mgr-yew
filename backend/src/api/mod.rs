pub mod team;
pub mod ticket;
pub mod user;

use actix_web::web::{scope, ServiceConfig};

use self::{team::team_api, ticket::ticket_api, user::user_api};

pub fn api_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .configure(ticket_api)
            .configure(user_api)
            .configure(team_api),
    );
}
