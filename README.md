# rust-pdx-schedule

## Description

The motivation behind this project was to replace the implementation of the server-side aspect an existing dynamic web application.
The front-end of this application was already written in Elm – a purely functional language that transpiles to JavaScript – and the goal was to have a full-stack purely functional web application.

The purpose of the web application is to display PSU schedule information for the current term.
It gets its information via an HTTP request that returns the database information in JSON.
That information contains instructor, course, and term data.

The server is then responsible for serving up the database information as well as the HTML page.
It must set up API endpoints that, once hit will perform queries on the database and return the appropriate information as JSON.
The root endpoint of the server must load the front-end successfully.

Alternatively, a simple Yew project exists on the [client](./client) section of the repository that retrieved and displays hardcoded course data from the back-end.

## Installation and Setup

Ensure that Rust is installed by running `rustc --version`.
If not, follow the [official guide](https://www.rust-lang.org/tools/install) to install `rustup`.

There is an existing database in the [dist](dist/) directory, but the path to that database can be overridden by setting an
environment variable pointing to another path:

```bash
echo DATABASE_URL=/path/to/file > .env
```

## Running the Application

In the terminal, execute `cargo run` to build the resources and run the application.

When the server is up and running, visit http://localhost:8080.

For instructions how to run the Yew client, go [here](./client/README.md).

## Application Details

This application is the back-end for a web-application that displays PSU schedule information.

It has a few endpoints defined in it:
* http://localhost:8080/ routes to the application
* http://localhost:8080/api/class/{id} responds with JSON from the database matching the class offering with the class id
* http://localhost:8080/api/classes/latest responds with JSON from the database matching the list of classes
* http://localhost:8080/api/course/{id} responds with JSON from the database matching the course with the course id
* http://localhost:8080/api/courses responds with JSON from the database matching the list of courses
* http://localhost:8080/api/instructor/{id} responds with JSON from the database matching the instructor with the instructor id
* http://localhost:8080/api/instructors responds with JSON from the database matching the list of instructors
* http://localhost:8080/api/terms responds with JSON from the database matching the list of terms

These endpoints were implemented, as some of them are required for front-end to function.

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

## Reflection

### What Worked

The documentation for Actix was surprisingly extensive.
From my brief experience with Rust, documentation seems to be a sore point, but that is not the case with Actix.
With enough motivation it was possible to find answers to seemingly unique problems that I came across.

`serde` was also a pleasure to work with.
It feels like a truly robust package, with a lot of functionality and good documentation.

### What Didn't Work

There was a substantial amount of time invested in Yew, but the results are not what were intended.
Ideally, I would have liked to have replaced the existing front-end with pure Yew.
However, there were some issues having Yew fetch responses from the back-end that included `Optional` types; they simply wouldn't parse.

In general, the experience of debugging Rust is not a pleasurable one.
From my brief experience, a lot of the code can't be stepped through, which makes it difficult to quickly overcome issues.

### Future Improvements

It would be nice to fully replace the front-end with pure Yew.
That seems possible, provided that the issues mentioned in the previous sections are overcome.

Another potential enhancement is flattening the server and the client together, and reduce some duplication especially regarding the data models.