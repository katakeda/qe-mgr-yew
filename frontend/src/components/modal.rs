use crate::common::ComponentProps;
use stylist::yew::styled_component;
use web_sys::MouseEvent;
use yew::{create_portal, function_component, html, Callback, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    #[prop_or_default]
    pub children: Children,
    pub close: Callback<()>,
}

#[styled_component(StyledModal)]
fn styled_modal(props: &ModalProps) -> Html {
    let close_modal = {
        let close = props.close.clone();
        Callback::from(move |_| {
            close.emit(());
        })
    };

    html! {
        <div class={css!("
            position: absolute;
            top: 0;
            display: flex;
            justify-content: center;
            align-items: center;
            width: 100%;
            height: 100%;
            background-color: rgba(0, 0, 0, 0.5);
        ")} onclick={close_modal}>{props.children.clone()}</div>
    }
}

#[styled_component(StyledModalInner)]
fn styled_modal_inner(props: &ComponentProps) -> Html {
    html! {
        <div class={css!("
            display: flex;
            height: 350px;
            width: 650px;
            max-width: 100%;
            max-height: 100%;
            border-radius: 5px;
            background-color: #ffffff;
        ")} onclick={|e: MouseEvent| e.stop_propagation()}>{props.children.clone()}</div>
    }
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let host = gloo_utils::document()
        .get_element_by_id("root")
        .expect("root element not found");

    create_portal(
        html! {
            <StyledModal close={props.close.clone()}>
                <StyledModalInner>{props.children.clone()}</StyledModalInner>
            </StyledModal>
        },
        host.into(),
    )
}
