use yew::{function_component, html, props, Html, Renderer};

// import components
mod components;

// create structure that will hold the card data

#[function_component(App)]
fn app() -> Html {
    // create header properties

    // create a list of card data

    html! {
        <div class={"main-window"}>
            {
                // add header here
            }
            {
                // create a container for the rest of the page
                // with class "container"
                // create a container within that container that will
                // hold card data with class "card-container"
                // iterate through the data and return each keyed element
            }
        </div>
    }
}

fn main() {
    // hook up the WASM rendering
    Renderer::<App>::new().render();
}
