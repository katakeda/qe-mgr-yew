use actix_web::{
    delete,
    error::ErrorNotFound,
    get, post, put,
    web::{scope, Data, Json, Path, Query, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    model::{
        team::Team,
        ticket::{Ticket, TicketStatus},
        user::User,
    },
    repository::inmem::Inmem,
};

#[derive(Deserialize)]
pub struct TicketFilter {
    pub status: Option<TicketStatus>,
    pub team: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
    pub assigned_to: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
    pub assigned_to: Option<String>,
    pub team_id: Option<String>,
}

#[derive(Serialize)]
pub struct TicketResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub created_by: Option<User>,
    pub assigned_to: Option<User>,
    pub team: Option<Team>,
}

#[get("/{id}")]
pub async fn find(id: Path<String>, repo: Data<Inmem>) -> Result<impl Responder> {
    match repo.get_ticket(id.to_string()) {
        Some(ticket) => Ok(Json([create_ticket_response(ticket, repo)])),
        None => Err(ErrorNotFound("Not found")),
    }
}

#[get("/")]
pub async fn find_all(filter: Query<TicketFilter>, repo: Data<Inmem>) -> Result<impl Responder> {
    let mut ticket_responses = vec![];
    for ticket in repo.get_tickets(&filter).iter() {
        ticket_responses.push(create_ticket_response(ticket.clone(), repo.clone()));
    }
    Ok(Json(ticket_responses))
}

#[post("/")]
pub async fn create(
    req: HttpRequest,
    body: Json<CreateTicket>,
    repo: Data<Inmem>,
) -> Result<impl Responder> {
    let user_id = get_user_id(req);
    let ticket = Ticket {
        id: Uuid::new_v4().to_string(),
        title: body.title.clone(),
        description: body.description.clone().unwrap_or("".into()),
        status: body.status.clone().unwrap_or(TicketStatus::New),
        assigned_to: body.assigned_to.clone().unwrap_or("".into()),
        team_id: body.team_id.clone().unwrap_or("".into()),
        created_by: user_id,
    };
    repo.create_ticket(ticket.clone());
    Ok(HttpResponse::Created().json([create_ticket_response(ticket, repo)]))
}

#[put("/{id}")]
pub async fn update(
    id: Path<String>,
    req: HttpRequest,
    body: Json<UpdateTicket>,
    repo: Data<Inmem>,
) -> Result<impl Responder> {
    match repo.get_ticket(id.to_string()) {
        Some(ticket) => {
            let ticket = Ticket {
                id: ticket.id.clone(),
                title: body.title.clone().unwrap_or(ticket.title.clone()),
                description: body
                    .description
                    .clone()
                    .unwrap_or(ticket.description.clone()),
                status: body.status.clone().unwrap_or(ticket.status.clone()),
                assigned_to: body
                    .assigned_to
                    .clone()
                    .unwrap_or(ticket.assigned_to.clone()),
                team_id: body.team_id.clone().unwrap_or(ticket.team_id.clone()),
                created_by: ticket.created_by.clone(),
            };
            repo.create_ticket(ticket.clone());
            Ok(HttpResponse::Ok().json([create_ticket_response(ticket, repo)]))
        }
        None => {
            let user_id = get_user_id(req);
            let ticket = Ticket {
                id: Uuid::new_v4().to_string(),
                title: body.title.clone().unwrap_or("".into()),
                description: body.description.clone().unwrap_or("".into()),
                status: body.status.clone().unwrap_or(TicketStatus::New),
                assigned_to: body.assigned_to.clone().unwrap_or("".into()),
                team_id: body.team_id.clone().unwrap_or("".into()),
                created_by: user_id,
            };
            repo.create_ticket(ticket.clone());
            Ok(HttpResponse::Created().json([create_ticket_response(ticket, repo)]))
        }
    }
}

#[delete("/{id}")]
pub async fn delete(id: Path<String>, repo: Data<Inmem>) -> Result<impl Responder> {
    repo.delete_ticket(id.to_string());
    Ok(HttpResponse::Ok().finish())
}

pub fn ticket_api(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/tickets")
            .service(find)
            .service(find_all)
            .service(create)
            .service(update)
            .service(delete),
    );
}

fn create_ticket_response(ticket: Ticket, repo: Data<Inmem>) -> TicketResponse {
    TicketResponse {
        id: ticket.id.clone(),
        title: ticket.title.clone(),
        description: ticket.description.clone(),
        status: ticket.status.clone(),
        assigned_to: repo.get_user(ticket.assigned_to.clone()),
        created_by: repo.get_user(ticket.created_by.clone()),
        team: repo.get_team(ticket.team_id.clone()),
    }
}

fn get_user_id(req: HttpRequest) -> String {
    let mut user_id = "".into();
    if let Some(cookie) = req.cookie("user_id") {
        user_id = cookie.value().into();
    }
    user_id
}
