# Rust CRUD REST API

## Purpose

Learn web-development and microservices in Rust.

This application demonstrates the how to implement a common design for CRUDs in, potentially, a system of microservices. The design pattern inspired by nestjs in javascript.

This porject illustrates:

- use of `Actix-web`
- use of `sea-orm`
- demonstrates write `test` for web-application
- `dotenv` + `config` for configuration
- `logging`
- rest api

## API Design

It's REST and it's:

|Operation  |Method  |Endpoint                    |Body                                   |
|-----------|--------|----------------------------|---------------------------------------|
|Create     |POST    |`/api/user/`                |`{<field_1>:<value>, ... }`            |
|Get        |GET     |`/api/user/:phone_number`   |                                       |
|Get all    |GET     |`/api/user/`                |                                       |
|Update     |PUT     |`/api/user/:phone_number`   |`{<updated_field>:<new_value>, ... }`  |
|Delete     |DELETE  |`/api/user/:phone_number`   |                                       |

## Data Design and Entites

The database layout is simple. The database there is 1 table: (1) `users`. but migration and Entites Initialized with sea-orm.

## Application Design

The application itself has been designed with a modular approach. Dividing this project into multiple modules, and many small files was intentional, as it makes the overall architecture clear.

File structre inspired by nestjs. like this src:

src
├── config.rs
├── error.rs
├── lib.rs
├── main.rs
└── <module_0>
    ├── controller.rs
    ├── dto.rs
    ├── mod.rs
    └── service.rs

- `main.rs`, tried to keep simple. it's contain Initialize of other modules, and run http-server.
- `lib.rs` expose API for tests.
- `error.rs` uniform all error in projects.
- `config.rs` read configuration from environment (`.env` exists for conveniences).
- `<module_0>`, each resource is know as module and will configure separately. (like `users`).
  - `controller.rs`, has controllers of resource.
  - `dto.rs`, dto stand for Data-Transfer-Objects, they use in controllers as data transfer objects.
  - `mod.rs`, contain resource configuration.
  - `service.rs`, contain routines used in controllers.

## Test Coverage

This application uses an integration testing. These test serve as an example for what is sufficient test coverage for an initial application.

## Setup

1. Create database user

   ```shell
   createuser -P postgres
   ```

   Enter a password of your choice. The following instructions assume you used `123456` as password.

   This step is **optional** and you can also use an existing database user for that. Just make sure to replace `postgres` by the database user of your choice in the following steps and change the `.env` file containing the configuration accordingly.

`DATABASE_URL=postgres://<database_user>:<password>@<host>/<database_name>`

2. Create database

   ```shell
   createdb -O postgres testing_db
   ```

4. Rename `.env.sample` to `.env`.

5. Run the server:

   ```shell
   cargo run
   ```

6. Using a different terminal send an HTTP POST request to the running server:

   ```shell
   echo '{"phone_number": "09127274356", "first_name": "marco", "last_name": "rues"}' | http -f --json --print h POST http://127.0.0.1:8080/users
   ```

   **...or using curl...**

   ```shell
   curl -d '{"phone_number": "09127274356", "first_name": "marco", "last_name": "ruse"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/users
   ```

   A unique constraint exists for phone_number, so sending this request twice will return an internal server error (HTTP 500).

7. Run the tests:

   ```shell
   cargo test
   ```

Be comfortable and test api using Postman ;)
