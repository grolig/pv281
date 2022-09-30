---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust Data Structures
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

1. Práce se soubory a stdin
2. TODO: Generika
3. TODO: Utility traity
4. TODO: Přetěžování operátorů
5. Vektory a iterátory
6. Datové struktury

---

# <!--fit--> Práce se soubory

---

# Vytvoření souboru a zápis

```rust
use std::fs::File;
// importuje běžně používané traity pro práci se soubory
use std::io::prelude::*;

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

# Načtení řádku

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

# Standardní vstup

```rust
use std::io;

fn main() {
   let mut input = String::new();
   
   match io::stdin().read_line(&mut input) {
       Ok(byte_count) => {
           println!("{} bytes read", byte_count);
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

# <!--fit--> Vektory a iterátory

---

# Připomenutí vektorů

Souvislý blok paměti, uložený na haldě, lze měnit jeho velikost.

```rust
fn main() {
    let values = vec![1, 2, 3];
    let values = vec![0; 64];

    let mut values = Vec::new();
    values.push(1);
    values.push(2);
    
    match values.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    for value in &values {
        println!("{}", value);
    }
}
```

---

# Iterátor

Jde o trait, který dává následující položku.
Vrácená položka je typu `Option`. Podle toho poznáme, jestli jsme na konci.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
    
    // ...
}
```

---

# Iterátor nad vektorem

Vektor implementuje trait `Iterator`. 
Můžeme využít funkce `.iter()`.

```rust
fn main() {
    let values = vec![1, 2, 3];

    let iterator = v1.iter();

    // `.iter()` lze využít i ve for cyklu
    for value in iterator {
        println!("Got: {}", val);
    }
}
```

---

# Closure

Anonymní funkce. Z jiných jazyků znáte jako _lambda funkce_.

```rust
fn main() {
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let closure_parameterless = || 1;
    println!("closure returning one: {}", closure_parameterless());
}
```

---

# Closure jako vstupní parametr

Argumenty funkcí musí být vždy explicitně otypovány.
Typ closure jakožto parametru musí být jeden z následujících traitů:

`Fn`: closure používá své parametry jako reference (`&T`)
`FnMut`: closure používá své parametry jako mutable reference
(`&mut T`)
`FnOnce`: closure se stává vlastníkem svých parametrů (`T`)

---

# Closure jako vstupní parametr

```rust
// Funkce, která bere closure jako parametr a zavolá ji.
// Poznámka: F je typické písmeno generického typu pro otypování closure.
fn apply<F>(f: F)
where
    // Samotná closure nemá žádné vstupní parametry a nic nevrací
    F: FnOnce(),
{
    f();
}

// Funkce, který bere closure jako parametr a vrací `i32`.
fn apply_to_3<F>(f: F) -> i32
where
    // Samotná closure bere i vrací `i32`.
    F: Fn(i32) -> i32
{
    f(3)
}

// TODO: zkuste si zaměnit `FnOnce()`, `Fn()` a `FnMut()` v kódu výše.
```

---

# Metody pracující s iterátory

1. Metody produkující jiný iterátor
2. Metody konzumující iterátor

---


### Map

Funkcionální přístup k iterování: na každý prvek iterátoru se zavolá closure, výsledkem je nový iterátor s modifikovanými prvky.

```rust
fn main() {
    let a = [1, 2, 3];

    // typem parametru metody `.map()` je `F: FnMut(Self::Item) -> B`
    let mut iter = a.iter().map(|x| 2 * x);

    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);
}
```

---

### Filter

Výsledkem je nový iterátor, jehož prvky tvoří podmnožinu prvků původních.

```rust
fn main() {
    let a = [1, 4, 2, 3];

    let sum = a.iter()
        .cloned() // duplikuje položky
        .inspect(|x| println!("about to filter: {}", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("made it through filter: {}", x))
}
```

---

### Filter Map

V iterátoru zůstanou jen ty prvky, pro které closure vrací `Some(mapped_value)`.

```rust
fn main() {
    let a = [-1, 1, -10, 10, 0];

    let mut iter = a
        .into_iter()
        .filter_map(|n| if n > 0 { Some(n.to_string()) } else { None });

    assert_eq!(iter.next(), Some("1".to_string()));
    assert_eq!(iter.next(), Some("10".to_string()));
    assert_eq!(iter.next(), None);
}
```

---

### Enumerate

Transformuje iterátor na iterátor dvojic: index a prvek.

```rust
fn main() {
    let a = ['a', 'b', 'c'];

    let mut iter = a.iter().enumerate();

    assert_eq!(iter.next(), Some((0, &'a')));
    assert_eq!(iter.next(), Some((1, &'b')));
    assert_eq!(iter.next(), Some((2, &'c')));
    assert_eq!(iter.next(), None);
}
```

Poznámka: index je typu `usize`, pro vlastní typ použijte `zip()`.

---

### Skip

Přeskočí prvních _n_ prvků.

```rust
fn main() {
    let a = [1, 2, 3];

    let mut iter = a.iter().skip(2);

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}
```

---

### Take

Vezme prvních _n_ prvků.

```rust
fn main() {
    let a = [1, 2, 3];

    let mut iter = a.iter().take(2);

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}
```

---

### Fold

Bere iniciální hodnotu akumulátoru a closure o dvou parametrech. Iterátor je zkonzumován.

```rust
fn main() {
    let a = [1, 4, 2, 3];
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(10, sum);
}
```

Poznámka: `reduce()` používá první prvek iterátoru jako iniciální hodnotu akumulátoru.
Poznámka č. 2: existuje metoda `sum()`.

---

### Zip

```rust
use std::iter::zip;

fn main() {
    let xs = [1, 2, 3];
    let ys = [4, 5, 6];
    for (x, y) in zip(&xs, &ys) {
        println!("x:{}, y:{}", x, y);
    }

    // Zip můžeme i vnořovat:
    let zs = [7, 8, 9];
    for ((x, y), z) in zip(zip(&xs, &ys), &zs) {
        println!("x:{}, y:{}, z:{}", x, y, z);
    }
}
```

---

# Spojení dvou iterátorů

```rust
fn main() {
    // Varianta 1: enumerate()
    let enumerated: Vec<_> = "foo".chars().enumerate().collect();

    // Varianta 2: zip()
    let zipped: Vec<_> = (0..).zip("foo".chars()).collect();

    assert_eq!((0, 'f'), enumerated[0]);
    assert_eq!((0, 'f'), zipped[0]);

    assert_eq!((1, 'o'), enumerated[1]);
    assert_eq!((1, 'o'), zipped[1]);

    assert_eq!((2, 'o'), enumerated[2]);
    assert_eq!((2, 'o'), zipped[2]);
}
```

---

# Pro side efekty je vhodnější použít for cyklus...

...pokud se nejedná o poslední volání v dlouhém řetězci metod iterátorů, pak se nabízí metoda `for_each()`.

---

# <!--fit--> Datové struktury

---

# Dvousměrný vektor 

Využití:
1. Chceme vkládat na začátek.
2. Potřebujeme frontu.
3. Potřebujeme obousměrnou frontu.

Je implementován jako _ring buffer_, tj. nemusí zabírat kontinuální prostor v paměti. Pokud chceme dělat slice, tak potřebujeme kontinuální prostor – získáme ho metodou `make_contiguous()`.

---

# Ring buffer

![w:512 h:200](./assets/04-images/circular-buffer.png)

---

# Dvousměrný vektor 

```rust
fn main() {
    use std::collections::VecDeque;

    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    buf.push_front(2);
    
    if let Some(elem) = buf.get_mut(2) {
        *elem = 7;
    }

    assert_eq!(d.pop_front(), Some(2));
    assert_eq!(buf[1], 7);
}
```

---

# Hašovací tabulka

Využití:
1. Potřebujeme slovník.
2. Potřebujeme cache.

Implementovaná podle _Google SwissTable_, jako hashovací funkci používá _SipHash 1-3_. Ta je vhodná pro středně velké slovníky a je odolná proti HashDoS útokům.
Pro malé a velké hashovací tabulky je vhodnější použít jinou hashovací funkci.

---

# Hašovací tabulka

![w:512 h:512](./assets/04-images/hash-table.png)

---

# Hašovací tabulka

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Inserts if the key does not exist.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}
```

---

# Hašovací tabulka – ukázka 2

```rust
fn main() {
    let mut book_reviews = HashMap::new();

    // Review some books.
    book_reviews.insert("Adventures of Huckleberry Finn".to_string(), "My favorite book. 10/10.".to_string());
    
    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.", book_reviews.len());
    }

    // Oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // Look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for &book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book)
        }
    }
}
```

---

# Množina

Využití:
1. Chceme zaznamenávat prošlé prvky.
2. Chceme mít hodnotu uloženou pouze jednou.

Nejrychlejší implementace je `HashSet`.
To platí ale jen do chvíle, než potřebujeme mít položky sežezené. Potom už použijeme `BTreeMap`.

---

# Množina

```rust
fn main() {
    use std::collections::HashSet;

    // Type inference lets us omit an explicit type signature (which would be `HashSet<String>` in this example).
    let mut books = HashSet::new();

    // Add some books.
    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    // Check for a specific book.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one.",
                books.len());
    }

    // Remove a book.
    books.remove("The Odyssey");
}
```

---

# B-strom

Využití:
1. Chceme mapu seřazenou podle klíčů.
2. Chceme získávat položky v nějakém rozsahu.
3. Chceme rychle získávat nejmenší nebo největší položku.
4. Chceme najít klíče, které jsou větší nebo menší než jiný klíč.

---

# B-strom

```rust
fn main() {
    use std::collections::BTreeMap;

    // Type inference lets us omit an explicit type signature (which would be `BTreeMap<&str, u8>` in this example).
    let mut player_stats = BTreeMap::new();

    // Insert a key only if it doesn't already exist.
    player_stats.entry("health").or_insert(100);

    // Insert a key using a function that provides a new value only if it doesn't already exist.
    player_stats.entry("defence").or_insert_with(|| 42);

    // Update a key, guarding against the key possibly not being set.
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += 13;
}
```

---

# B-strom – ukázka 2

```rust
fn main() {
    use std::collections::BTreeMap;

    // Type inference lets us omit an explicit type signature (which would be `BTreeMap<&str, &str>` in this example).
    let mut movie_reviews = BTreeMap::new();

    // Review some movies.
    movie_reviews.insert("Office Space",       "Deals with real issues in the workplace.");
    movie_reviews.insert("Pulp Fiction",       "Masterpiece.");
    movie_reviews.insert("The Godfather",      "Very enjoyable.");
    movie_reviews.insert("The Blues Brothers", "Eye lyked it a lot.");

    // Check for a specific one.
    if !movie_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.", movie_reviews.len());
    }

    // Continued on the next slide...
}
```

---

# B-strom – ukázka 2

```rust
fn main() {
    // Continued from previous slide...

    // Oops, this review has a lot of spelling mistakes, let's delete it.
    movie_reviews.remove("The Blues Brothers");

    // Look up the values associated with some keys.
    let to_find = ["Up!", "Office Space"];
    for movie in &to_find {
        match movie_reviews.get(movie) {
            Some(review) => println!("{}: {}", movie, review),
            None => println!("{} is unreviewed.", movie)
        }
    }

    // Look up the value for a key (will panic if the key is not found).
    println!("Movie review: {}", movie_reviews["Office Space"]);

    // Iterate over everything.
    for (movie, review) in &movie_reviews {
        println!("{}: \"{}\"", movie, review);
    }
}
```

---

# Halda

Využití:
1. Potřebujeme prioritní frontu.
2. Chceme pracovat s největší/nejdůležitější položkou.

---

# Halda

```rust
fn main() {
    use std::collections::BinaryHeap;

    // Type inference lets us omit an explicit type signature (which would be `BinaryHeap<i32>` in this example).
    let mut heap = BinaryHeap::new();

    // We can use peek to look at the next item in the heap. In this case, there's no items in there yet so we get None.
    assert_eq!(heap.peek(), None);

    // Let's add some scores...
    heap.push(1);
    heap.push(5);
    heap.push(2);

    // Continued on the next slide...
}
```

---

# Halda

```rust
fn main() {
    // Continued from previous slide...

    // Now peek shows the most important item in the heap.
    assert_eq!(heap.peek(), Some(&5));

    // We can check the length of a heap.
    assert_eq!(heap.len(), 3);

    // We can iterate over the items in the heap, although they are returned in a random order.
    for x in &heap {
        println!("{}", x);
    }

    // If we instead pop these scores, they should come back in order.
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), None);

    // We can clear the heap of any remaining items.
    heap.clear();

    // The heap should now be empty.
    assert!(heap.is_empty())
}
```

---

# Pro rozšíření možností iterátorů

využijte crate [itertools](https://docs.rs/itertools/latest/itertools/index.html)

---

# Příklady metod v itertools

`interleave()` - střídavě poskytuje prvky ze dvou iterátorů
`intersperse()` - mezi každý prvek iterátoru vloží hodnotu
`group_by()` - seskupuje po sobě jdoucí prvky se společným klíčem 
`merge()` - spojí dva iterátory sléváním
`sorted()` - seřadí iterátor bez potřeby vytvoření vektoru (interně iterátor zkonzumuje, seřadí a vytvoří nový)
`unfold()` - generuje iterátor na základě výchozího stavu a builder funkce

---

# Pro jednoduchou paralelizaci na úrovni iterátoru

využijte crate [Rayon](https://docs.rs/rayon/latest/rayon/index.html)

---

# Instalace Rayon

Do `Cargo.toml` přidáme závislost: 
```toml
[dependencies]
rayon = "1.5"
```

---

# Použití Rayon

Metodu `iter()` nahradíme za metodu `par_iter()`.
```rust
use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

