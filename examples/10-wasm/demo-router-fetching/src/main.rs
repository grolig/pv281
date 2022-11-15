use yew::{function_component, html, props, Html, Renderer};

// import components
mod components;
mod views;

use components::header::{Header, HeaderProps};
use views::{home::HomePage, server::ServerPage};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Home,
    #[at("/server-data")]
    Fetched,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Fetched => html! { <ServerPage /> },
        _ => html! { <HomePage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let header_properties = props!(HeaderProps {
        image_url: "https://yew.rs/img/logo.png".to_string(),
        title: "New awesome app".to_string(),
    });

    html! {
        <BrowserRouter>
            <div class={"main-window"}>
                <Header  ..header_properties/>
                <Switch<Routes> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
