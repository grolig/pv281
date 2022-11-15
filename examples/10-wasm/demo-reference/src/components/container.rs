use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ContainerProps {
    pub class: String,
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    html! {
        <div class={props.class.clone()}>
            {
                for props.children.iter()
            }
        </div>
    }
}
