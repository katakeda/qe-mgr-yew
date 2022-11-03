use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ComponentProps {
    #[prop_or_default]
    pub children: Children,
}
