use super::like_button::LikeButton;
use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub description: String,
    pub image_url: String,
    pub title: String,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <section class={"card"}>
            <div class={"card__image-container"}>
                <img class={"image"} src={ props.image_url.clone() } />
            </div>
            <div class={"card__content"}>
                <h2 class={classes!("card__title", "heading")}> { props.title.clone() } </h2>
                <p class={"card__description"}>{ props.description.clone() }</p>
                <LikeButton />
            </div>
        </section>
    }
}
