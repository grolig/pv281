use yew::{classes, function_component, html, Html, Properties};

use crate::components::like_button::LikeButton;

// /// Create a card that displays an image, a title, a description, and a button
// /// which can be toggled (liked/unliked)
// /// MARKUP
// /// ------
// <section class={"card"}>
//     <div class={"card__image-container"}>
//         <img class={"image"} src={ "url goes here" } />
//     </div>
//     <div class={"card__content"}>
//         <h2 class={classes!("card__title", "heading")}> { "title goes here" } </h2>
//         <p class={"card__description"}>{ "description goes here" }</p>
//         { "like button goes here" }
//     </div>
// </section>

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub image_url: String,
    pub description: String,
    pub title: String,
}

#[function_component(Card)]
pub fn component(props: &CardProps) -> Html {
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
