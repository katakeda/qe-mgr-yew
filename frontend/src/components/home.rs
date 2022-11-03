use super::board::Board;
use crate::{common::ComponentProps, AppContext, Ticket};
use stylist::{css, yew::styled_component};
use yew::{function_component, html, use_context};

#[styled_component(StyledHome)]
fn styled_home(props: &ComponentProps) -> Html {
    html! {
        <div class={css!("
            height: calc(100vh - 85px);
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin: 0 48px;
        ")}>{props.children.clone()}</div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let context = use_context::<AppContext>().expect("no ctx found");
    let tickets = context.tickets.clone();
    let new_tickets = tickets
        .iter()
        .filter(|t| t.status == "New")
        .map(|t| t.clone())
        .collect::<Vec<Ticket>>();
    let pending_tickets = tickets
        .iter()
        .filter(|t| t.status == "Pending")
        .map(|t| t.clone())
        .collect::<Vec<Ticket>>();
    let accepted_tickets = tickets
        .iter()
        .filter(|t| t.status == "Complete")
        .map(|t| t.clone())
        .collect::<Vec<Ticket>>();
    let rejected_tickets = tickets
        .iter()
        .filter(|t| t.status == "Rejected")
        .map(|t| t.clone())
        .collect::<Vec<Ticket>>();

    html! {
        <StyledHome>
            <Board title={"Ready For Review"} status={"New"} tickets={new_tickets} />
            <Board title={"In Review"} status={"Pending"} tickets={pending_tickets} />
            <Board title={"Accepted"} status={"Complete"} tickets={accepted_tickets} />
            <Board title={"Rejected"} status={"Rejected"} tickets={rejected_tickets} />
        </StyledHome>
    }
}
