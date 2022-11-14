use super::modal::Modal;
use crate::{common::ComponentProps, AppContext};
use gloo_net::http::Request;
use serde_json::json;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::{
    function_component, html, use_context, use_state, Callback, Html, Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct CardCreateModalProps {
    pub status: String,
    pub close: Callback<()>,
}

pub struct NewTicket {
    pub title: String,
    pub description: String,
    pub status: String,
    pub assigned_to: String,
    pub team_id: String,
}

async fn create_ticket(ticket: &NewTicket) {
    let _response = Request::post("/api/tickets/")
        .json(&json!({
            "title": ticket.title,
            "description": ticket.description,
            "status": ticket.status,
            "assigned_to": ticket.assigned_to,
            "team_id": ticket.team_id,
        }))
        .unwrap()
        .send()
        .await;
}

#[styled_component(StyledCardCreateModal)]
pub fn styled_card_create_modal(props: &ComponentProps) -> Html {
    html! {
        <div class={css!("
            display: flex;
            flex-direction: column;
            margin-top: 40px;
            width: 100%;
            padding: 16px;
            .new-ticket-group {
                display: flex;
                justify-content: space-between;
                margin-top: 20px;
            }
            .new-ticket-group > span {
                width: 20%;
            }
            .new-ticket-input-group {
                display: flex;
                flex-grow: 1;
            }
            .new-ticket-input-group > input {
                width: 100%;
            }
            .new-ticket-input-group > textarea {
                width: 100%;
                height: 65px;
                font-family: sans-serif;
                resize: none;
            }
            .new-ticket-action-group {
                display: flex;
                justify-content: end;
                margin-top: 20px;
            }
            .new-ticket-action-group > button {
                margin-left: 5px;
            }
        ")}>{props.children.clone()}</div>
    }
}

#[function_component(CardCreateModal)]
pub fn card_create_modal(props: &CardCreateModalProps) -> Html {
    let context = use_context::<AppContext>().expect("no ctx found");
    let team_id = {
        let current_team = context.current_team.clone();
        let team = context.teams.clone().into_iter().find(|t| {
            if let Some(team) = current_team.clone() {
                team == t.name.clone()
            } else {
                false
            }
        });
        match team {
            Some(t) => t.id.clone(),
            None => String::from(""),
        }
    };
    let new_ticket = use_state(|| NewTicket {
        title: String::from(""),
        description: String::from(""),
        assigned_to: String::from(""),
        status: props.status.clone(),
        team_id: team_id.clone(),
    });

    let reset_ticket = {
        let new_ticket = new_ticket.clone();
        let status = props.status.clone();
        let team_id = team_id.clone();
        Callback::from(move |_| {
            new_ticket.set(NewTicket {
                title: "".into(),
                description: "".into(),
                assigned_to: "".into(),
                status: status.clone(),
                team_id: team_id.clone(),
            });
        })
    };

    let save_ticket = {
        let new_ticket = new_ticket.clone();
        let update_tickets = context.update_tickets.clone();
        let close = props.close.clone();
        Callback::from(move |_| {
            let new_ticket = new_ticket.clone();
            let update_tickets = update_tickets.clone();
            let close = close.clone();
            spawn_local(async move {
                create_ticket(&new_ticket).await;
                update_tickets.emit(());
                close.emit(());
            });
        })
    };

    html! {
        <Modal close={props.close.clone()}>
            <StyledCardCreateModal>
                <div class="new-ticket-group">
                    <span>{"Title:"}</span>
                    <div class="new-ticket-input-group">
                        <input
                            type="text"
                            value={new_ticket.title.clone()}
                            onchange={{
                                let new_ticket = new_ticket.clone();
                                Callback::from(move |e: Event| {
                                    new_ticket.set(NewTicket {
                                        title: e.target_dyn_into::<HtmlInputElement>().unwrap().value(),
                                        description: new_ticket.description.clone(),
                                        status: new_ticket.status.clone(),
                                        assigned_to: new_ticket.assigned_to.clone(),
                                        team_id: new_ticket.team_id.clone(),
                                    });
                                })
                            }}
                        />
                    </div>
                </div>
                <div class="new-ticket-group">
                    <span>{"Description:"}</span>
                    <div class="new-ticket-input-group">
                        <textarea
                            value={new_ticket.description.clone()}
                            onchange={{
                                let new_ticket = new_ticket.clone();
                                Callback::from(move |e: Event| {
                                    new_ticket.set(NewTicket {
                                        title: new_ticket.title.clone(),
                                        description: e.target_dyn_into::<HtmlTextAreaElement>().unwrap().value(),
                                        status: new_ticket.status.clone(),
                                        assigned_to: new_ticket.assigned_to.clone(),
                                        team_id: new_ticket.team_id.clone(),
                                    });
                                })
                            }}
                        />
                    </div>
                </div>
                <div class="new-ticket-group">
                    <span>{"Assigned To:"}</span>
                    <div class="new-ticket-input-group">
                        <select
                            value={new_ticket.assigned_to.clone()}
                            onchange={{
                                let new_ticket = new_ticket.clone();
                                Callback::from(move |e: Event| {
                                    new_ticket.set(NewTicket {
                                        title: new_ticket.title.clone(),
                                        description: new_ticket.description.clone(),
                                        status: new_ticket.status.clone(),
                                        assigned_to: e.target_dyn_into::<HtmlSelectElement>().unwrap().value(),
                                        team_id: new_ticket.team_id.clone(),
                                    });
                                })
                            }}
                        >
                            <option
                                default={true}
                                selected={new_ticket.assigned_to.clone() == String::from("")}
                            >{"Unassigned"}</option>
                            {context.users.clone().into_iter().map(|user| {
                                html! {
                                    <option
                                        value={user.id.clone()}
                                        selected={user.id.clone() == new_ticket.assigned_to.clone()}
                                    >{user.name.clone()}</option>
                                }
                            }).collect::<Html>()}
                        </select>
                    </div>
                </div>
                <div class="new-ticket-action-group">
                    <button onclick={reset_ticket}>{"Reset"}</button>
                    <button onclick={save_ticket}>{"Save"}</button>
                </div>
            </StyledCardCreateModal>
        </Modal>
    }
}
