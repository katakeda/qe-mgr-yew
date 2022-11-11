use crate::{common::ComponentProps, AppContext};
use stylist::yew::styled_component;
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent};
use yew::{
    function_component, html, use_context, use_effect_with_deps, use_state, Callback, Html,
    TargetCast,
};

#[styled_component(StyledHeader)]
fn styled_header(props: &ComponentProps) -> Html {
    html! {
        <header class={css!("
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 24px 48px;
            background-color: #fff;
            .header-item {
                display: flex;
                justify-content: space-evenly;
            }
            .header-item > img {
                height: 36px;
            }
            .header-title {
                font-family: sans-serif;
                font-size: 32px;
                margin-left: 8px;
                color: #808080;
            }
            .header-team-switcher {
                border: 1px solid #d3d3d3;
                border-radius: 3px;
                display: flex;
                align-items: center;
            }
            .header-team-switcher > input {
                border: none;
                outline: none;
                padding: 9px;
            }
        ")}>{props.children.clone()}</header>
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    let context = use_context::<AppContext>().expect("no ctx found");
    let teams = context.teams.clone();

    let filtered_teams = use_state(|| vec![]);
    {
        let filtered_teams = filtered_teams.clone();
        use_effect_with_deps(
            move |teams| {
                filtered_teams.set(teams.clone());
                || ()
            },
            teams.clone(),
        );
    }

    let filter_teams = {
        let teams = teams.clone();
        let filtered_teams = filtered_teams.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if target.value().trim().is_empty() {
                    filtered_teams.set(teams.clone());
                } else {
                    filtered_teams.set(
                        teams
                            .iter()
                            .filter(|t| t.name.contains(&*target.value().clone()))
                            .map(|t| t.clone())
                            .collect(),
                    );
                }
            }
        })
    };

    let submit_team = {
        let teams = teams.clone();
        Callback::from(move |e: KeyboardEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlInputElement>() {
                if e.key() == "Enter" {
                    let team = target.value();
                    if let Some(_) = teams.iter().find(|t| t.name == team) {
                        context.update_current_team.emit(team);
                    }
                }
            }
        })
    };

    html! {
        <StyledHeader>
            <div class="header-item">
                <img class="header-logo" src={"assets/yew.png"} alt="logo" />
                <span class="header-title">{"QE Manager"}</span>
            </div>
            <div class="header-item">
                <div class="header-team-switcher">
                    <input
                        list="teams"
                        placeholder="Select Team"
                        oninput={filter_teams}
                        onkeypress={submit_team}
                    />
                    <datalist id="teams">
                        {(&*filtered_teams).into_iter().map(|team| {
                            html!{<option>{team.name.clone()}</option>}
                        }).collect::<Html>()}
                    </datalist>
                </div>
            </div>
        </StyledHeader>
    }
}
