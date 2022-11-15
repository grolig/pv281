use gloo_net::http::Request;
use serde::Deserialize;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

use crate::components::{
    card::{Card, CardProps},
    container::Container,
};

#[derive(Clone, PartialEq, Deserialize)]
struct ImageCard {
    description: String,
    id: u32,
    image_url: String,
    title: String,
}

#[function_component(ServerPage)]
pub fn server_page() -> Html {
    let cards = use_state(|| vec![]);

    {
        let cards = cards.clone();
        use_effect_with_deps(
            move |_| {
                // create a JS Promise from the Rust's Future
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_cards: Vec<ImageCard> = Request::get("http://localhost:5000/cards")
                        .send()
                        .await
                        .unwrap() // get the response, will crash the app
                        .json()
                        .await
                        .unwrap(); // try to parse the json;
                                   // serde tries to deserialize the json to
                                   // vector of image cards

                    // set the cards
                    cards.set(fetched_cards);
                })
            },
            (),
        )
    }

    html! {
        <Container class={"container"}>
            <Container class={"card-container"}>
                {
                    cards.iter().map(|card_data| {
                        let props = CardProps {
                            description: card_data.description.clone(),
                            image_url: card_data.image_url.clone(),
                            title: card_data.title.clone(),
                        };

                        html! {
                            <Card key={card_data.id} ..props />
                        }
                    }).collect::<Html>()

                }
            </Container>
        </Container>
    }
}
