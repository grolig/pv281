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
2. Práce s proměnnými prostředí
3. SQLX
4. ORM

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

# Alternativy k Dockeru

Vzhledem k licenční police se dnes přechází od použití Dockeru jako řešení kontejnerizace. Na produkci se používá v rámci Kubernetes conteinerd, a trend je ho využít i pro lokální vývoj.

Rancher Desktop
colima + nerdctl

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

# PlantUML

```
@startuml
' hide the spot
hide circle

' avoid problems with angled crows feet
skinparam linetype ortho

entity "Entity01" as e01 {
  *e1_id : number <<generated>>
  --
  *name : text
  description : text
}

entity "Entity02" as e02 {
  *e2_id : number <<generated>>
  --
  *e1_id : number <<FK>>
  other_details : text
}

entity "Entity03" as e03 {
  *e3_id : number <<generated>>
  --
  e1_id : number <<FK>>
  other_details : text
}

e01 ||..o{ e02
e01 |o..o{ e03
@enduml
```

---

![w:512 h:512](./assets/08-images/plantuml.png)

---

# Postgres

+ klasická relační SQL databáze
+ open-source
+ s velkým množstvím funkcí
+ velmi dobrý výkon i pro velké systémy

- občas neřešené starší bugy
- performance jiných DB systémů bývá lepší

---

# Přístupy pro práci s databází

## Ručně vytvořené SQL dotazy
+ veškeré funkce k dispozici
+ lehké optimatlizovat výkon

- možnost SQL injection (vzhledem k neznalosti)
- nutnost znát SQL a zavádět další jazyk do projektu

---


# Přístupy pro práci s databází

## Query Builder
+ většina funkcí k dispozici
+ stále relativně lehké optimatlizovat výkon
+ nedochází k SQL injection
+ není zavedený další jazyk do projektu

- nutná znalost SQL a k tomu knihovny, která ho na pozadí vygeneruje
- nemáme tolik možností jako u čistého SQL


---

# Přístupy pro práci s databází

# ORM = object relation mapping

+ omezuje množství možných útoků
+ jednoduchá a na vývoj rychlá práce s databází
+ vše typovené, a tím pádem možné odhalit chyby

- ne vše podprované ORM knihovnami
- ztráta výkonnosti - vygenerované dotazy nemusí být ideální

---

# Diesel

+ nejpoužívanější ORM v Rustu
+ jeden z nejrychlejších ORM systémů v Rustu
+ eliminuje runtime errory při práci s DB (aspoň většinu)
+ je celkem lehce rozšiřitelný

- komplexnější dotazy jsou komplikované, a musíte si stejně sami stavět dotaz
- komplexnější věci jsou opravdu tak komplikované, že je lepší využí jiné technologie

---

# Connection pooling

- vytváření a zavření spojení je drahé a způsobuje latenci
- spojení si můžeme uložit a nechat jej otevřené, tím nemusíme platit za jeho nové vytvoření
- díky poolu můžeme i ovlinit minimální a maximální počet spojení

---

# Cachování dotazu

- databázové dotazy je vhodné cachovat 
- běžné je použití in-memory cache jako je Redis
- vytáhnout výsledek z Redisu (key-value) je levnější než zpracovat dotaz nad DBMS

---

# Práce s proměnnými prostředí

```rust

use std::env;

fn main() {
    let host_key = "HOST";
    let port_key = "PORT";
    let default_port = 8080;
    
    let host = match env::var(host_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, host_key);
            process::exit(1);
        },
    };
}

```

---

# Práce s proměnnými prostředí

```rust

use std::env;

fn main() {
    let host_key = "HOST";
    let port_key = "PORT";
    let default_port = 8080;
    
    let host = env!(host_key);
    let port = option_env!(port_key);
}
```

---

# Envy

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
  foo: u16,
  bar: bool,
  baz: String,
  boom: Option<u64>
}

fn main() {
    match envy::from_env::<Config>() {
       Ok(config) => println!("{:#?}", config),
       Err(error) => panic!("{:#?}", error)
    }
}
```

---

# structopt

```rust
use std::path::PathBuf;
use structopt::StructOpt;

/// A database applicaiton
#[derive(StructOpt, Debug)]
#[structopt(name = "dbapp")]
struct Opt {
    /// Database URL
    #[structopt(short, long, parse(from_os_str))]
    database_url: String,

    /// Port number
    #[structopt(short = "p", long)]
    port: Option<i32>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
```

---

# .env

```rust
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
```

---

# Konfigurace připojení

Na DEV si uděláme connection string (jen to nedávejte do GITu...)

```sh
echo DATABASE_URL=postgres://postgres:postgrespass@localhost/simple_chat > .env
```

Načteme následně pomocí `dotenv`.

---

# SQLx

SQLx je crate ke kontole dotazu během kompilace. Nepoužívá žádný DSL.

Podporuje PostgreSQL, MySQL, SQLite, and MSSQL.

Podporuje různé asynchronní runtimy (async-std / tokio / actix) a TLS backendy (native-tls, rustls).

---

# Cargo.toml

```toml
[dependencies]
# tokio + rustls
sqlx = { version = "0.5", features = [ "runtime-tokio-rustls", "offline" ] }
# async-std + native-tls
sqlx = { version = "0.5", features = [ "runtime-async-std-native-tls", "offline" ] }
```

---

# Migrace

- pro migrace nainstalujeme `sqlx-cli` 
- ```cargo install sqlx-cli```

- ujistíme se, že v projektu máme `.env` soubor s proměnnou `DATABASE_URL`

---

# Vytvoření databáze

```sh
sqlx database create
sqlx database drop
```

---

# Vytvoření migrace

Pro vytvoření migrace použijeme příkaz `migrate add`

```sh
sqlx migrate add <name>
Creating migrations/20211001154420_<name>.sql
```

---

# Revertibilní migrace 

Přepínačem -r vytvoříme revertibilní migraci

```sh
sqlx migrate add -r user
Creating migrations/20211001154420_user.up.sql
Creating migrations/20211001154420_user.down.sql
```

---

# Vytvoření tabulky v migraci

Soubor `user.up.sql`

```sql
create table "user"
(
    user_id       uuid primary key default gen_random_uuid(),
    username      text unique not null,
    password_hash text        not null
);
```

Soubor `user.down.sql`
```sql
drop table user; -- nebezpecne
```

---

# Spuštění migrace

```sh
sqlx migrate run
Applied migrations/20211001154420 user (32.517835ms)
```

---

# Reverzace migrace

```sh
sqlx migrate revert
Applied 20211001154420/revert user (32.517835ms)
```

---

# Spuštění migrace v aplikaci

```rust
sqlx::migrate!("db/migrations") // <- cesta ke slozce s migracemi nebo konkretni soubor migrace
    .run(&pool)
    .await?;
```
---

# Prepare a offline mode

Před vytvářením dotazů v aplikaci je nutné udělat prepare. Bez něj budeme mít problémy s query makry.

```cargo sqlx prepare```

Na CLI můžeme provést check bez připojení k db
```cargo sqlx prepare --check```

Poznámky: proměnná prostředí SQLX_OFFLINE na true enforcuje kontrolu proti offline modelu a ne DB.

---

```rust
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?).await?;

    let row: (i64,) = sqlx::query_as("SELECT ?")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}

```

---

```rust
use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}

```

---

# Command

```rust
sqlx::query("DELETE FROM table").execute(&mut conn).await?;
sqlx::query("DELETE FROM table").execute(&pool).await?;

```

---

# Verifikování při kompilaci

```rust
let countries = sqlx::query!(
        "
                SELECT country, COUNT(*) as count
            FROM users
            GROUP BY country
            WHERE organization = ?
        ",
        organization
    )
    .fetch_all(&pool) // -> Vec<{ country: String, count: i64 }>
    .await?;

// countries[0].country
// countries[0].count

```

---

# query_as! do struktury

```rust
// no traits are needed
struct Country { country: String, count: i64 }

let countries = sqlx::query_as!(Country,
        "
SELECT country, COUNT(*) as count
FROM users
GROUP BY country
WHERE organization = ?
        ",
        organization
    )
    .fetch_all(&pool) // -> Vec<Country>
    .await?;

// countries[0].country
// countries[0].count

```

---

# Funkce pro práci s DB

```rust
async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
            SELECT id, description, done
            FROM todos
            ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}
```

---

# Transakce

```rust
use sqlx::postgres::PgPoolOptions;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;

    let mut tx = pool.begin().await?; // <- begin slouzi i pro vytvoreni savepointu, pokud vnorime transakce

    sqlx::query("INSERT INTO articles (slug) VALUES ('this-is-a-slug')")
        .execute(&mut tx).await?; // <- otaznik zpusobi okamzity rollback pri vraceni chyby

    tx.commit().await?;

    // tx.rollback().await?;

    Ok(())
} // <- rollback je provedeny taky v ramci Drop na konci scopu

```


---

# Repository pattern

```rust
#[async_trait]
pub trait TodoRepo {
    async fn add_todo(&self, description: String) -> anyhow::Result<i64>;
    async fn complete_todo(&self, id: i64) -> anyhow::Result<bool>;
    async fn list_todos(&self) -> anyhow::Result<()>;
}

struct PostgresTodoRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresTodoRepo {
    fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pg_pool),
        }
    }
}

```

---

# Repository pattern

```rust
#[async_trait]
impl TodoRepo for PostgresTodoRepo {
    async fn add_todo(&self, description: String) -> anyhow::Result<i64> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO todos ( description )
            VALUES ( $1 )
            RETURNING id
            "#,
            description
        )
        .fetch_one(&*self.pg_pool)
        .await?;

        Ok(rec.id)
    }

    async fn complete_todo(&self, id: i64) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE todos
            SET done = TRUE
            WHERE id = $1
            "#,
            id
        )
        .execute(&*self.pg_pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}

```

---

# Práce s časem

K dispozici jsou knihovny `time` a `chrono`, který je postavený nad time. 

Problém chrono byl chybějící maintainer dělší dobu, kdy některé důležité bugy nebyly opraveny, proto hodně uživatelů přešlo na time. Dneska už maintainera má, a pravidelně dostává opravy.

Time bylo původně postavené nad libc a taky skončilo bez maintainera. Později se objevil nový, došlo ke kompletnímu přepisu. Chrono dlouho zůstávalo na staré verzi 0.1. 

Dnes už nejsou nekompatibilní. Chrono je stále používanější z obou dvou.

SQLX v případě povolení obou preferuje použítí time.

---

# time

```toml
[dependencies]
time = { version = "0.3", features = ["macros"] }
sqlx = { version = "0.5", features = [ "runtime-tokio-rustls", "time" ] }


```

---

# time

```rust
use time::{Date, PrimitiveDateTime, OffsetDateTime, UtcOffset};
use time::Weekday::Wednesday;

let date = Date::from_iso_week_date(2022, 1, Wednesday).unwrap();
let datetime = date.with_hms(13, 0, 55).unwrap();
let datetime_off = datetime.assume_offset(UtcOffset::from_hms(1, 2, 3).unwrap());

println!("{date}, {datetime}, {datetime_off}");
// 2022-01-01, 2022-01-01 13:00:55.0, 2022-01-01 13:00:55.0 +01:02:03
```

---

# time - makra

```rust
use time::macros::{date, datetime};

let date = date!(2022-01-01);
let datetime = datetime!(2022-01-01 13:00:55);
let datetime_off = datetime!(2022-01-01 13:00:55 +1:02:03);

println!("{date}, {datetime}, {datetime_off}");
// 2022-01-01, 2022-01-01 13:00:55.0, 2022-01-01 13:00:55.0 +01:02:03
```

---

# time - offset

```rust
    assert_eq!(
        datetime!(2000-01-01 0:00 UTC).to_offset(offset!(-1)).year(),
        1999,
    );

    let sydney = datetime!(2000-01-01 0:00 +11);
    let new_york = sydney.to_offset(offset!(-5));
    let los_angeles = sydney.to_offset(offset!(-8));
    assert_eq!(sydney.hour(), 0);
    assert_eq!(sydney.day(), 1);
    assert_eq!(new_york.hour(), 8);
    assert_eq!(new_york.day(), 31);
    assert_eq!(los_angeles.hour(), 5);
    assert_eq!(los_angeles.day(), 31);
```

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

