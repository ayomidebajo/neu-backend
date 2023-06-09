# How to run

## Install dependencies
`cargo install`

## add a user to db
`cargo run -- --db postgres://postgres@localhost:5432 add user-1 user-1@example.com`

## create users table
`cargo run -- --db postgres://postgres@localhost:5432 create`

## import users from json file
`cargo run -- --db postgres://postgres@localhost:5432 import temp-db.json`

## list users
`cargo run -- --db postgres://postgres@localhost:5432 list`

