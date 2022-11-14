use yew::{function_component, html, use_state, Callback, Html};

// /// Create a button with the following markup:
// /// MARKUP
// /// ------
// <div {onclick} class={"card__button"}>{
//     "stateful data here!"
// }</div>
// /// ------
// /// The button will respond to clicks by visually modifying itself

#[function_component(LikeButton)]
pub fn like_button() -> Html {
    let liked = use_state(|| false);
    let onclick = {
        let liked = liked.clone();

        Callback::from(move |_| liked.set(!*liked))
    };

    html! {
        <div {onclick} class={"card__button"}>
            {
                if !*liked {
                    "Like!"
                } else {
                    "Unlike."
                }
            }
        </div>
    }
}
