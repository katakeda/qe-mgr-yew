use super::modal::Modal;
use crate::{common::ComponentProps, AppContext, Ticket};
use gloo_net::http::Request;
use serde_json::json;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement, HtmlTextAreaElement};
use yew::{
    function_component, html, use_context, use_state, Callback, Html, Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct CardEditModalProps {
    pub ticket: Ticket,
    pub close: Callback<()>,
}

async fn update_ticket(id: String, key: String, value: String) -> Vec<Ticket> {
    let mut tickets = vec![];
    let response = Request::put(&format!("/api/tickets/{}", id))
        .json(&json!({
            key: value,
        }))
        .unwrap()
        .send()
        .await;
    if let Ok(r) = response {
        tickets = r.json::<Vec<Ticket>>().await.unwrap_or(tickets);
    }
    tickets
}

#[styled_component(StyledCardEditModal)]
fn styled_card_edit_modal(props: &ComponentProps) -> Html {
    html! {
        <div class={css!("
            display: flex;
            flex-direction: column;
            margin-top: 40px;
            width: 100%;
            padding: 16px;
            .card-detail-group {
                display: flex;
                justify-content: space-between;
                margin-top: 20px;
            }
            .card-detail-group > span {
                width: 20%;
            }
            .card-detail-label-group {
                display: flex;
                flex-grow: 1;
                margin-right: 5px;
            }
            .card-detail-title span {
                font-size: 22px;
            }
            .card-detail-title input {
                width: 100%;
            }
            .card-detail-desc textarea {
                margin-left: 3px;
                width: 100%;
                height: 65px;
                font-family: sans-serif;
                resize: none;
            }
            .card-detail-action-group {
                display: flex;
                justify-content: end;
                margin-top: 20px;
            }
            .card-detail-action-group > button {
                background-color: #f44336;
                color: #fff;
                border-radius: 3px;
                border: none;
                padding: 4px 6px 3px;
            }
            button {
                cursor: pointer;
            }
        ")}>{props.children.clone()}</div>
    }
}

#[function_component(CardEditModal)]
pub fn card_edit_modal(props: &CardEditModalProps) -> Html {
    let context = use_context::<AppContext>().expect("no ctx found");
    let editing_title = use_state(|| false);
    let editing_description = use_state(|| false);
    let editing_assigned_to = use_state(|| false);
    let editing_status = use_state(|| false);
    let updated_title = use_state(|| props.ticket.title.clone());
    let updated_description = use_state(|| props.ticket.description.clone());
    let updated_assigned_to = use_state(|| match props.ticket.assigned_to.clone() {
        Some(assigned_to) => assigned_to.id,
        None => String::from(""),
    });

    let save_ticket = {
        let editing_title = editing_title.clone();
        let editing_description = editing_description.clone();
        let editing_assigned_to = editing_assigned_to.clone();
        let updated_title = updated_title.clone();
        let updated_description = updated_description.clone();
        let updated_assigned_to = updated_assigned_to.clone();
        let id = props.ticket.id.clone();
        let update_tickets = context.update_tickets.clone();
        Callback::from(move |field: String| {
            let id = id.clone();
            let update_tickets = update_tickets.clone();
            let mut state = editing_title.clone();
            let mut value = (*updated_title).clone();
            if field == String::from("description") {
                state = editing_description.clone();
                value = (*updated_description).clone();
            }
            if field == String::from("assigned_to") {
                state = editing_assigned_to.clone();
                value = (*updated_assigned_to).clone();
            }
            spawn_local(async move {
                update_ticket(id, field, value).await;
                state.set(false);
                update_tickets.emit(());
            });
        })
    };

    let delete_ticket = {
        let editing_title = editing_title.clone();
        Callback::from(move |_| {
            editing_title.set(false);
        })
    };

    html! {
        <Modal close={props.close.clone()}>
            <StyledCardEditModal>
                <div class="card-detail-group card-detail-title">
                    {if *editing_title {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    <input
                                        type="text"
                                        value={(*updated_title).clone()}
                                        onchange={{
                                            let updated_title = updated_title.clone();
                                            Callback::from(move |e: Event| {
                                                updated_title.set(
                                                    e.target_dyn_into::<HtmlInputElement>()
                                                        .unwrap()
                                                        .value()
                                                );
                                            })
                                        }}
                                    />
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={{
                                        let editing_title = editing_title.clone();
                                        Callback::from(move |_| {
                                            editing_title.set(false);
                                        })
                                    }}>{"Cancel"}</button>
                                    <button onclick={{
                                        let save_ticket = save_ticket.clone();
                                        Callback::from(move |_| {
                                            save_ticket.emit(String::from("title"));
                                        })
                                    }}>{"Save"}</button>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    <span>{props.ticket.title.clone()}</span>
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={{
                                        let editing_title = editing_title.clone();
                                        Callback::from(move |_| {
                                            editing_title.set(true);
                                        })
                                    }}>{"Edit"}</button>
                                </div>
                            </>
                        }
                    }}
                </div>
                <div class="card-detail-group card-detail-desc">
                    <span>{"Description: "}</span>
                    {if *editing_description {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    <textarea
                                        value={(*updated_description).clone()}
                                        onchange={{
                                            let updated_description = updated_description.clone();
                                            Callback::from(move |e: Event| {
                                                updated_description.set(
                                                    e.target_dyn_into::<HtmlTextAreaElement>()
                                                        .unwrap()
                                                        .value()
                                                );
                                            })
                                        }}
                                    />
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={{
                                        let editing_description = editing_description.clone();
                                        Callback::from(move |_| {
                                            editing_description.set(false);
                                        })
                                    }}>{"Cancel"}</button>
                                    <button onclick={{
                                        let save_ticket = save_ticket.clone();
                                        Callback::from(move |_| {
                                            save_ticket.emit(String::from("description"));
                                        })
                                    }}>{"Save"}</button>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    <span>{props.ticket.description.clone()}</span>
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={{
                                        let editing_description = editing_description.clone();
                                        Callback::from(move |_| {
                                            editing_description.set(true);
                                        })
                                    }}>{"Edit"}</button>
                                </div>
                            </>
                        }
                    }}
                </div>
                <div class="card-detail-group">
                    <span>{"Assigned To: "}</span>
                    {if *editing_assigned_to {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    <select value={(*updated_assigned_to).clone()}>
                                        <option
                                            default={true}
                                            selected={true}
                                            value={None::<String>}
                                        >{"Unassigned"}</option>
                                        {context.users.clone().into_iter().map(|user| {
                                            html! {
                                                <option
                                                    value={user.id.clone()}
                                                    selected={(*updated_assigned_to).clone() == user.id.clone()}
                                                >{user.name}</option>
                                            }
                                        }).collect::<Html>()}
                                    </select>
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={{
                                        let editing_assigned_to = editing_assigned_to.clone();
                                        Callback::from(move |_| {
                                            editing_assigned_to.set(false);
                                        })
                                    }}>{"Cancel"}</button>
                                    <button onclick={{
                                        let save_ticket = save_ticket.clone();
                                        Callback::from(move |_| {
                                            save_ticket.emit(String::from("assigned_to"));
                                        })
                                    }}>{"Save"}</button
                                    >
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <div class="card-detail-label-group">
                                    {if let Some(assigned_to) = props.ticket.assigned_to.clone() {
                                        html!(<span>{assigned_to.name}</span>)
                                    } else {
                                        html!(<span>{"Unassigned"}</span>)
                                    }}
                                </div>
                                <div class="card-detail-button-group">
                                    <button onclick={{
                                        let editing_assigned_to = editing_assigned_to.clone();
                                        Callback::from(move |_| {
                                            editing_assigned_to.set(true);
                                        })
                                    }}>{"Edit"}</button>
                                </div>
                            </>
                        }
                    }}
                </div>
                // <div class="card-detail-group">
                //     <span>Status: </span>
                //     {#if editingStatus}
                //     <div class="card-detail-label-group">
                //         <select bind:value={updatedTicket.status}>
                //         {#each statuses as status}
                //             <option
                //             value={status.value}
                //             selected={status.value == updatedTicket.status}
                //             >{status.label}</option
                //             >
                //         {/each}
                //         </select>
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button
                //         on:click={() => {
                //             editingStatus = false;
                //             resetTicket('status');
                //         }}>Cancel</button
                //         >
                //         <button
                //         on:click={() => {
                //             editingStatus = false;
                //             saveTicket({ status: updatedTicket.status });
                //         }}>Save</button
                //         >
                //     </div>
                //     {:else}
                //     <div class="card-detail-label-group">
                //         <span>{statuses.find((s) => s.value === ticket.status)?.label}</span>
                //     </div>
                //     <div class="card-detail-button-group">
                //         <button on:click={() => (editingStatus = true)}>Edit</button>
                //     </div>
                //     {/if}
                // </div>
                <div class="card-detail-action-group">
                    <button onclick={delete_ticket}>{"Delete"}</button>
                </div>
            </StyledCardEditModal>
        </Modal>
    }
}
