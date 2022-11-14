use yew::{function_component, html, use_state, Callback, Html};

#[function_component(LikeButton)]
pub fn like_button() -> Html {
    let liked = use_state(|| false);
    let onclick = {
        let liked = liked.clone();
        Callback::from(move |_| liked.set(!*liked))
    };

    html! {
        <div {onclick} class={"card__button"}>{
            if *liked {
                "Unlike"
            } else {
                "Like"
            }
        }</div>
    }
}
