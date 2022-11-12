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

# Today lecture's content

1. Introduction to Webassembly
2. Yew
3. Stav aplikace
4. Funkce prohlížeče

---

# <!--fit--> Introduction to Webassembly

---

# Webassembly

- Binary instruction format for (virtual) state machine with a stack.
- Compliers (for lanuages such as C, C++, Rust) can target this intermediate language.
- The instructions are then interpreted by the web browser. WASM can run alongside existing JavaScript.

---

<!-- Imagine a nice picture -->

---

# WASM advantages

- Speed (only executing instructions rather than interpreting them)
- Smaller memory usage footprint\* than JS
- Paralelism (we can write multithreaded code in the browser)
- Memory Safety (only when )
- Debugging

\*only when used with non-GC languages

---

# WASM Disadvantages

- VM-based machines have to bundle the VM along with the app
- The resulting WASM binary file can grow very large (impacts network performance)
- Still cannot do everything JS can, in some cases the reliance on JS code will be inevitable
- The immature ecosystem
- Development time & cost (compilation, using a more strict language, also directly ties to previous point)

---

# Existing WASM apps

- Figma
- CAD @TODO FIND THE CORRECT LINK
- GAMES @TODO FIND SUCH GAMES
- egui @TODO MORE INFO
- A bunch of [hobby projects]() @TODO LINK

---

# Apps using Canvas

---

<!-- Image of such app -->

---

# Advantages of canvas-based apps

- Complete control over the look and feel of components
- Smooth user experience (when done right) and absolute control over responsivity
- Smaller app size
- Unlimited control over what is rendered - allows easy development of 2D / 3D apps

---

# Disadvantages of canvas-based WASM apps

- Often implementation from scratch
- When using a library, it's very hard to modify elements
- The ecosystem is small
- The debugging is hard

---

# Apps using DOM (HTML, CSS)

---

<!-- Image of such apps (yew based) -->

---

# Advantages of DOM-based WASM apps

- Used and tested model of UI design
- Complete rewrite from JS based project is relatively straight forward
- Easy debugging
- Vast ecosystem of libraries and solutions (most involve JS, joining WASM with JS is possible)

---

# Disadvantages of using DOM-based WASM apps

- Potentially slower than canvas-based apps
- The bundle size is larger (need to ship HTML & CSS & WASM code)
- Limited options with the design - 2D / 3D apps and games are way harder to implement

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

Ref využíváme pro manipulaci s elementem přímo na úrovni DOM. To se hodí v případech, kdy využíváme existující knihovnu v JS a chceme ovliňovat stejný element jak z WASM tak JS.

---

# Callbacky

```rust
pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value += 1;
                ConsoleService::log("plus one");
                true
            }
            Msg::Decrement => {
                self.value -= 1;
                ConsoleService::log("minus one");
                true
            }
        }
    }
}
```

---

# Callbacky

```rust
pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::Increment)>
                        { "Increment" }
                    </button>
                    <button onclick=self.link.callback(|_| Msg::Decrement)>
                        { "Decrement" }
                    </button>
                    <button onclick=self.link.batch_callback(|_| vec![Msg::Increment, Msg::Increment])>
                        { "Increment Twice" }
                    </button>
                </nav>
                <p>
                    <b>{ "Current value: " }</b>
                    { self.value }
                </p>
                <p>
                    <b>{ "Rendered at: " }</b>
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}
```

--- 

# Inicializace aplikace

```rust
fn main() {
    yew::start_app::<Model>();
}
```

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

# WebSocket klient (Reqwasm)

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

# WebGL

Plusy
- skvělá rychost
- neomezené možnosti

Mínusy
- moc práce na vytvoření aplikace
- podpora prohlížečů

---

# WebGL

```rust
impl Model {
    fn render_gl(&mut self, timestamp: f64, link: &Scope<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        // This list of vertices will draw two triangles to cover the entire canvas.
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        gl.draw_arrays(GL::TRIANGLES, 0, 6);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}
```

---

# <!--fit--> Any questions?

---

# <!--fit--> Thank you for your attention!
