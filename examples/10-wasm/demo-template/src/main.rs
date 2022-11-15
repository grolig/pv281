use yew::{classes, function_component, html, props, Html, Renderer};

// import components
mod components;

use components::{
    card::{Card, CardProps},
    container::Container,
    header::{Header, HeaderProps},
};

struct ImageCard {
    id: u32,
    image_url: String,
    description: String,
    title: String,
}

// create structure that will hold the card data

#[function_component(App)]
fn app() -> Html {
    // create header properties
    let header_props = props!(HeaderProps {
        image_url: "https://store.storeimages.cdn-apple.com/4668/as-images.apple.com/is/iphone-13-model-unselect-gallery-1-202207?wid=5120&hei=2880&fmt=p-jpg&qlt=80&.v=1654893619853".to_string(),
        title: "Je to eshop".to_string(),
    });

    let cards: Vec<ImageCard> = vec![
        ImageCard {
            id: 1,
            image_url: "https://images.samsung.com/cz/smartphones/galaxy-z-fold3-5g/buy/02_Carousel/02_Feature/ZFold3_Carousel_FeatureKV_ArmorAluminum_MO.jpg".to_string(),
            title: "Galaxy".to_string(),
            description: "Yayyy".to_string(),
        },
        ImageCard {
            id: 2,
            image_url: "https://images.samsung.com/cz/smartphones/galaxy-z-fold3-5g/buy/02_Carousel/02_Feature/ZFold3_Carousel_FeatureKV_ArmorAluminum_MO.jpg".to_string(),
            title: "Another galaxy".to_string(),
            description: "Shall I go into detail?".to_string(),
        }
    ];

    // create a list of card data

    html! {
        <div class={"main-window"}>
            <Header ..header_props />
            <Container class={classes!("container", "special-container")}>
                <Container class={"card-container"}>
                    {
                        cards.into_iter().map(|card_data| {
                            let props = props!(CardProps {
                                title: card_data.title.clone(),
                                description: card_data.description.clone(),
                                image_url: card_data.image_url.clone(),
                            });

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
    // hook up the WASM rendering
    Renderer::<App>::new().render();
}
