pub mod db;
pub mod error;
pub mod models;
pub mod helpers;
pub mod routes;
use clap::{Arg, Command};
use dotenv::dotenv;
use neu_backend::config::get_configuration;
use neu_backend::run;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde_json::Value;
use sqlx::PgPool;
use std::io::BufReader;
use std::net::TcpListener;
use std::thread;
use std::{env, error as std_error, fs::File};

use crate::error::MyError;

//  Create login endpoint (JWT) for users
//  Create register endpoint for users

fn import() -> Result<(), impl std_error::Error> {
    const CMD_CREATE_TABLE: &str = "create_table";
    // const CMD_CREATE_MERCHANTS_TABLE: &str = "create_merchants_table";
    const CMD_ADD: &str = "add";
    const CMD_LIST: &str = "list";
    const CMD_IMPORT_USERS: &str = "import_users";

    let matches = Command::new("neu-backend")
        .version("0.1.0")
        .author("Neu Team <hello.neu@gmail.com>")
        .subcommand(Command::new(CMD_CREATE_TABLE).about("create table"))
        .subcommand(
            Command::new(CMD_ADD)
                .about("add user to the table")
                .arg(
                    Arg::new("email")
                        .help("Sets the name of a user")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("password")
                        .help("Set the email of a user")
                        .index(2)
                        .required(true),
                ),
        )
        .subcommand(Command::new(CMD_LIST).about("print list of users"))
        .subcommand(
            Command::new(CMD_IMPORT_USERS)
                .about("import users from json file")
                .arg(
                    Arg::new("name")
                        .help("add file")
                        .value_name("FILE")
                        .required(true),
                ),
        )
        .get_matches();

    let addr = env::var("DATABASE_URL").expect("DB must be set");
    let manager = PostgresConnectionManager::new(addr.parse().unwrap(), NoTls);
    let pool = r2d2::Pool::new(manager).expect("error creating pool");
    let mut conn = pool.get().expect("error creating connection");

    match matches.subcommand() {
        Some((CMD_CREATE_TABLE, _)) => {
            match db::create_table(&mut conn) {
                Ok(_) => println!("table created"),
                Err(e) => println!("error creating table: {}", e),
            };
        }
        Some((CMD_ADD, matched)) => {
            let email = matched.get_one::<String>("email").unwrap().to_owned();
            let password = matched.get_one::<String>("password").unwrap().to_owned();
            let user = db::User::new("placeholder name".to_owned(), email, password);
            match db::login_user(&mut conn, &user) {
                Ok(e) => println!("logging in user: {:?}", e),
                Err(e) => println!("error logging in user {}", e),
            }
        }
        Some((CMD_LIST, _)) => {
            println!("list");
            // let users = list_users(&mut conn)?;

            match db::list_users(&mut conn) {
                Ok(users) => {
                    for user in users {
                        println!("Name: {:20}    Email: {:20}", user.name, user.email);
                    }
                }
                Err(e) => println!("error listing users: {}", e),
            }
        }
        Some((CMD_IMPORT_USERS, matched)) => {
            let name = matched.get_one::<String>("name").unwrap();

            let file = File::open(name).expect("error opening file");

            let reader = BufReader::new(file);

            // Deserialize the JSON data
            let data: Value = serde_json::from_reader(reader).expect("error reading json file");

            // Iterate through the JSON object
            if let Some(items) = data.as_array() {
                let mut user_collections: Vec<(&str, &str)> = Vec::new();

                for item in items {
                    // Access individual fields or values within each item
                    let name = item.get("name").expect("error getting name");
                    let email = item.get("email").expect("error getting email");

                    //  push to vector
                    user_collections.push((name.as_str().unwrap(), email.as_str().unwrap()));
                }
                user_collections
                    .par_iter()
                    .map(|item| {
                        let mut conn = pool.get().expect("error getting connection");
                        let name = item.0;
                        let email = item.1;

                        let user = db::User::new(
                            name.to_string(),
                            email.to_string(),
                            "password".to_string(),
                        );

                        db::create_user(&mut conn, &user)
                    })
                    .for_each(drop);

                println!("imported users");
            }
        }
        _ => println!("no subcommand, will continue to run as a web server"),
    }

    Ok::<(), crate::MyError>(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // this thread is needed to run the blocking function `import` for importing the data into the db
    // Might change to use tokio::spawn_blocking instead
    thread::spawn(|| {
        import().expect("expected a command at least");
    })
    .join()
    .expect("thread errror");
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = TcpListener::bind("127.0.0.1:0")?;
    let port = address.local_addr().unwrap().port();

    let random_addr = format!("http://127.0.0.1:{}", port);
    println!("listening on {}", random_addr);

    let postgres_conn = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(address, postgres_conn)?.await
}
