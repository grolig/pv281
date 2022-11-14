use yew::{classes, function_component, html, Html, Properties};

/// Create a card that displays an image, a title, a description, and a button
/// which can be toggled (liked/unliked)
/// MARKUP
/// ------
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
