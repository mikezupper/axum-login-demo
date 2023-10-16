# Auxm Login Example

Using axum and the axum-login crate to show how to enable login to your rust web application. 

This is a demo application and should not be used in production. 

Security is a big deal and this example does not take in to consideration all the necessary steps to secure a web application.

## Assumptions

1. You system should have Rust installed.

   - Cargo version: _cargo 1.73.0 (9c4383fb5 2023-08-26)_
   - Rustup version: _rustup 1.26.0 (5af9b9484 2023-04-05)_
   - Rustc version: _rustc 1.73.0 (cc66ad468 2023-10-03)_

2. You should have Sqlite installed.

   - I tested this with sqlite3 on linux _SQLite version 3.37.2_

3. You are familiar with HTTP request/response. This includes cookies and server side sessions.

## Setup

`git clone https://github.com/mikezupper/axum-login-demos.git`
`cd axum-login-demos`

### Create DB

Step 1. create a folder called _sqlite_

`mkdir sqlite`

Step 2. create the users database

`sqlite3 sqlite/users.db`

Step 3. create a table to store all users

`CREATE TABLE users (id int primary key not null,email text not null, password*hash text not null);`

Step 4. create a sample user.

`insert into users (id,email,password*hash) values (1,'ferris@crab.domain','ferris');`

`select * from users;`

### Run

`cargo run`

## Verification

### Test Case: Basic

Open your browser to `http://localhost:3000`

You should see an index page with a email/password form.

The form is populated with username: _ferris@crab.domain_ and password: _ferris_

Click submit and then you should see

_Hello, ferris@crab.domain_

Click logout and you should see the Login form again.

### Test Case: protected url

Open your browser to `http://localhost:3000/protected`

You should get an HTTP 401 status code

### Test Case: Sneaky

Run the basic test case. When your are back on the Index login page, Click back in your browser and see what happens 👀👀

_pay special attention to your server console log_

Send me a message on Twitter/ X.com @mikezupper if you figure it out!
