---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust Bonus
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

1. Deployment aplikací
2. OAuth, OpenID
3. Zero-Trust architektury
4. Makra

---

# <!--fit--> Deployment aplikací

---

# Produkční build

Produkční build spustíme příkazem

```bash
cargo build --release
```

Pro produkční build je vhodné nastavit úroveň optimalizací v Cargo.toml. Znamená to ovšem delší kompilační čas.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

---

# Nasazení webové služby

1. Na cílový server nahrajeme binárku aplikaci. 
2. Registrujeme jako službu/démona nebo jiným způsobem zařídíme spuštění po startu systému.
3. V případě potřeby nasadíme proxy.

---

# Nahrátí na cílový server

Záleží na možnostech, které máme. Pokud nasazujeme u cloud providera, tak využijeme jeho možností (zip package, teracopy). Pokud fungujeme na vlastním serveru, tak máme možnosti přes FTP, git apod.

---

# Kontejnery

Jako lepší scénář se dnes nabízí kontejnerizace. Pro menší deployment jde použít Docker Swarm. Lepší alternativou (ale vyšší učící křivkou) je Kubernetes. Pro jednoduché deploymenty existuje k3s v lokální kombinaci s nerdctl. 

Větší deploymenty jde realizovat ve vlastním k8s clusteru nebo využít cloud-hosted.

---

# CD pipeline

Ideální je řešit nasazení přes continous delivery. Oblíbené se staly Github Actions.

---

# <!--fit--> OAuth, OpenID

---

# OAuth

Zavádí autorizační vrstvu oddělující klienta a resource.

---

# Role

resource owner - uživatel, který může získat přístup
resource server - má data, která jsou chráněná proti neoprávněnému přístupu
klient - klientská aplikace, která získává data třeba přes REST
auth server - server starající se o identitu

---

# Flow

1. Klient pošle auth request na vlastníka (nebo ideálně na auth server)
2. Klient dostává grant
3. Klient posílá auth grant na auth server
4. Klient dostává access token
5. Klient posílá access token na resource
6. Klient dostává data

---

# 4 druhy flow

1. autorizační kód
2. implicit
3. resource owner password credentials
4. client credentials

---

# Implicit flow

Zjednodušené flow určené primárně pro webový prohlížeč. Je vynechána část pro získání autorizačního kódu/grantu. Klient získává přímo access token.
Pozn. neautentizuje klienta. Identita klienta může být ověřena podle return URL.

---

# Access token

= kredenšly pro přístup k resourcu

Je to string obsahující scope a délku trvání přístupu. Je nečitelný klientem.
Může být restriktivnější než grant.

---

# Refresh token

Optional. Slouží pro obnovení expirovaného access tokenu.
Nikdy se neposílá na resource server.

---

# Redirect z auth server

Klient je pro získání přístupu přesměrován na auth server. Ten má pri klienta zadanou return URL (je třeba registrovat předem všechny možné). Přesměrování je řešeno primárně přes 302 Found s uri v Location header.

---

# Registrace klienta

= typ klienta (confidential nebo public) + redirect url + další informace

---

# OpenID Connect 1.0

Identita nad OAuth 2.0. OID dává autentizaci a OAuth autorizaci.
Informace je vrácena jako JWT.

---

# Flow

1. Relying party pošle auth request na OpenID provider
2. User provede authn a authz na provideru
3. Relying party dostává AuthN response
4. Relying party posílá user info request na OpenID Provider
5. Relying party dostává user info.

---

# Request

scope = openid
respose_type
client_id
redirect_uri
state

---

# <!--fit--> Zero Trust Architectures

---

Korporátní sítě se dlouho spoléhali na bezpečnost na úrovni perimetru. Ve vnitřní síti nebylo často žádné zabezpečení.

Komunikace uvnitř sítě byla nešifrovaná.

---

# Tradiční zabezpečení infrastruktury

Zabezpečení na perimetru (tj. firewall), přičemž vnitřní komunikace je považována za důvěryhodnou.	
Pevné IP adresy a hardware pro určité aplikace.	
Identita založená na IP adrese.	
Služby jsou provozovány na známém, očekávaném místě.	
Specifické bezpečnostní požadavky zabudované do každé aplikace a vynucované samostatně.	
Omezená omezení způsobu vytváření a kontroly služeb.	
Omezený dohled nad bezpečnostními složkami.	
Složité a občasné nasazování.	
Appky jsou obvykle nasazovány jako virtuální počítače nebo na fyzické hosty a k zajištění izolace používají fyzický stroj nebo hypervizor.	

---

# Zásady bezpečnosti v cloudovém prostředí

Ochrana sítě na okraji
Žádná inherentní vzájemná důvěra mezi službami
Důvěryhodné stroje, na kterých běží kód se známým původem
Kontrolní body pro konzistentní prosazování zásad napříč službami
Jednoduché, automatizované a standardizované zavádění změn
Izolace mezi workloady sdílejícími operační systém

---

# <!--fit--> Makra

---

```rust
// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}

```

---

# Designator

```rust
macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro.
create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
}

```

---

# Designator

```rust
macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression);
    };
}

fn main() {
    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

```

---

# Možnosti

block
expr is used for expressions
ident is used for variable/function names
item
literal is used for literal constants
pat (pattern)
path
stmt (statement)
tt (token tree)
ty (type)
vis (visibility qualifier)

---

# Overload

```rust
// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    };
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    };
}

fn main() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}
```

---

# Repeat

```rust
// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}

```

---

# Variadická rozhraní

```rust
macro_rules! calculate {
    // The pattern for a single `eval`
    (eval $e:expr) => {{
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    // Decompose multiple `eval`s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
```

---

# Procedurální makra

1. funkční
2. derive
3. atributová

---

# Funkční

```rust
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
```

---

# Funkční

```rust
extern crate proc_macro_examples;
use proc_macro_examples::make_answer;

make_answer!();

fn main() {
    println!("{}", answer());
}
```

---

# Derive

```rust
extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
```

---

# Derive

```rust
extern crate proc_macro_examples;
use proc_macro_examples::AnswerFn;

#[derive(AnswerFn)]
struct Struct;

fn main() {
    assert_eq!(42, answer());
}
```

---

# Derive trait

```rust
#[proc_macro_derive(Trait)]
pub fn derive_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Trait for #name {
            fn print(&self) -> usize {
                println!("{}","hello from #name")
           }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
```

---

# Atribut

```rust
// my-macro/src/lib.rs

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}
```

---

# Atribut

```rust
// src/lib.rs
extern crate my_macro;

use my_macro::show_streams;

// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

```

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

