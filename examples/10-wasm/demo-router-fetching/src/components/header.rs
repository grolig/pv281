use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub image_url: String,
    pub title: String,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <div class={classes!("header")}>
            <div class={"header__logo-container"}>
                <img src={props.image_url.clone()} class={"image"} />
            </div>
            <h1 class={classes!("header__text", "heading")}>
                { props.title.clone() }
            </h1>
        </div>
    }
}
