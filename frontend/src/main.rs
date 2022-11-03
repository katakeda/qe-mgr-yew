mod common;
mod components;

use common::ComponentProps;
use components::{header::Header, home::Home};
use reqwasm::http::Request;
use serde::Deserialize;
use std::vec;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, ContextProvider};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub assigned_to: Option<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppContext {
    pub users: Vec<User>,
    pub teams: Vec<Team>,
    pub tickets: Vec<Ticket>,
    pub current_team: Option<String>,
    pub update_current_team: Callback<String>,
}

async fn refresh_tickets(team: String) -> Vec<Ticket> {
    let mut tickets = vec![];
    let response = Request::get(&format!("/api/tickets/?team={}", team))
        .send()
        .await;
    if let Ok(r) = response {
        tickets = r.json::<Vec<Ticket>>().await.unwrap_or(tickets);
    }
    tickets
}

#[styled_component(StyledMain)]
fn styled_app(props: &ComponentProps) -> Html {
    html! {
        <main class={css!("
            background-color: #74be43;
            height: 100vh;
        ")}>
        {props.children.clone()}
        </main>
    }
}

#[function_component(App)]
fn app() -> Html {
    let users = use_state(|| Vec::<User>::new());
    let teams = use_state(|| Vec::<Team>::new());
    let tickets = use_state(|| Vec::<Ticket>::new());
    let current_team = use_state(|| None::<String>);

    {
        let users = users.clone();
        let teams = teams.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let response = Request::get("/api/users/").send().await;
                    if let Ok(r) = response {
                        users.set(r.json::<Vec<User>>().await.unwrap_or((*users).clone()));
                    }
                    let response = Request::get("/api/teams/").send().await;
                    if let Ok(r) = response {
                        teams.set(r.json::<Vec<Team>>().await.unwrap_or((*teams).clone()));
                    }
                });
                || ()
            },
            (),
        );
    }
    {
        let tickets = tickets.clone();
        let current_team = current_team.clone();
        use_effect_with_deps(
            move |team| {
                let team = team.clone();
                spawn_local(async move {
                    tickets.set(refresh_tickets((*team).clone().unwrap_or("".into())).await);
                });
                || ()
            },
            current_team.clone(),
        );
    }
    let update_current_team = {
        let current_team = current_team.clone();
        Callback::from(move |team| {
            current_team.set(Some(team));
        })
    };

    html! {
        <ContextProvider<AppContext> context={AppContext{
            users: (*users).clone(),
            teams: (*teams).clone(),
            tickets: (*tickets).clone(),
            current_team: (*current_team).clone(),
            update_current_team,
        }}>
            <StyledMain>
                <Header />
                <Home />
            </StyledMain>
        </ContextProvider<AppContext>>
    }
}

fn main() {
    yew::start_app::<App>();
}
