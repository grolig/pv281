---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust Enums and Structures
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

1. Enumy
2. Pattern Matching
3. Struktury
4. Traity
5. Ošetření chyb
6. CLI aplikace

---

# <!--fit--> Enumy

---

# Základní varianta

```rust 
enum Delivery {
    Pickup,
    Parcel,
}

fn main() {
    let delivery = Delivery::Pickup;
}
```

---

# C varianta

```rust 
// hodnota defaultně začíná nulou
enum Number {
    Zero,
    One,
    Two,
}

// hodnota je ale nastavitelná
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // enum můžeme přetypovat na celé číslo
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
```

---

# Monadická varianta

```rust 
enum Delivery {
    Pickup,
    Parcel(String),
}

fn main() {
    let delivery = Delivery::Parcel(String::from("Ceska 0, 60200 Brno"));
}
```

---

# Paměťová reprezentace

* stejně jako union je velikost určena podle největší položky
* kromě toho přidává jestě skrytou položku pro diskriminant
* velikost diskriminantu je závislá na počtu variant

---

# Zpracování hodnoty pomocí `if let`

```rust
fn main() {
    let some_u8_value = Option::Some(0u8);
    
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
```

---

# <!--fit--> Pattern Matching

---

# Výhody pattern matchingu

1. Kontrola všech variant větvení
   To pomáhá i při refaktoringu - nikdy nezpomenete změnit další místa v kódu
2. Lepší čitelnost

---

### Zpracování hodnoty pattern matchingem

```rust 
fn deliver(delivery: Delivery) {
    match delivery {
        Delivery::Pickup => {
            println!("Vyzvednete to u nás v obchodě!");
        },
        Delivery::Parcel(address) => {
            println!("Zboží bude doručeno na adresu: {}!", address);
        },
        _ => {
            println!("Zatím neimplementovaný způsob doručení!");
        }
    }
}
```

---

# Porovnání s jinou proměnnou

```rust 
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

---

### S využitím tuple nebo struktury

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, and blue {}", r, g, b),
    }
}
```

---

# Match několika explicitně zadaných variant

```rust
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

---

# Match nad range

```rust
fn main() {
    let x = 'c';

    // ..= znamená včetně posledního prvku
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
```

---

# Match nad tuple

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}
```


---

# <!--fit--> Struktury

---

# Definice struktury dle C

```rust
#[repr(C)]
struct Foo {
  tiny: bool,
  normal: u32,
  small: u8,
  long: u64,
  short: u16,
}
```

---

# Zarovnání v paměti dle C

* nejprve překladač objeví ```tiny```, který má logickou velikost 1 bit - dostane 1 bajt
* následně vidí ```normal```, který má 4 bajty
* Pokud by ```tiny``` měl 1 byte, byly by problémy se zarovnáním proti ```normal```. Proto je za ```tiny``` vložené zarovnání velikosti 3 bajty.

---

# Zarovnání v paměti dle C

* Následuje ```small``` - má velikost 1 byte. Aktuální zarovnání je 1 + 3 + 4 = 8. Je zarovnáno, takže ```small``` může být vloženo na konec.
* S ```long``` máme zase stejný problém se zarovnáním. Abychom zarovnali, musíme za ```small``` vložit 7 bajtů.
* ```short``` vložíme přímo. 
* zarovnáme strukturu podle největší položky, takže za ```short``` přídáme 6 bajtů

---

# Změny v Rustu

* C reprezentace vyžaduje, aby položky byly za sebou dle definice
* výchozí (```repr(Rust)```) toto omezení odstraňuje
* v Rustu není ani deterministické řazení položek
* tato struktura po přeskládání položek bude mít pouze 16 bajtů, nepotřebujeme zarovnání

---

# Alternativní modely

```#[repr(packed)]``` 
* nepoužívá zarovnání
* se používá při scénářích s málo pamětí, nebo pro pomalé síťové spojení
* může velmi zpomalit vykonávání, může dojít k pádu pokud CPU podporuje pouze zarovnané argumenty.

---

# Alternativní modely

```#[repr(align(n))]``` 
* umožňuje větší zarovnání než by bylo nutné
* pro scénáře, kdy potřebujeme zařidít, aby položky byly v různých cache lines. Vyhneme se problému __false sharing__. 
* K false sharingu dochází, když různá CPU sdíli cache line. Oba se ji mohou pokusit změnit současně.

---

# Další ukázka struktury

```rust 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // ...
}
```

---

# Použití struktury

```rust 
struct User { 
    //... 
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

---

# Funkce pro vytvoření struktury

```rust 
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

---

### Vytvoření z již existující struktury

```rust 
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ...user1
    };
}
```

---

### Makro Debug

```rust 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    
    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

---

# Struktury s referencí

* pokud struktura má mít referenci, tak musíme definova lifetime

```rust
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32
}
```

---

# Příklad použití funkce extrema

```rust
fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least    { least    = &slice[i]; }
        if slice[i] > *greatest { greatest = &slice[i]; }
    }
    Extrema { greatest, least }
}
```

---

# Lifetime

Je konstrukce překladače, která dává dobu platnosti reference. Dříve bylo nutností ji explicitně definovat, dneska už není moc často třeba. Kód by  měl většinou jít napsat i bez specifikace lifetimu.

---

# Lifetime
```rust 
fn main() {
    let i = 3; // Lifetime pro `i` začíná. ────────────────┐
    //                                                     │
    { //                                                   │
        let borrow1 = &i; // `borrow1` lifetime začíná. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 koncí. ─────────────────────────────────┘│
    //                                                     │
    //                                                     │
    { //                                                   │
        let borrow2 = &i; // `borrow2` lifetime začíná. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` končí. ────────────────────────────────┘│
    //                                                     │
}   // Lifetime končí. ────────────────────────────────────┘

```

---

# Explicitní anotace lifetimu
```rust
// `print_refs` bere dvě reference na  `i32`, které mají
// lifetime `'a` a `'b`. Oba musí žít minimálně stejné 
// dlouho jako funkce `print_refs`.
fn print_refs<'a, 'b>(x: &'a i32, y: &'b mut i32) {
    println!("x is {} and y is {}", x, y);
}
```

---

# Lifetime s generikou

Pokud v předchozím příkladu nepoužijeme lifetime, tak příklad nejde přeložit. Překladač netuší, jestli bude návratová hodnota mít lifetime x nebo y. 

---

# Coercion
```rust
// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime
    
    {
        let second = 3; // Shorter lifetime
        
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
```

---

# Lifetime s generikou
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

# Další příklad generiky
```rust
// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}
```

---

# Lifetime u struktur
```rust
// Typ `Borrowed` obsahuje referenci na
// `i32`. Reference`i32` musí přežít `Borrowed`.
// Pokud máme ve struktuře referenci, tak musíme lifetime definovat vždy.
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Enum, který je `i32` or nebo referencí na něj.
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}
```

---

# Elision

Pro běžné příklady určuje lifetime sám překladač. Dělá to podle následujících pravidel:
1. pravidlo pro životnost vstupních parametrů
   Každý vstupní parametr dostává vlastní lifetime. 
2. pravidlo pro životnost výstupních parametrů
   Pokud má funkce jeden vstupní parametr, tak všechny výstupy mají stejný lifetime.
3. pravidlo pro metody s parametrem self
   Pokud má metoda vstupní parametr referenci na self, všechny výstupní parametry mají stejný lifetime.

---

# 'static

Snažte se mu vyhnout. Dává životnost po celý běh programu. Hodí se například pro chybové hlášky.

---

# Metody nad strukturou

```rust
pub struct Queue {
    older: Vec<char>,   // older elements, eldest last.
    younger: Vec<char>  // younger elements, youngest last.
}

impl Queue {
    /// Push a character onto the back of a queue.
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    /// Pop a character off the front of a queue. Return `Some(c)` if there
    /// was a character to pop, or `None` if the queue was empty.
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring the elements in younger over to older, and put them in
            // the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }
}
```

---

# Traity

Zjednodušeně můžeme **trait** považovat za **interface** v jiných programovacích jazycích.

Definujeme pomocí nich společnou funkcionalitu.

---

# Implementace traitu

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, 
        self.author, self.location)
    }
}
```

---

### Implementace traitu pro druhou strukturu

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

---

# Výchozí implementace

```rust
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

---

# Využití jiných metod ve výchozí implementaci

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

---

# Trait jako parametr

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```


---

# Trait Bound

Syntax `impl Trait` u parametru je syntaktický cukr pro delší zápis, kterému se říká **trait bound**. Následující bloky kódu jsou ekvivalentní, jen je zápis pomocí trait bound delší a hůře čitelný. Proto doporučujeme používat `impl Trait`.

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

---

# Více traitů

```rust
pub fn notify(item: &(impl Summary + Display)) {
    // ...
}
```

---

# Zápis pomocí where

Použijeme pro situace, kde máme více parametrů s různými kombinacemi traitů.

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone,
      U: Clone + Debug
{
    // ...
}
```

---

# Dynamický trait

```rust
use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}
```

---

# Trait Object

* ```dyn Write``` předstravuje jednu variantu polymorfismu, které říkáme trait object.
* slouží k provedení volání přes virtuální tabulku (vtable)
* C++ používá vptr jako součást struktury, Rust oproti tomu má fat pointer. Nic dalšího se do struktury nepřidává.
* trait object nemůže být použit jako typ proměnné, reference na něj ale ano
* trait object není známý v době překladu, proto obsahuje další informace o typu referenta
* Rust umožní koverzi Box<File> na Box<dyn Write>

---

# Reference na trait object

V jazyce Java je proměnná typu OutputStream (Java analogie pro std::io::Write) referencí na libovolný objekt, který implementuje OutputStream. Skutečnost, že se jedná o referenci, je samozřejmá. 

---

# Reference na trait object

```rust
let mut buf: Vec<u8> = vec![];
let writer: &mut dyn Write = &mut buf;
```

---

# Lifetime traitu
```rust
// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
```

---

# Subtrait

* můžeme vytvořit subtrait, který vyžaduje i implementaci nadřezeného
* říkáme, že ```Creature``` je extension ```Visible```

```rust
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
    ...
}
```

---

# Subtrait

* na pořadí implementace nezáleží

```rust
impl Visible for Broom {
    ...
}

impl Creature for Broom {
    ...
}
```

---

# <!--fit--> Ošetření chybových stavů

---

# Typy chyb

1. Chyby, ze kterých se můžeme zotavit.
2. Chyby, po kterých to můžeme zabalit.

---

# Panika

```rust
fn main() {
    panic!("crash and burn");
}
```

Poznámka: v Rustu při panice program sám projde stack a uklidí po sobě veškerá data. Je to za cenu větší binárky. Pokud chcete snížit velikost binárky a nechat úklid na operačním systému, tak můžete udělat následující konfiguraci v `Cargo.toml`:

```toml
[profile.release]
panic = 'abort'
```

---

# Enum `Result`

Funkce, kde může nastat chyba, vrací `Result`. Z něho můžeme vyčíst výsledek, nebo chybu.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

--- 

### Příklad ošetření chybového stavu

Pokud chceme otevřít soubor, tak může dojít k chybě.
Ošetřit ji můžeme následně:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the 'hello.txt' file: {:?}", error),
    };
}
```

---

# Ošetření dílčích chyb

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = File::open("hello.txt");

    let file = match file {
        Ok(ok_file) => ok_file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

---

# vlastní zpracování chyby pomoc &dyn Error

```rust
use std::error::Error;
use std::io::{Write, stderr};

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

```

---

# Zpanikaření v případě chyby

```rust
use std::fs::File;

fn main() {
    // Při chybě ukončí program s obecnou chybovou zprávou
    let file = File::open("hello.txt").unwrap();

    // Při chybě ukončí program s naší vlastní chybovou hláškou
    let file = File::open("hello.txt").expect("hello.txt se nepovedlo otevřít");
}
```

---

### Propagace chyb

Operátor `?` použijeme za hodnotou typu `Result`.
Pokud je hodnota `Ok(value)`, výsledkem operace je `value`.
Pokud je hodnota `Err(error)`, funkce skončí s `Err(error)`.

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {    
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

---

# Práce s chybami různých typů

```rust
type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;
```

---

---

# `anyhow`

Nejpopulárnější knihovna pro práci s chybami je anyhow. Je doporučená pro aplikace. Pro knihovny doporučujeme se podívat na thiserror

```toml
[dependencies]
anyhow = "1"
```

---

# Práce s chybami přes anyhow

```rust
use anyhow::Result;

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}
```

---

# <!--fit--> Práce s CLI

---

# Argumenty příkazové řádky

```rust
let pattern = std::env::args().nth(1).expect("No pattern given.");
let path = std::env::args().nth(2).expect("No path given.");
```

---

# Uložení argumentů do struktury

```rust
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given.");
    let path = std::env::args().nth(2).expect("No path given.");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
}
```

---

# `clap` crate

Nejpopulárnější knihovna na zpracování argumentů z CLI.
Závislost se liší podle toho, jestli budeme používat
`derive` pattern:

```toml
[dependencies]
clap = { version = "4.0.0-rc.2", features = ["derive"] }
```

nebo `builder` pattern:

```toml
[dependencies]
clap = "4.0.0-rc.2"
```

---

# Jednodušší zpracování přes clap

```rust
use clap::{ArgAction, Parser};

/// This doc string acts as a help message when the user runs '--help'.
/// The same applies for all doc strings on struct fields.
#[derive(Parser)]
#[clap(version = "1.0", author = "John Smith")]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short = 'c', long = "config", default_value = "default.conf")]
    config: String,
    
    /// Some input. Because this isn't an Option<T> it is required to be used
    input: String,
    
    /// A level of verbosity, and can be used multiple times
    #[clap(short = 'v', long = "verbose", action = ArgAction::Count)]
    verbose: u8,
}
```

---

# Jednodušší zpracování přes clap

```rust
fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v')
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}
```

---

# Zpracování přes builder pattern

```rust
use clap::{arg, Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("myapp")
        .version("1.0")
        .author("John Smith")
        .about("Does awesome things")
        .arg(arg!(-c --config [FILE] "Sets an optional custom config file"))
        .arg(arg!(<INPUT>            "Sets the required input file to use"))
        .arg(Arg::new("verbosity").short('v').long("verbose").action(ArgAction::Count))
        .get_matches();

    if let Some(i) = matches.get_one::<String>("INPUT") {
        println!("Value for input: {}", i);
    }
    if let Some(c) = matches.get_one::<String>("config") {
        println!("Value for config: {}", c);
    }
    match matches.get_count("verbosity") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be crazy"),
    }
}
```

---

# Standardní vstup

```rust
use std::io;

fn main() {
   let mut input = String::new();
   
   match io::stdin().read_line(&mut input) {
       Ok(n) => {
           println!("{} bytes read", n);
           println!("{}", input);
       }
       Err(error) => println!("error: {}", error),
   }
}
```

---

# Standardní vstup

```rust
use std::io;

fn main() {
   let lines = io::stdin().lines();
   for line in lines {
       println!("got a line: {}", line.unwrap());
   }
}
```

---

# <!--fit--> Práce se soubory

---

# Vytvoření souboru a zápis

```rust
use std::fs::File;
use std::io::prelude::*; // vlozi vezne pouzivane traity

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    
    file.write_all(b"Hello, world!")?;
    
    Ok(())
}
```

---

# Načtení obsahu ze souboru

```rust
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    
    Ok(())
}
```

---

# Práce přes buffer

```rust
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("foo.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!");
    
    Ok(())
}
```

---

# Načtení řádky

```rust
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("foo.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
```

---

# Synchronizace na disk

```rust
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    
    file.write_all(b"Hello, world!")?;
    file.sync_all()?;
    
    Ok(())
}
```

---

# Flush bufferu

```rust
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create("foo.txt")?);

    buffer.write_all(b"some bytes")?;
    buffer.flush()?;
    
    Ok(())
}
```

---

# Zápis přes buffer

```rust
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let data = "Some data!";
    let file = File::create("/tmp/foo").expect("Unable to create file");
    let mut file = BufWriter::new(f);
    
    file.write_all(data.as_bytes()).expect("Unable to write data");
}
```

Poznámka: dočasný adresář by bylo lepší zjistit nezávisle na platformě pomocí `env::temp_dir()`.

---
# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

