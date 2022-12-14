use super::card_edit_modal::CardEditModal;
use crate::{common::ComponentProps, Ticket};
use stylist::{css, yew::styled_component};
use yew::{function_component, html, use_state, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub ticket: Ticket,
}

#[styled_component(StyledCard)]
fn styled_card(props: &ComponentProps) -> Html {
    html! {
        <div class={css!("
            min-height: 80px;
            box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1),
              0 4px 6px -4px rgb(0 0 0 / 0.1);
            margin-top: 16px;
            cursor: pointer;
            background-color: #fff;
            border-radius: 3px;
            display: flex;
            justify-content: space-between;
            .card:hover {
                box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1),
                0 8px 10px -6px rgb(0 0 0 / 0.1);
            }
            .card-detail {
                padding: 10px;
                font-family: sans-serif;
                font-size: 14px;
                display: flex;
                flex-direction: column;
                justify-content: space-between;
                color: #4c4e52;
            }
            .card-ctrls {
                display: flex;
                flex-direction: column;
                justify-content: space-evenly;
                align-items: center;
                padding: 0 10px;
            }
        ")}>{props.children.clone()}</div>
    }
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let show_edit_modal = use_state(|| false);
    let open_edit_modal = {
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |_| {
            show_edit_modal.set(true);
        })
    };
    let close_edit_modal = {
        let show_edit_modal = show_edit_modal.clone();
        Callback::from(move |_| {
            show_edit_modal.set(false);
        })
    };

    html! {
        <StyledCard>
            <div class="card-detail" onclick={open_edit_modal}>
                <div class="card-title">{props.ticket.title.clone()}</div>
                <div class="card-description">{props.ticket.description.clone()}</div>
                <div class="card-assigned-to">
                    {match props.ticket.assigned_to.clone() {
                        Some(u) => u.name,
                        None => "Unassigned".into(),
                    }}
                </div>
            </div>
            {if *show_edit_modal {
                html! {
                    <CardEditModal ticket={props.ticket.clone()} close={close_edit_modal} />
                }
            } else {
                html!()
            }}
        </StyledCard>
    }
}
