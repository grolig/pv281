use yew::{classes, function_component, html, Children, Html, Properties};

// /// Create a container that can accept children
// /// - this is for educational purposes, as this example is trivial
// /// MARKUP
// /// ------
// <div class={"you should allow the container to accept classes"}>
//     {
//         "here go the children"
//     }
// </div>

#[derive(PartialEq, Properties)]
pub struct ContainerProps {
    pub children: Children,
    pub class: String,
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
