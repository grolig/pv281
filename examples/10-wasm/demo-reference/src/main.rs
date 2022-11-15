use yew::{classes, function_component, html, props, Html, Renderer};

// import components
mod components;

use components::{
    card::{Card, CardProps},
    container::Container,
    header::{Header, HeaderProps},
};

struct ImageCard {
    description: String,
    id: u32,
    image_url: String,
    title: String,
}

#[function_component(App)]
fn app() -> Html {
    let header_properties = props!(HeaderProps {
        image_url: "https://yew.rs/img/logo.png".to_string(),
        title: "New awesome app".to_string(),
    });

    let cards: Vec<ImageCard> = vec![
        ImageCard {
            id: 0,
            description: "No, I don't have one. :(".into(),
            image_url: "https://store.storeimages.cdn-apple.com/4668/as-images.apple.com/is/mbp-spacegray-select-202206_GEO_CZ?wid=904&hei=840&fmt=jpeg&qlt=90&.v=1664497359383".into(),
            title: "Macbook".into(),
        },
        ImageCard {
            id: 1,
            description: "No, I don't have that one either... :D".into(),
            image_url: "https://store.storeimages.cdn-apple.com/4668/as-images.apple.com/is/iphone-13-model-unselect-gallery-1-202207?wid=5120&hei=2880&fmt=p-jpg&qlt=80&.v=1654893619853".into(),
            title: "iPhone".into(),
        },
    ];

    html! {
        <div class={classes!("main-window")}>
            <Header  ..header_properties/>
            <Container class={"container"}>
                <Container class={"card-container"}>
                    {
                        cards.into_iter().map(|card_data| {
                            let props = CardProps {
                                description: card_data.description,
                                image_url: card_data.image_url,
                                title: card_data.title,
                            };

                            html! {
                                <Card key={card_data.id} ..props />
                            }
                        }).collect::<Html>()
                    }
                </Container>
            </Container>
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
