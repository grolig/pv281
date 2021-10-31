---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust Databases
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

1. Docker pro setup prostředí
2. Postgres
3. SQLX
4. ORM & Diesel


---

# Docker

Kontejnerizační technologie, kterou si můžeme představit jako lehkou virtualizaci.

Kontejner je standardizovaný balík softwaru, který poskytuje osekaný OS, knihovny potřebné pro běh aplikace a appku samotnou.

---

# Instalace

stáhnout Docker for Desktop

```sh
https://www.docker.com/get-started
```

na Win10/11 je nutné nainstalovat nejdříve WSL2. Alternativa je mít nainstalované Hyper-V.

---

# Práce s Dockerem

spustit CMD a vyzkoušet

```sh
docker run -dp 80:80 docker/getting-started
```

to se připojí na Docker Hub, stáhne image, spustí démona, namapuje porty

---

# Závislosti pro dev

```yaml
# Use postgres/example user/password credentials
version: '3.1'
 
services:
 
  db:
    image: postgres
    restart: always
    environment:
      POSTGRES_PASSWORD: example
    volumes:
      - db-data:/var/lib/postgresql/data
 
  adminer:
    image: adminer
    restart: always
    ports:
      - 8080:8080
      
volumes:
  db-data:
```

---

# Postgres

klasická relační SQL databáze

open-source

s velkým množstvím funkcí

velmi dobrý výkon i pro velké systémy

---

# ORM
= object relation mapping

V Rustu sice nemáme objekty, ale mapujeme na struktury.

---

# Důležité detaily k ORM:

+ omezuje množství možných útoků
+ jednoduchá a na vývoj rychlá práce s databází
+ vše typovené, a tím pádem možné odhalit chyby

- ne vše podprované ORM knihovnami
- vygenerované dotazy nemusí být ideální

---

# Diesel

nejpoužívanější ORM v Rustu (ne že by bylo moc alternativ)
jeden z nejrychlejších ORM systémů
eliminuje runtime errory při práci s DB (aspoň většinu)
je celkem lehce rozšiřitelný

---

# Cargo.toml

```toml
[dependencies]
diesel = { version = "1.4.4", features = ["postgres"] }
dotenv = "0.15.0"
```

---

# CLI

nainstalujeme CLI

```sh
cargo install diesel_cli
```

a spustíme

```sh
cargo install diesel_cli --no-default-features --features postgres
```

pozn. je potřeba mít nainstalovaného klienta na práci s DB

---

# Konfigurace připojení

Na DEV si uděláme connection string

```sh
echo DATABASE_URL=postgres://postgres:postgrespass@localhost/simple_chat > .env
```

Jen to nedávejte do GITu, ani to takto nedělejte pro produkční appky.

---

# Setup Dieslu

```sh
diesel setup
```

vytvoří databázi, pokud neexistuje a složku na migrace

---

# Migrace

verzují změny ve struktuře databáze a umožňují zmigrovat data mezi verzema

```sh
diesel migration generate create_messages
```
 
---

# Up.sql

```sql
CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  text TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
)
```

---

# Down.sql

```sql
DROP TABLE messages
```

---

# Spuštění migrace

Na produkci je potřeba dát spuštění migrace buď do nasazení nebo provést při startu aplikace.

```sh
diesel migration run
```

---

# Přípojení k DB


```rust
#[macro_use]
extern crate diesel;
extern crate dotenv;
 
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
 
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
 
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
```

---

# Struktura pro čtení

```rust
use diesel::types::Timestamp;
 
#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub created_at: Timestamp,
}
```

---

# Struktura pro zápis

```rust
use super::schema::messages;
 
#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub text: &'a str,
}

```

---

# Insert

```rust
use self::simple_chat::*;
use self::models::*;
use self::diesel::prelude::*;
 
use self::models::{Message, NewMessage};
 
pub fn create_message<'a>(conn: &PgConnection, text: &'a str) -> Message {
    use schema::messages;
 
    let new_message = NewMessage {
        text: text,
    };
 
    diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result(conn)
        .expect("Error saving new message")
}
```

---

# Select

```rust
use self::simple_chat::*;
use self::models::*;
use self::diesel::prelude::*;
 
fn main() {
    use simple_chat::schema::messages::dsl::*;
 
    let connection = establish_connection();
    let results = messages.filter(published.eq(true))
        .limit(5)
        .load::<Message>(&connection)
        .expect("Error loading messages");
        
    let text = String::from("Prvni zprava");
    let message = create_message(&connection, text);
 
    println!("Displaying {} messages", results.len());
    for message in results {
        println!("{}", message.text);
        println!("----------\n");
    }
}
```

---

# r2d2

Jelikož všechno, co potřebujete si musíte dodat sami, tak musíte řešit i connection pooling.

Connection pooling umožňuje znovu používat databázové spojení. 

---

# r2d2

```toml
[dependencies]
diesel = { version = "1.0.0", features = ["postgres", "r2d2"] }
```

---

# Connection pool

```rust
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
 
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
 
fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}
 
pub fn establish_connection() -> PgPool {
    dotenv().ok();
 
    let database_url = env::var("DATABASE_URL")
         .expect("DATABASE_URL must be set");
         
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
        
    init_pool(&database_url).expect("Failed to create pool")
}
```

---

# SQLx

SQLx is an async, pure Rust† SQL crate featuring compile-time checked queries without a DSL.

Truly Asynchronous. Built from the ground-up using async/await for maximum concurrency.

Compile-time checked queries (if you want). See SQLx is not an ORM.

Database Agnostic. Support for PostgreSQL, MySQL, SQLite, and MSSQL.

Pure Rust. The Postgres and MySQL/MariaDB drivers are written in pure Rust using zero unsafe†† code.

Runtime Agnostic. Works on different runtimes (async-std / tokio / actix) and TLS backends (native-tls, rustls).

---

# Cargo.toml

```toml
[dependencies]
# tokio + rustls
sqlx = { version = "0.5", features = [ "runtime-tokio-rustls" ] }
# async-std + native-tls
sqlx = { version = "0.5", features = [ "runtime-async-std-native-tls" ] }
```

---


# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

