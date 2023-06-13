# How to run

## Install dependencies
`cargo install`

## add a user to db
`cargo run add user-1 user-1@example.com`

## initialize db
`cargo run create_table`

## import users from json file
`cargo run import_users temp-db.json`

## list users
`cargo run list`


## docker and postgres

### create db
`export DATABASE_URL=postgres://postgres:password@localhost:5432/neu_db`

### migrate db
for example `sqlx migrate add create_subscriptions_table`

then run `sqlx migrate run`

