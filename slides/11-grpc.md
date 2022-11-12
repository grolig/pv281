---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust Grpc
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

1. Úvod do gRPC
2. Protobuf
3. Tonic
4. Debugging

---

# gRPC

gRPC je architektura pro rychlé a vysoce propustné propojení služeb. Protokol byl navržený Googlem jako alternativa k RESTu. Umožňuje propojení služeb psaných v různých jazycích (RPC).

Zprávy jsou předávány v binární serializované podobě. Kontrakt (struktura) je definovaný pomocí Protobuf protokolu "protocol buffer". 

---

# System architecture

![w:512 h:512](./assets/10-images/grpc.svg)

---

# Why gRPC vs REST?

![w:1024 h:512](./assets/10-images/grpc-vs-rest.png)

---

# Why Rust for gRPC?

![w:1024 h:512](./assets/10-images/grpc-fw-benchmark.png)

---

# Message

Data jsou předávána jako message. Položkám říkáme field.

```proto
message Person {
  string name = 1;
  int32 id = 2;
  bool has_ponycopter = 3;
}
```

---

# Service 

Dále máme services. To jsou akce, které můžeme provolat a vrátí nám odpověď.

```proto
// The greeter service definition.
service Greeter {
  // Sends a greeting
  rpc SayHello (HelloRequest) returns (HelloReply) {}
}

// The request message containing the user's name.
message HelloRequest {
  string name = 1;
}

// The response message containing the greetings
message HelloReply {
  string message = 1;
}
```

---

# Streaming

Data je možné streamovat.

```proto
rpc BidiHello(stream HelloRequest) returns (stream HelloResponse);
```

---

# Typy

```
+─────────────+──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────+
| proto Type  | Notes                                                                                                                                            |
+─────────────+──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────+
| double      |
| float       |
| int32       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead.  |
| int64       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead.  |
| uint32      | Uses variable-length encoding.                                                                                                                   |
| uint64      | Uses variable-length encoding.                                                                                                                   |
| sint32      | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s.                             |
| sint64      | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s.                             |
| fixed32     | Always four bytes. More efficient than uint32 if values are often greater than 228.                                                              |
| fixed64     | Always eight bytes. More efficient than uint64 if values are often greater than 256.                                                             |
| sfixed32    | Always four bytes.                                                                                                                               |
| sfixed64    | Always eight bytes.                                                                                                                              |
| bool        |
| string      | A string must always contain UTF-8 encoded or 7-bit ASCII text, and cannot be longer than 232.                                                   |
| bytes       | May contain any arbitrary sequence of bytes no longer than 232.                                                                                  |
+─────────────+──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────+

```

---

# Výčet

```proto
message MyMessage1 {
  enum EnumAllowingAlias {
    option allow_alias = true;
    UNKNOWN = 0;
    STARTED = 1;
    RUNNING = 1;
  }
}
```

---

# Repeated

Aneb list/vector

```proto
message SearchResponse {
  repeated Result results = 1;
}

message Result {
  string url = 1;
  string title = 2;
  repeated string snippets = 3;
}
```

---

# Map

```proto
message Result {
  string url = 1;
  string title = 2;
  map<int32, string> my_map = 4;
}
```

---

# Wellknown types

```proto
import "google/protobuf/timestamp.proto";
import "google/protobuf/empty.proto";

message SalesOrder 
{
   int32 SoId = 1;
   google.protobuf.Timestamp DeliveryDate = 2;
}
```

---

# Chybové kódy

```
const (
	OK Code = 0
	Canceled Code = 1
	Unknown Code = 2
	InvalidArgument Code = 3
	DeadlineExceeded Code = 4
	NotFound Code = 5
	AlreadyExists Code = 6
	PermissionDenied Code = 7
	ResourceExhausted Code = 8
	FailedPrecondition Code = 9
	Aborted Code = 10
	OutOfRange Code = 11
	Unimplemented Code = 12
	Internal Code = 13
	Unavailable Code = 14
	DataLoss Code = 15
	Unauthenticated Code = 16
)
```

---

# GRPC web

- přechodová vrstva mezi prohlížečem a GRPC serverem
- často využívá proxy
- komunikuje přes HTTP2 s možností fallbacku na HTTP1 (chunky)

---

# Identity Server

- open-source řešení Ory Kratos
- pro OAuth2 Ory Hydra

---

# Poznámky

- Auth se řeší přes metadata. Obsluha je na úrovni jazyka/platformy, kterou využíváme.
- Ne vždy musí být gRPC rychlejší než REST.

---

# <!--fit--> Tonic

---

# Závislosti

```rust
[dependencies]
tonic = "0.4"
prost = "0.7"
futures-core = "0.3"
futures-util = "0.3"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "sync", "time"] }

async-stream = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rand = "0.7"

[build-dependencies]
tonic-build = "0.4"
```

---

# Build.rs

```rust
fn main() {
    tonic_build::compile_protos("proto/definice.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
```

---

# Server

```rust
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}
```

---

# Greeter implementace

```rust
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

```

---

# Server main

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
```

---

# Klient

```rust
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}
```

---

# Klient main

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```

---

# Autentizace na klientu

```rust
pub mod pb {
    tonic::include_proto!("grpc.examples.echo");
}

use pb::{echo_client::EchoClient, EchoRequest};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let token = MetadataValue::from_str("Bearer some-auth-token")?;

    let mut client = EchoClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(EchoRequest {
        message: "hello".into(),
    });

    let response = client.unary_echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```

---

# gzip komprese

```rust
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tonic::transport::Channel;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::builder("http://[::1]:50051".parse().unwrap())
        .connect()
        .await
        .unwrap();

    let mut client = GreeterClient::new(channel).send_gzip().accept_gzip();

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    dbg!(response);

    Ok(())
}
```

---

# Grpc Web

```rust
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse().unwrap();

    let greeter = MyGreeter::default();
    let greeter = GreeterServer::new(greeter);
    let greeter = tonic_web::config()
        .allow_origins(vec!["127.0.0.1"])
        .enable(greeter);

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(greeter)
        .serve(addr)
        .await?;

    Ok(())
}
```

---

# REST gateway před GRPC

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

