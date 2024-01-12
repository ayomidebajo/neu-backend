# How to run

## Install dependencies
`cargo install`

## docker and postgres

<!-- ##  running scripts
run this in your project dir first`chmod +x scripts/init_db.sh`
then do this too `./scripts/init_db.sh ` -->

## docker
`docker compose build`

`docker compose up -d neu_server`


<!-- ### migrate db
for example `sqlx migrate add create_subscriptions_table`

then run `sqlx migrate run` -->

### run maker file
`make run` this will run the server
`make build` this will build docker deps and build the server
`make sqlx_prepare` this will run sqlx prepare command

