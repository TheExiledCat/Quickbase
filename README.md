## Quickbase

_The all in one backend written in Rust_

---

## Full backend with database and api in one file

Quickbase makes it so you dont have to worry about your backend anymore so you can just focus on the frontend.

Quickbase consists of 2 parts: A schema and a runtime.

The schema contains all your definitions: your entities, the fields they can store, your api rules and DTOs and more

The runtime is the actual quickbase application. This is the REST API that allows you to both create the schema (using the admin endpoints) and also send queries to the database from the frontend while handling authentication for you.

## Installation

To install simply download one of the binaries for your platform from the release page.

to build from source install the cargo tool chain and use `cargo build -r`

## Usage
