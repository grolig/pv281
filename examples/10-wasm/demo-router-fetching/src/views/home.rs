use crate::{components::container::Container, Routes};
use yew::{classes, function_component, html, Callback, Html};
use yew_router::prelude::use_navigator;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Routes::Fetched));

    html! {
        <Container class={"container"}>
            <h1 {onclick} class={classes!("heading", "clickable")}>{"Go to fetched data!"}</h1>
        </Container>
    }
}
