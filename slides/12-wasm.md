---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust WASM
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

1. Úvod do Webassembly
2. Yew
3. Stav aplikace
4. Funkce prohlížeče

---

# Webassembly

Jedná se o binární instrukční formát (mezijazyk) pro zásobníkový virtuální stroj. Čili jde o cílový kód pro jiné jazyky. Primárně je určený pro běh ve webovém prohlížeči současně s Javascriptem.

---

# Výhody

- rychlost (nižší nároky na CPU a paměť)
- paralelismus
- bezpečnost
- otevřenost
- laditelnost

---

# Nevýhody

- není ještě stále vhodný pro jazyky s VM
- může vniknout velká binárka, která se dlouho stahuje
- nejsou dostupné stejné možnosti jako v JS
- chybějící ekosystém
- delší doba vývoje aplikací

---

# Existující WASM aplikace

- Figma

---

# Aplikace využívající DOM

## Výhody
- ověřený a známý způsob vývoje
- lehký debugging
- dostupné knihovny

## Nevýhody
- potenciálně horší rychlost
- větší velikost aplikace
- omezené možnosti (hry, 2D, 3D aplikace)

---

# Aplikace postavená nad canvasem

## Výhody
- rychlost a responzivita
- menší velikost aplikace
- neomezené možnosti funkcí aplikace

## Nevýhody
- delší doba implementace
- chybějící ekosystém
- nutnost implementace základních prvků GUI
- těžší debugging

---

# Yew

Rustový WASM framework inspirovaný ELMem. Základem jsou komponenty. 

Má menší spotřebu CPU a paměti oproti klasickým JS aplikacím, ale rychlostí je jen o něco lepší než React.

Dokumentace a množství příkladů ještě není dostatečné. Komunita zatím není příliš velká.

---

# Pricipy z ELMu

<!-- _class: invert + plantuml -->

@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

Component(store, "Store", "Drží aplikační stav")
Component(view, "View", "Zobrazuje UI komponentu")
Component(update, "Update", "Zpráva obsahující událost")
Component(reducer, "Reducer", "Stavový automat, který vrací nový stav")

Rel_L(store, view, "vykreslení")
Rel_L(view, update, "událost/zaslání zprávy")
Rel_L(update, reducer, "přijímá ke zpracování")
Rel(reducer, store, "změní")

@enduml


---

# Závislosti

```toml
[package]
name = "yew-app"
version = "0.1.0"
edition = "2018"

[dependencies]
yew = "0.18"
```

---

# Zobrazení HTML

```rust
use yew::html;

html! {
    <div>
        <div data-key="abc"></div>
        <div class="parent">
            <span class="child" value="anything"></span>
            <label for="first-name">{ "First Name" }</label>
            <input type="text" id="first-name" value="placeholder" />
            <input type="checkbox" checked=true />
            <textarea value="write a story" />
            <select name="status">
                <option selected=true disabled=false value="">{ "Selected" }</option>
                <option selected=false disabled=true value="">{ "Unselected" }</option>
            </select>
        </div>
    </div>
}
```

---

# Fragmenty

```rust
use yew::html;

html! {
    <>
        <div></div>
        <p></p>
    </>
}
```

---

# Podmíněné vykreslení

```rust
use yew::html;

html! {
  <div>
    {
      if show_link {
        html! {
          <a href="https://example.com">{"Link"}</a>
        }
      } else {
        html! {}
      }
    }
  </div>
}
```

---

# Zpracování událostí

```rust
use yew::{html, Callback};

html! {
    <button onclick={Callback::from(|_| ())}>
    //      ^^^^^^^ event listener name
        { "Click me!" }
    </button>
};
```

---

# Důležité události

- onclick
- onchange
- onkeypress
- onblur
- ondrag, ondragstart, ondragover, ondragleave, ondrop

---

# Vytvoření komponenty

```rust
use yew::{html, Children, Component, Html, Properties};

#[derive(Properties, Clone)]
pub struct Props {
    pub id: String,
    pub children: Children,
}

pub struct Container(Props);
impl Component for Container {
    type Properties = Props;

    // ...

    fn view(&self) -> Html {
       html! {
           <div id=self.0.id.clone()>
               { self.0.children.clone() }
           </div>
       }
    }
}
```

---

# Použití komponenty

```rust
use yew::{html, props, Children};

let props = yew::props!(Container::Properties {
    id: "container-2",
    children: Children::default(),
});
html! {
    <Container with props>
        // props.children will be overwritten with this
        <span>{ "I am a child, as you can see" }</span>
    </Container>
}
```

---

# Atributy key a ref

Key má podobnou funkcionalitu jako v Reactu. Je to unikátní identifikátor položky v seznamu. Umožňuje efektivnější vykreslení/překreslení.

Ref využíváme pro manipulaci s elementem přímo na úrovni DOM. To se hodí v případech, kdy využíváme existující knihovnu v JS a chceme ovliňovat stejný element jak z RS tak JS.

---

# Klienstký routing

---

# Router

```rust
use yew_router::Switch;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/posts/{}"]
    Post(u64),
    #[to = "/posts/?page={}"]
    PostListPage(u64),
    #[to = "/posts/"]
    PostList,
    #[to = "/authors/{id}"]
    Author(u64),
    #[to = "/authors/"]
    AuthorList,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}
```

---

# Namapování na rendering

```rust
fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Post(id) => {
                html! { <Post seed=id /> }
            }
            AppRoute::PostListPage(page) => {
                html! { <PostList page=page.max(1) /> }
            }
            AppRoute::PostList => {
                html! { <PostList page=1 /> }
            }
            AppRoute::Author(id) => {
                html! { <Author seed=id /> }
            }
            AppRoute::AuthorList => {
                html! { <AuthorList /> }
            }
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
        }
    }
```

---

# Vykreslení v HTML

```rust
<AppRouter
    render=AppRouter::render(Self::switch)
    redirect=AppRouter::redirect(|route: Route| {
        AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
    })
/>
```

---

# Odkaz

```rust
<div class="navbar-start">
    <AppAnchor classes="navbar-item" route=AppRoute::Home>
        { "Home" }
    </AppAnchor>
    <AppAnchor classes="navbar-item" route=AppRoute::PostList>
        { "Posts" }
    </AppAnchor>
    <div class="navbar-item has-dropdown is-hoverable">
        <a class="navbar-link">
            { "More" }
        </a>
        <div class="navbar-dropdown">
            <a class="navbar-item">
                <AppAnchor classes="navbar-item" route=AppRoute::AuthorList>
                    { "Meet the authors" }
                </AppAnchor>
            </a>
        </div>    
    </div>
</div>

```

---

# Sdílený stav (agenti)

Umožňuje komunikaci a sdílení dat napříč komponentami nehledě na to, jak hluboko se musíme zanořit.

Agenti jsou založení na actor modelu. 

Agenti běží paralelně. Každý ve svém web workeru.

---

# Sdílený stav (agenti)

```rust
use std::collections::HashMap;

pub type PostId = u32;

#[derive(Debug)]
pub enum Request {
    CreatePost(String),
    UpdatePost(PostId, String),
    RemovePost(PostId),
}

#[derive(Debug)]
pub enum Action {
    SetPost(Option<PostId>, String),
    RemovePost(PostId),
}

pub struct PostStore {
    pub posts: HashMap<PostId, String>,

    // Stores can have private state too
    id_counter: PostId,
}
```

---

# Sdílený stav (agenti)

```rust
use std::collections::HashMap;
use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

impl Store for PostStore {
    type Action = Action;
    type Input = Request;

    fn new() -> Self {
        let mut posts = HashMap::new();

        posts.insert(0, "Magic first post".to_owned());

        PostStore {
            posts,
            id_counter: 1,
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Request::CreatePost(text) => {
                link.send_message(Action::SetPost(None, text));
            }
            Request::UpdatePost(id, text) => {
                link.send_message(Action::SetPost(Some(id), text));
            }
            Request::RemovePost(id) => {
                link.send_message(Action::RemovePost(id));
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetPost(id, text) => {
                let id = id.unwrap_or_else(|| self.next_id());
                self.posts.insert(id, text);
            }
            Action::RemovePost(id) => {
                self.posts.remove(&id);
            }
        }
    }
}
```

---

# Použití store

```rust
pub enum Msg {
    UpdateText(String),
    Delete,
    PostStoreMsg(ReadOnly<PostStore>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: PostId,
}

pub struct Post {
    link: ComponentLink<Self>,
    id: PostId,
    text: Option<String>,
    post_store: Box<dyn Bridge<StoreWrapper<PostStore>>>,
}

```

---

# Použití store

```rust
impl Component for Post {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::PostStoreMsg);
        Self {
            link,
            id: props.id,
            text: None,
            post_store: PostStore::bridge(callback),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateText(text) => {
                self.post_store.send(Request::UpdatePost(self.id, text));
                false
            }
            Msg::Delete => {
                self.post_store.send(Request::RemovePost(self.id));
                false
            }
            Msg::PostStoreMsg(state) => {
                let state = state.borrow();

                // Only update if the post changed.
                if let Some(text) = state.posts.get(&self.id) {
                    self.text.neq_assign(Some(text.clone()))
                } else {
                    false
                }
            }
        }
    }
}
```

---

# Navázání události na store

```rust
impl Component for Post {
    // ...

    fn view(&self) -> Html {
        let text = self.text.as_deref().unwrap_or("<pending>");

        html! {
            <div>
                <h2>{ format!("Post #{}", self.id) }</h2>
                <p>{text}</p>

                <TextInput value=text.to_owned() onsubmit=self.link.callback(Msg::UpdateText) />
                <button onclick=self.link.callback(|_| Msg::Delete)>
                    { "Delete" }
                </button>
            </div>
        }
    }
}
```

---

# Práce s konzolí (gloo)

```rust
use gloo_console::log;
let object = JsValue::from("any JsValue can be logged");
log!("text", object)
```

---

# Local storage (gloo)

```rust
use gloo_storage::LocalStorage;

let key = "key";
let value = "value";
LocalStorage::set(key, value).unwrap();

let obtained_value: String = LocalStorage::get(key).unwrap();

#[derive(Deserialize)]
struct Data {
    key1: String,
    key2: String,
}

let data: Data = LocalStorage::get_all().unwrap();
```

---

# Časovače (gloo)

```rust
use gloo_timers::callback::Timeout;

let timeout = Timeout::new(1_000, move || {
    // Do something after the one second timeout is up!
});

// Since we don't plan on cancelling the timeout, call `forget`.
timeout.forget();
```

---

# WASM klient (Reqwasm)

```rust
use reqwasm::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

let mut ws = WebSocket::open("wss://echo.websocket.org").unwrap();
let (mut write, mut read) = ws.split();

spawn_local(async move {
    write.send(Message::Text(String::from("test"))).await.unwrap();
    write.send(Message::Text(String::from("test 2"))).await.unwrap();
});

spawn_local(async move {
    while let Some(msg) = read.next().await {
        console_log!(format!("1. {:?}", msg))
    }
    console_log!("WebSocket Closed")
})
```

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost
