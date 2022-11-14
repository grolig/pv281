use yew::{classes, function_component, html, Html, Properties};

/// Create a header for the app with the following markup:
/// MARKUP
/// ------
// <div class={classes!("header")}>
//     <div class={"header__logo-container"}>
//         <img src={ "something goes here" } class={"image"} />
//     </div>
//     <h1 class={classes!("header__text", "heading")}>
//         { "something goes here" }
//     </h1>
// </div>
/// ------
/// Header displays the title and the image from provided url