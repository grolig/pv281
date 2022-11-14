use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ComponentProps {}

#[function_component(Component)]
pub fn component(props: &ComponentProps) -> Html {
    html! {}
}
