use stylist::{css, yew::styled_component};
use yew::{function_component, html, Html, Properties};
use yew_icons::{Icon, IconId};

use crate::{common::ComponentProps, Ticket};

use super::card::Card;

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub title: String,
    pub status: String,
    pub tickets: Vec<Ticket>,
}

#[styled_component(StyledBoard)]
fn styled_board(props: &ComponentProps) -> Html {
    html! {
        <div class={css!("
            display: flex;
            flex-direction: column;
            background-color: #f7f7f7;
            width: 20%;
            height: 80%;
            padding: 16px;
            border-radius: 10px;
            .board-header {
                display: flex;
                justify-content: space-between;
            }
            .board-header > span {
                cursor: pointer;
            }
            .board-title {
                font-family: sans-serif;
                font-size: 18px;
                color: #808080;
            }
            .card-container {
                height: 100%;
                width: 100%;
                border-radius: 10px;
                overflow: auto;
            }
        ")}>{props.children.clone()}</div>
    }
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    html! {
        <StyledBoard>
            <div class="board-header">
                <div class="board-title">{props.title.clone()}</div>
                <span>
                <Icon
                    icon_id={IconId::FontAwesomeSolidCirclePlus}
                    style={"height: 20px; width: 20px; fill: #008000;"}
                />
                </span>
            </div>
            <div class="card-container">
                {props.tickets.clone().into_iter().map(|ticket| {
                    html!{<Card ticket={ticket} />}
                }).collect::<Html>()}
            </div>
        </StyledBoard>
    }
}
