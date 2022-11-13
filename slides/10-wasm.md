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
3. Application state
4. Browser API support in WASM

---

# <!--fit--> Introduction to Webassembly

---

# Webassembly

- Binary instruction format for a (virtual) stack-based state machine.
- Compliers (for lanuages such as C, C++, Rust) can target this intermediate language.
- The instructions are then interpreted by the web browser. WASM can run alongside existing JavaScript.
- Code runs in a sandboxed environment.
- For more information, see [this github page](https://webassembly.github.io/spec/core/index.html).

---


<!-- _class: split -->

### WASM

<div class=left-column>

#### C code

```c
int factorial(int n) {
  if (n == 0)
    return 1;
  else
    return n * factorial(n-1);
}
```

</div>
<div class=right-column>

#### WASM text output

```wat
(func (param i64) (result i64)
  local.get 0
  i64.eqz
  if (result i64)
      i64.const 1
  else
      local.get 0
      local.get 0
      i64.const 1
      i64.sub
      call 0
      i64.mul
  end)

```

</div>

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

- [Figma](https://www.figma.com/)
- [AutoCAD Web](https://web.autocad.com/login)
- [Google Earth](https://earth.google.com/web)
- Some games and 3D visualizers

---

# Things you should know about

- [Rust WASM book](): Understanding how the Rust code gets to the browser
- [WebGL](https://www.khronos.org/webgl/): 3D rendering in browser
- [wasmer](https://wasmer.io/) & [wasmtime](https://wasmtime.dev/): WASM runtimes (like `Node` or `Deno` for JS/TS projects)
- [trunk](https://trunkrs.dev/): Rust-generated WASM packager for browsers 
- [wasm-pack](https://rustwasm.github.io/wasm-pack/): Package Rust-generated WASM to run with JS code 
- [equi](https://github.com/emilk/egui): Uses WebGL & WASM to render in browser

---

# Apps using Canvas

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

# Yew

---

# Yew

Component-based Rust framework utilizing DOM, based on the principles of ELM. Utilizes Virtual DOM architecture.

Better performance and less CPU intense than JS-based solutions. Slightly more RAM usage.

Docs are somewhat disappointing, showing slow and steady improvement over the years.

---

# Performance

![h:550px](./assets/10-images/wasm-benchmark.png)

---

# ELM principles

- Functional paradigm
- State machines
- `ui = fn(state)`
- Pure functions, less mistakes

---

# ELM principles

<!-- _class: invert + plantuml -->

@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml


Component(store, "Store", "Holds app state")
Component(view, "View", "Renders the UI of the component")
Component(update, "Update", "Sends message with the event")
Component(reducer, "Reducer", "State automata, returns new state")

Rel_L(store, view, "render")
Rel_L(view, update, "(event) message passing")
Rel_L(update, reducer, "processes")
Rel(reducer, store, "changes")

@enduml

---

# Before we start

We need to add the WASM compilation target for Rust

```sh
rustup target add wasm32-unknown-unknown
```

---

# Before we start

We need to install [trunk](https://trunkrs.dev/) - tool for deployment and packaging Yew apps.

```sh
cargo install trunk
```

---

# Before we start - the reality of the framework

- Beta framework (current ver: `0.19`)
- Libraries pretty much non-existent
- Breaking changes between versions
- Non-existing tutorials for complex solutions
- Documentation of advanced features is lacking

---

# Before we start

Some slides deal with `next` version. These slides will be marked.

---

# Dependencies

```toml
# Current version! Some slides deal with "next" version.
[dependencies]
yew = "0.19"

# If we want to use the browser's api, we can use web-sys crate.
# There are also alternatives, we'll talk about them later. 
[dependencies.web-sys]
version = "0.3"
# We need to enable all the web-sys features we want to use!
features = [
    "console",
    # ...
]
```

---

# `html!` macro

```rust
use yew::html;

html! {
    <div>
        <div data-key="abc"></div>
        <div class="parent">
            <span class="child"></span>
            <textarea value="write a story" />
        </div>
    </div>
}
```

---

# `html!` macro - advanced

```rust
use yew::html;

let show_link = false;

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

# Fragments

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

# Components (v. `0.19`)

Components are Rust structs. They resemble class-based components in React. Every component needs to implement trait `Component` from Yew crate:

```rust
// these imports are used also on the following slides
use yew::{Component, Context, html, Html, Properties};

pub struct AmazingYewComponent;

impl Component for AmazingYewComponent {
    // ... implementation goes here
}
```

---


## Messages and Properties (v. `0.19`)

The component can have messages (events) that can happen within the component.

```rust
enum Msg {
    DoSomething,
}

#[derive(PartialEq, Properties)]
struct Props {
    some_property: String, // String is used just as the demo
}

impl Component for AmazingYewComponent {
    type Message = Msg;
    type Properties = Props;
    // the rest of the implementation ↓
}
```

---


## Messages and Properties (v. `0.19`)

Some components don't have props / events. Missing types are set to Rust's Unit type - `()`

```rust
enum HomePageMsg {
    DoSomething,
}

impl Component for HomePage {
    type Message = HomePageMsg;
    // there are no properties for this component
    type Properties = ();
    // the rest of the implementation ↓
}
```

---


# `create` method (v. `0.19`)

Every component has a create method. The element also has a context, which has the element's props and and `Scope`.

```rust
// imports and message / prop definitions ↑
impl Component for HomePage {
    // message / prop types here

    fn create(ctx: &Context<Self>) -> Self {
        HomePage // or you can use "Self" here as well
    }
}
```

---


# `view` method (v. `0.19`)

Every component has a view method. View takes the element's context - its scope and properties and renders the component to html.

```rust
impl Component for AmazingYewComponent {
    // message / prop types, create method here

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <p> { &ctx.props().some_property } </p>
        }
    }
}
```

---

# `update` method (v. `0.19`)

Consider this more advanced example. First, we have a component with an inner state (note: the imports are omitted):


```rust
enum ComponentWithStateMsg {
    SetShowText(bool),
}

pub struct ComponentWithState {
    show_text: bool;
}

impl Component for ComponentWithState {
    type Message = ComponentWithStateMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            show_text: false;
        }
    }

    // the rest of the implementation ↓
}
```

---

# `update` method (v. `0.19`)

```rust
impl Component for ComponentWithState {
    // the implementation continues:

    fn update(&mut self, ctx: &Context <Self>, msg: Self::Message) -> bool {
        match msg {
            ComponentWithStateMsg::SetShowText(set) => {
                if self.show_text != set {
                    self.show_text = set;
                    true // trigger re-rendering
                } else {
                    false // nothing should change
                }
            }
        }
    }
}
```

---

# `update` method (v. `0.19`)

```rust
impl Component for ComponentWithState {
    // types, create and update previously implemented

    fn view(&self, ctx: &Context <Self>) -> Html {
        let onclick = ctx.link().callback(|_| ComponentWithStateMsg::SetShowText(!self.show_text));

        if (show_text) {
            html! {
                <>
                    <button {onclick}> { "Hide text" } </button>
                    <p> { "Never gonna give you up" } </p>
                </>
            }
        } else {
            html! {
                <button {onclick}> { "Show text" } </button>
            }
        }
    }
}
```

---

# Other methods for components (v. `0.19`)

- `rendered`: Used to perform side effects after the component is rendered.
- `changed`: Whether the component needs to be re-rendered after its props change. Default implementation re-renders the component.
- `destroy`: On component unmount, there might be a need to clean up after the component.

By default, only the `create` and `view` method have to be implemented.

---

# Callbacks (v. `0.19`, similar in v. `next`)

Allow communication within the component, as well as with agents, services and parent components. This is how to handle interaction between components.

```rust
use yew::{html, Callback};

html! {
    <button onclick={Callback::from(|_| ())}> // instead of (), we could send some specific message
    //      ^^^^^^^ event listener name;
        { "Click me!" }
    </button>
};
```

---

# Important HTML events

- `onclick`
- `onchange`
- `onkeypress`
- `onblur`
- `ondrag`, `ondragstart`, `ondragover`, `ondragleave`, `ondrop`, ...

---

# Function components (v. `next`)

Check [this page](https://yew.rs/docs/next/concepts/function-components) for more in-depth explanation of preferred version to write Yew components in the `next` version.

```rust
use yew::{function_component, html, Html};

#[function_component]
fn HelloFI() -> Html {
    html! { <p> { "42 is a nice number" } </p> }
}

// to use it, we can call the component name within the html! macro
#[function_component]
fn App() -> Html {
    html! { <HelloFI /> }
}
```

---

# Function components (v. `next`)

Function components can utilize [pre-defined hooks](https://yew.rs/docs/concepts/function-components/pre-defined-hooks) and also [custom hooks](//yew.rs/docs/concepts/function-components/custom-hooks). Usage is very similar to React hooks.

```rust
use yew::{function_component, html, Html, use_state};

#[function_component]
fn HelloFI() -> Html {
    let counter = use_state(|| 42);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    }

    html! { <button {onclick}> { format!("{} is a nice number!", *counter) } </button> }
}
```

---

# Using the component (v `0.19`)

```rust
use yew::{html, props, Children};
// create properties with props! macro
let props = props!(Container::Properties {
    id: "container-2",
    children: Children::default(),
});
html! {
    <Container with props> // instead of manually passing props use "with" syntax
        // props.children will be overwritten with this
        <span>{ "I am a child, as you can see" }</span>
    </Container>
}
```

---

# Using the component (v. `next`)

```rust
use yew::{Properties, function_component, Html, html};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    if props.is_loading {
        html! { "Loading" }
    } else {
        html! { "Hello world" }
    }
}
```

---

# Using the component - properties (v. `next`)

```rust
// we use the HelloWorld component fromt the last slide
#[function_component]
fn App() -> Html {
    let hello_world_props = props! { // we can create props with props! macro
        HelloWorld::Properties {
            is_loading: true,
        }
    };

    html! {
        <HelloWorld ..hello_world_props>
            <p> { "Hello, I am a child of this component." } </p>
        </HelloWorld>
    }
}
```

---

# Attributes `key` and `ref`

`key` works similarly to keys in React. It helps Yew determine which elements need to be re-rendered by uniquely describing the element (when rendering lists of elements / components). 

`ref` is used to manipulate the DOM element directly. This is useful if we want to embed JS library to our WASM project and modify the DOM element from JS and WASM.

--- 

# App initialization (v. `0.19`)

```rust
fn main() {
    yew::start_app::<Model>();
}
```

---


# App initialization
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
