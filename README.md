# rust-pdx-schedule

## Installation and Setup

Ensure that Rust is installed by running `rustc --version`.
If not, follow the [official guide](https://www.rust-lang.org/tools/install) to install `rustup`.

There is an existing database in the [dist](dist/) directory, but the path to that database can be overridden by setting an
environment variable pointing to another path:

```bash
echo DATABASE_URL=/path/to/file > .env
```

Run `cargo run` to build the resources and run the application.

## Application Details

This application is the back-end for a web-application that displays PSU schedule information.

It has a few endpoints defined in it:
* http://localhost:8080/ routes to the application
* http://localhost:8080/api/classes/latest responds with JSON from the database matching the list of classes
* http://localhost:8080/api/courses responds with JSON from the database matching the list of courses
* http://localhost:8080/api/instructors responds with JSON from the database matching the list of instructors
* http://localhost:8080/api/terms responds with JSON from the database matching the list of terms

These endpoints were built, as they are required for front-end to function.

A few binaries were created in the course of developing a solution to interact with the database.
These binaries can be found in the [src/bin](src/bin/) directory.

Here are examples on how to run them:
* `cargo run --bin show_classoffering`
* `cargo run --bin show_courses`
* `cargo run --bin show_instructors`
* `cargo run --bin show_joined_classoffering`
* `cargo run --bin show_terms`

## Notable Third-Party Dependencies

|Crate      |Description                   |
|-----------|------------------------------|
| actix-web | Web framework                |
| diesel    | ORM and query builder        |
| serde     | Data serializer/deserializer |

