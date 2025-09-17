---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust GraphQL
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

1. Úvod do GraphQL
2. Dotazy
3. Mutace
4. Architektury s GraphQL
5. Juniper
6. Integrace do Actix

---

# <!--fit--> GraphQL

---

# Field

Field je jedna properta objektu. Z důvodu ušetření velikosti musíme vyjmenovat všechny fieldy, které chceme získat.

```graphql
{
  hero {
    name
  }
}
```

---

# Argumenty

Objekty můžou mít argumenty, které zužují výběr. Mohou být různého typu nebo i výčtu.

```graphql
{
  human(id: "1000") {
    name
    height(unit: FOOT)
  }
}
```

---

# Alias

V případě, kdy potřebujeme získat jeden objekt s různými filtry je vhodné upravit pojmenování pomocí aliasu.

```graphql
{
  empireHero: hero(episode: EMPIRE) {
    name
  }
  jediHero: hero(episode: JEDI) {
    name
  }
}
```

---

# Fragment

Pro opakující se položky vyžijeme fragment.

```graphql
{
  leftComparison: hero(episode: EMPIRE) {
    ...comparisonFields
  }
  rightComparison: hero(episode: JEDI) {
    ...comparisonFields
  }
}

fragment comparisonFields on Character {
  name
  appearsIn
  friends {
    name
  }
}
```

---

# Struktura dotazu

Dotaz je pojmenovaný. Jednotlivé proměnné jsou v dotazu otypovány a samotné hodnoty předány v samostatném objektu mimo dotaz.

```grapqhl
query HeroNameAndFriends($episode: Episode) {
  hero(episode: $episode) {
    name
    friends {
      name
    }
  }
}

{
  "episode": "JEDI"
}
```

---

# Výchozí hodnoty

V případě kdy hodnota proměnné zadaná, může být doplněna výchozí hodnotou.

```graphql
query HeroNameAndFriends($episode: Episode = JEDI) {
  hero(episode: $episode) {
    name
    friends {
      name
    }
  }
}
```

---

# Direktivy

```graphql
query Hero($episode: Episode, $withFriends: Boolean!) {
  hero(episode: $episode) {
    name
    friends @include(if: $withFriends) {
      name
    }
  }
}

{
  "episode": "JEDI",
  "withFriends": false
}
```

---

# Mutace

Mutace slouží k modifikaci položek, vytvoření, mazání apod.

```graphql
mutation CreateReviewForEpisode($ep: Episode!, $review: ReviewInput!) {
  createReview(episode: $ep, review: $review) {
    stars
    commentary
  }
}

{
  "ep": "JEDI",
  "review": {
    "stars": 5,
    "commentary": "This is a great movie!"
  }
}
```

---

# Architektury s GQL

---

# GraphQL nad REST

@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

Container(fe, "Frontend", "")

Container(gql, "Vrstva GraphQL", "Rust, Juniper")

Container(s1, "REST služba 1", "Rust, Actix")
Container(s2, "REST služba 2", "Node.js, Express")
Container(s3, "REST služba 3", "C#, ASP.NET")

Container(db1, "SQL DB 1", "Postgres")
Container(db2, "SQL DB 2", "Postgres")
Container(db3, "noSQL DB", "Mongo")

Rel_D(fe, gql, "")

Rel_D(gql, s1, "")
Rel_D(gql, s2, "")
Rel_D(gql, s3, "")

Rel_D(s1, db1, "")
Rel_D(s2, db2, "")
Rel_D(s3, db3, "")

@enduml

---

# GRPC backend

@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

Container(fe, "Frontend", "")

Container(gql, "Vrstva GraphQL", "Rust, Juniper")

Container(s1, "GRPC služba 1", "Rust, Tonic")
Container(s2, "GRPC služba 2", "C#")
Container(s3, "REST služba 3", "C#, ASP.NET")

Container(db1, "SQL DB 1", "Postgres")
Container(db2, "SQL DB 2", "Postgres")
Container(db3, "SQL DB 3", "MS SQL")

Rel_D(fe, gql, "")

Rel_D(gql, s1, "")
Rel_D(gql, s2, "")
Rel_D(gql, s3, "")

Rel_D(s1, db1, "")
Rel_D(s2, db2, "")
Rel_D(s3, db3, "")

@enduml

---

# GraphQL nad databází

@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

Container(fe, "Frontend", "")

Container(gql, "GraphQL", "Rust, Juniper")

Container(db1, "SQL DB 1", "Postgres")

Rel_D(fe, gql, "")

Rel_D(gql, db1, "")

@enduml

---

# <!--fit--> Juniper

---

# Závislosti

```toml
[dependencies]
juniper = "0.14.2"
```

---

# Integrace s Actix

```rust
use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::schema::{create_schema, Schema};
```

---

# Integrace s Actix

```rust
#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

# Registrace endpointů

```rust
async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}
```

---

# Schéma - výčet

```rust
use juniper::FieldResult;
use juniper::RootNode;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

```

---

# Schéma - struktura

```rust
use juniper::FieldResult;
use juniper::RootNode;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

```

---

# Schéma - struktura pro mutaci

```rust
use juniper::FieldResult;
use juniper::RootNode;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

```

---

# Kořen pro dotazy

```rust
pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}
```

---

# Kořen pro mutace

```rust
pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}
```

---

# Vytvoření schéma z kořenů

```rust
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
```

---

# Kontext

```rust
pub struct Context {
    pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    fn users(context: &Context) -> FieldResult<Vec<User>> {
        let mut conn = context.dbpool.get().unwrap();
        let users = conn
            .prep_exec("select * from user", ())
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (id, name, email) = from_row(row);
                        User { id, name, email }
                    })
                    .collect()
            })
            .unwrap();
        Ok(users)
    }
}

```

---

# Naplnění kontextu z Actixu

```rust
pub async fn graphql(
    pool: web::Data<Pool>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        dbpool: pool.get_ref().to_owned(),
    };

    let res = web::block(move || {
        let res = data.execute_sync(&schema, &ctx);
        serde_json::to_string(&res)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}

```

---

# Playground

```rust
use juniper::http::graphiql::graphiql_source;

pub async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

// register to be called in actix builder
pub fn register(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphql_playground));
}
```

---

# Like this

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();
    let pool = get_db_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(register)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

