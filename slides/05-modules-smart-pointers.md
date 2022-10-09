---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust Generics and Smart Pointers
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

TODO: pridat podminenou kompilaci
1. Smart Pointer
2. Crates
3. Moduly
4. Testování
5. Dokumentace

---

# <!--fit--> Smart Pointery

---

# Pro připomenutí

1. Máme _stack_, který se uklízí automaticky na konci funkce.
2. Máme _heap_, kde máme velké objekty, objekty s neznámou velikostí za překladu a data, která nám musí přežít dlouho.
3. Heap se uklízí. V jiných jazycích to dělá _garbage collector_; v Rustu instrukce pro čištění dodá překladač.

---

#### Pointer
Základní odkaz na místo v paměti. Všechna omezení jsou definovaná _borrow checkerem_.
<br />
#### Smart Pointer
Chování odpovídá základnímu pointeru, ale dle varianty má další metadata a schopnosti.

---

# Smart Pointer

Smart Pointer implementuje trait `Deref` a `Drop`. Z toho je jasné, že je to struktura. Je vlastníkem dat, na která odkazuje.

Poznámka: `String` je smart pointer, např. kapacitu si drží jako metadata. Datové struktury, jako např. `Vec<T>`, jsou také smart pointery.

---

<!-- _class: split -->

### Deref coercion

<div class=common-text>

Automatický převod mezi referencemi parametrů funkcí/metod. Trait `Deref` umožňuje používat operátor dereference <code>*</code>.

</div>

<div class=left-column>

```rust
use std::ops::Deref;

struct DerefExample<T> {
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
```

</div>
<div class=right-column>

```rust
fn main() {
    let x = DerefExample { value: 'a' };
    
    assert_eq!('a', *x);
}
```

</div>

---

### Kde je operátor `->`?

V jazycích C a C++ se rozlišuje mezi `.` a `->` tak, že následující zápisy jsou ekvivalentní:

```c++
object->something();
(*object).something();
```

Rust při volání `object.something()` automaticky přidává `&`, `&mut`, či `*`, aby `object` typově odpovídal signatuře `something()`:

```rust
p1.distance(&p2);
(&p1).distance(&p2);
```

---

<!-- _class: split -->

### Drop

<div class=left-column>

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!
            "Dropping CustomSmartPointer with data `{}`!",
            self.data
        );
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```

</div>
<div class=right-column>

```shell
$ cargo run

CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

</div>


---

# Varianty smart pointerů

#### Box&lt;T>
#### Cell&lt;T>
#### RefCell&lt;T>
#### Rc&lt;T>
#### Ref&lt;T>
#### RefMut&lt;T>

---

# Box&lt;T>

Je nejjednodušším smart pointerem. Ukládá data na haldě (nikoli na zásobníku) i ve chvíli, kdy je jejich velikost známá za překladu. Samotný pointer může být na zásobníku, ale data ne.

Hodí se nám ve chvíli, kdy potřebujeme udělat rekurzivní struktury.

---

# Box&lt;T>

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}
```

```shell
$ cargo run

Cons(1, Cons(2, Nil))
```

---

# Cell&lt;T>

Umožňuje mutaci uvnitř imutabilní struktury. Má metody `get` a `set`. Metoda `get` vrací kopii dat.

Je vhodná pro menší data, a to z důvodu vracení kopie.

Je určena pouze pro jednovláknové použití. Vícevláknové alternativy budou v příští přednášce.

---

# Cell&lt;T>

```rust
use std::cell::Cell;

struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}

fn main() {
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };

    let new_value = 100;

    // ERROR: `my_struct` is immutable.
    // my_struct.regular_field = new_value;

    // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`, which can always be mutated.
    my_struct.special_field.set(new_value);
    assert_eq!(my_struct.special_field.get(), new_value);
}
```

---

# RefCell&lt;T>

Dává sdílený přístup k `T`, ale narozdíl od `Cell` je kontrolovaná za běhu.

Má metody, která vrací _mutovatelnou_ nebo _nemutovatelnou_ referenci. Musíme si sami napsat kontrolu, jestli se povedlo získat referenci.

Platí stejná pravidla pro získávání referencí jako při _borrow checkingu_ za překladu. Pokud ale pravidla porušíme, vlákno zpanikaří.

---

# RefCell&lt;T>

```rust
use std::cell::RefCell;

fn main() {
    let container = RefCell::new(11);
    
    {
        let _c = container.borrow();
        // You may borrow as immutable as many times as you want,...
        assert!(container.try_borrow().is_ok());
        // ...but cannot borrow as mutable because it is already borrowed as immutable.
        assert!(container.try_borrow_mut().is_err());
    } 
    
    // After the first borrow as mutable...
    let _c = container.borrow_mut();
    // ...you cannot borrow in any way.
    assert!(container.try_borrow().is_err());
    assert!(container.try_borrow_mut().is_err());
}

```

---

# Rc&lt;T>

Pokud potřebujeme více vlastníků, tak můžeme využít _reference counting_. Pokud existuje jakýkoliv odkaz, tak data nejsou uvolněna. 

`T` v `Rc` je imutabilní. Pokud chceme, aby se obsah dal měnit, tak musíme použít kombinace s `Cell` nebo `RefCell`.

Pokud tvoříme pomocí `Rc` cyklické vazby (např. obousměrné odkazy mezi potomkem a rodičem stromu), může dojít k _memory leaku_, protože čítač nikdy neklesne na 0.
Tehdy musíme použít `Weak<T>`, který není vlastníkem dat a ta mohou být odstraněna.

---

<!-- _class: split -->

### Rc&lt;T>

<div class=left-column>

```rust
use std::rc::Rc;
use List::{Cons, Nil}; // shorthand instead of using `List::Cons` everywhere

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after c is dropped: {}", Rc::strong_count(&a));
}
```

</div>
<div class=right-column>

```shell
$ cargo run

count after creating a: 1
count after creating b: 2
count after creating c: 3
count after c is dropped: 2
```

</div>

---

<!-- _class: split -->

### Kombinace Rc&lt;T> s RefCell&lt;U>

<div class=left-column>

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let tail = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let head_1 = Cons(Rc::new(RefCell::new(3)), Rc::clone(&tail));
    let head_2 = Cons(Rc::new(RefCell::new(4)), Rc::clone(&tail));

    *value.borrow_mut() += 10; // borrow_mut vrací RefMut, proto je tu třeba *.

    println!("tail   = {:?}", tail);
    println!("head_1 = {:?}", head_1);
    println!("head_2 = {:?}", head_2);
}
```

</div>
<div class=right-column>

```shell
$ cargo run

tail   = Cons(RefCell { value: 15 }, Nil)
head_1 = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
head_2 = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

</div>

---

# Ref&lt;T>

Typ obalující imutabilně vypůjčenou hodnotu z `RefCell<T>`.

```rust
use std::cell::{Ref, RefCell};

fn main() {
    let cell:  RefCell<(u32, char)> = RefCell::new((5, 'b'));
    let ref_1: Ref<(u32, char)>     = cell.borrow();
    let ref_2: Ref<u32>             = Ref::map(ref_1, |t| &t.0);

    assert_eq!(*ref_2, 5);
    assert_eq!(*cell.borrow(), (5, 'b'));

    // ERROR: borrow of moved value: `ref_1`
    // assert_eq!(*ref_1, (5, 'b'));
}
```

---

# RefMut&lt;T>

Typ obalující mutabilně vypůjčenou hodnotu z `RefCell<T>`.

```rust
use std::cell::{RefCell, RefMut};

fn main() {
    let cell: RefCell<(u32, char)> = RefCell::new((5, 'b'));

    {
        let ref_1:     RefMut<(u32, char)> = cell.borrow_mut();
        let mut ref_2: RefMut<u32>         = RefMut::map(ref_1, |t| &mut t.0);

        assert_eq!(*ref_2, 5);
        *ref_2 = 42;
    }

    assert_eq!(*cell.borrow(), (42, 'b'));
}
```

---

# <!--fit--> Modularita

---

# Crate

Jako názorná ukázka toho, co je to ```crate``` můžeme použít příkaz:

```cargo build --verbose```

---

# Crate

A nebo pohledem na závislosti v ```Cargo.toml```.

```toml
[dependencies]
num = "0.4"
image = "0.13"
crossbeam = "0.8"
```

---

# Externí závislost

V Rustu 2015 bylo nutné použít extern crate. Dnes to už potřeba není. Jelikož se s tímto zápisem stále můžete setkat, tak jej zde ukazujeme.

```rust

extern crate pcre;

extern crate std; // equivalent to: extern crate std as std;

extern crate std as ruststd; // linking to 'std' under another name

extern crate hello_world; // if hyphen in package name then it is replaced with an underscore
                          // crate name cannot have hyphen

extern crate foo as _; // when only linked and not referenced
```

---

# Externí závislosti

Dnes nám stačí použití ```use```.

```rust
use num::Complex;
// ...
use image::ColorType;
use image::png::PNGEncoder;
```

---

# Tranzitivní závislosti

Cargo si pří překladu stáhne zdrojový kód pro každou crate. Ta může záviset na dalších - tranzitivních závislostech. Vytvoří ze graf závislostí, který je cargem vyhodnocen a zpracován.

---

# Kompilace crate

Jednolivé crates jsou zkompilovány jako .rlib, která je následně staticky linkovaná do výsledné binárky.

---

# Moduly

Slouží k orgranizaci kódu v rámci projektu. Můžeme si je přiblížit jako Rustovou obdobu pro namespace. Je to logický kontejner pro struktury, funkce, typy aj.

---

# Ukázka modulu


```rust
mod spores {
    use cells::{Cell, Gene};

    pub struct Spore {
       // ...
    }

    /// Simulate the production of a spore by meiosis.
    pub fn produce_spore(factory: &mut Sporangium) -> Spore {
        // ...
    }

    /// Extract the genes in a particular spore.
    pub(crate) fn genes(spore: &Spore) -> Vec<Gene> {
        // ...
    }

    /// Mix genes to prepare for meiosis (part of interphase).
    fn recombine(parent: &mut Cell) {
       //    ...
    }

    // ...
}

```

---

# Komentář k ukázce

```pub``` udělá položku veřejnou, takže je přístupná mimo modul.
```pub(crate)``` je přístupná uvnitř crate, ale nemůže být použita jinými crates. Zároveň není součástí vygenerované dokumentace.
Cokoliv, co není označeno, se stává privátním. Může být použito pouze v rámci modulu nebo jeho potomků.

---

# Vnořené moduly

```rust
mod plant_structures {
    pub mod roots {
        pub(super) mod products {
            pub(in crate::plant_structures::roots) struct Cytokinin {
                ...
            }
        }
    }
    pub mod stems {
        ...
    }
    pub mod leaves {
        ...
    }
}
```

---

# Vnořené moduly

Modul může být veřejný (```pub```) stejně jako jiný typ.
```pub(super)``` omezí viditelnost pouze na nadřazený modul.
```pub(in ...)``` omezí viditelnost na daný modul a jeho potomky.


---

# Demostrační struktura programu

```sh
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

---

# Možná souborová struktura

```sh
.
 └── front_of_house
     ├── mod.rs
     ├── hosting.rs
     └── serving
         └── mod.rs
```

---

# Klíčové slovo mod

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}         
```

---

# Odkaz do jiného modulu

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}        
```

---

# Veřejné struktury

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

---

# Veřejné enumy

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

---

# Klíčové slovo use

---

# Use s absolutní cestou

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

---

# Use s relativní cestou

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting; // <- tady je rozdil

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

---

# Use pro funkci

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

---

# Use pro více modulů/členů

```rust
use std::io::{self, Write};
```

---

# Glob operátor

```rust
use std::collections::*;
```

---

# super

```rust
fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    // ...
}
```

---

# super

```rust
mod my {
    // ... 

    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();
        
        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        
        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
```

---

# Workspace

Pokud tvoříme projekt, který má více samostatně použitelných knihoven, ale chceme se na ně odkazovat bez nutnosti publikování, tak můžeme použít workspace.

```sh
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

---

# Cargo.toml workspacu

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

---

# Nová binárka do workspacu

```sh
cargo new adder
```

---

# Nová libka do workspacu

```sh
cargo new add-one --lib
```


---

# Závislost na libce v binárce

```toml
[dependencies]

add-one = { path = "../add-one" }
```

---

# Následné spuštění

```sh
cargo run -p adder
```

---

# Připomenutí konvencí pro binárky

```sh
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```

---

# <!--fit--> Testy

---

# Testy

Jednotkové testy jsou v souboru s implementací. Integrační testy jsou ve složce tests.

Spouštíme je přes 
```sh
cargo test
```

---

# Panika v testu

Zfailuje test.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

---

# Makro assert

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

---

# assert_eq!

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); // máme i assert_ne
    }
}
```

---

# vlastní zprávy při failu

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

---

# testování paniky

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

---

# result jako výsledek testu

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

---

# <!--fit--> Dokumentace

---

# Dokumentační komentáře

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

---

# Vytvoření sekcí

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

---

![w:768 h:512](./assets/05-images/doc-1.png)

---

# Ukázka Dokumentace

```
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
```

---

![w:768 h:512](./assets/05-images/doc-2.png)

---

# Testování dokumentace

```rust
/// Finally, we print the sum of `x` and `y`:
/// 
/// ```
/// # let x = 5;
/// # let y = 6;
/// println!("{}", x + y);
/// assert_eq!(x + y, 11);
/// ```
```

---

# ? v doc testech

```rust
/// ```
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
```

---

# Poznámky k dokumentaci

Generujeme pomocí
```sh
cargo doc
```

Examply testujeme pomocí
```sh
cargo test --examples
```

Dokumentaci přes
```sh
cargo test --doc
```

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

