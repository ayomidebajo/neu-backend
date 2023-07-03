# How to run

## Install dependencies
`cargo install`

## docker and postgres

##  running scripts
run this in your project dir first`chmod +x scripts/init_db.sh`
then do this too `./scripts/init_db.sh `

## docker
`docker build --tag neu-backend --file Dockerfile .`

### create db
`export DATABASE_URL=postgres://postgres:password@localhost:5432/neudb`

### migrate db
for example `sqlx migrate add create_subscriptions_table`

then run `sqlx migrate run`

