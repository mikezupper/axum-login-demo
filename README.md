# Axum Login Demo

This demo web application was built using the Axum web framework and includes authentication and session management with the `axum_login` crate, as well as a SQLite database integration for user management.

## Table of Contents
1. [Getting Started](#getting-started)
2. [Code Structure](#code-structure)
3. [Authentication](#authentication)
4. [Database Integration](#database-integration)
5. [Routes and Handlers](#routes-and-handlers)
6. [Usage](#usage)
7. [Testing](#testing)

## Getting Started <a name="getting-started"></a>

To run the application, follow these steps:

1. Ensure you have Rust and Cargo installed. If not, you can install them using [Rustup](https://rustup.rs/). Versions used in this demo:
   - Cargo version: _cargo 1.73.0 (9c4383fb5 2023-08-26)_
   - Rustup version: _rustup 1.26.0 (5af9b9484 2023-04-05)_
   - Rustc version: _rustc 1.73.0 (cc66ad468 2023-10-03)_
   - Sqlite3 on linux _SQLite version 3.37.2_
3. Clone the repo
```shell
git clone https://github.com/mikezupper/axum-login-demos.git cd axum-login-demos
```
4. Create the Database
   1. create a folder called _sqlite_
      `mkdir sqlite`

   2. create the users database
      `sqlite3 sqlite/users.db`

   3. create a table to store all users
      `CREATE TABLE users (id int primary key not null,email text not null, password*hash text not null);`

   4. create a sample user.
      `insert into users (id,email,password*hash) values (1,'ferris@crab.domain','ferris');`

      `select * from users;`

5. Run the application using the following command:

   ```shell
   cargo run
   ```

## Code Structure <a name="code-structure"></a>

Here is an overview of the key components and their purpose:

- `main`: The entry point of the application. It sets up various layers, the database connection, and routes.

- `AuthContext`: A type alias representing the authentication context for users. It is used for user authentication and authorization.

- `AppState`: A struct that holds application state, including a connection pool to a SQLite database.

- `Login`: A struct used for deserializing login information from HTTP requests.

- `User`: A struct representing user data. It is also used as the type for user authentication with methods like `get_id` and `get_password_hash`.

- `SqliteStore`: A store for user data that integrates with an SQLite database. Many other DMBS vendors are supported (Postgress, MySql,etc...)

## Authentication <a name="authentication"></a>

Authentication is a significant part of the application, and it is achieved using the `axum_login` crate. Here's an overview of the authentication process:

1. A secret key is generated for securing sessions.

2. An in-memory session store is created and configured to use the secret key. The `with_secure(false)` method indicates that the sessions are not secure in this example.

3. An SQLite database connection pool is established for user management.

4. An authentication layer is created using the SQLite store and the secret key.

5. Routes and layers are defined to protect certain routes and handle user login and logout.

## Database Integration <a name="database-integration"></a>

The application integrates with an SQLite database for user management. The `sqlx` crate is used for database operations. Here's how the database integration works:

1. A connection pool to the SQLite database is created.

2. The `User` struct represents user data and is used for database queries.

3. The `SqliteStore` is used to manage user data in the database.

4. During the login process, the application queries the database to authenticate the user.

## Routes and Handlers <a name="routes-and-handlers"></a>

The application defines the following routes and their corresponding handlers:

- `/protected`: A route that is protected by authentication. It uses the `protected_handler` to display the user's email if logged in.

- `/login`: A route for handling user login. The `login_handler` validates user credentials and logs the user in if successful.

- `/logout`: A route for user logout. The `logout_handler` logs the user out.

## Usage <a name="usage"></a>

The application is designed as a basic web service with user authentication and session management. When you run the application, you can access it via a web browser or HTTP client. Here are some key interactions:

- Visit `/protected`: If you are logged in, it will display "Hello, [user email]". If not, it will require you to log in.

- Visit `/login`: You can log in using a form by providing an email and password.

- Visit `/logout`: This will log the user out.

The application serves static assets from the "assets" directory and includes a custom "NotFound.html" page if a requested asset is not found.

For more in-depth understanding, please refer to the code itself and the documentation of the libraries and crates used in the application.

## Testing <a name="testing"></a>

### Test Case: Basic

1. Open your browser to `http://localhost:3000`
2. You should see an index page with a email/password form.
3. The form is populated with the following values:
   - username: _ferris@crab.domain_
   - password: _ferris_
4. Click submit
5. You should see: _Hello, ferris@crab.domain_
6. Click logout
7. You should be redirected to the Login form.

### Test Case: protected url

1. Open your browser and go to `http://localhost:3000/protected`
2. You should see an HTTP 401 status code

### Test Case: Sneaky

1. Run the "Basic" test case as described above.
2. After returning to login page, Click back in your browser.
3. Observce what happens 👀👀
4. Pay special attention to your server console log

If you like the demo, follow me on Twitter/ X.com @mikezupper
