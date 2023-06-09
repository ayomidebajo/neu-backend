use r2d2_postgres::PostgresConnectionManager;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde_derive::Deserialize;
use serde_json::{Result as SerdeResult, Value};
use std::io::BufReader;
use std::{
    fs::File,
    io::{self, BufRead},
};

// Title: Rust Postgres
use clap::{arg, Arg, ArgAction, Command, Parser};
use postgres::{Client, Error, NoTls};

//  TODO: Change API

#[derive(Deserialize, Debug)]
pub struct User {
    name: String,
    email: String,
    // password: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            // password,
        }
    }
}

fn main() -> Result<(), failure::Error> {
    const CMD_CREATE: &str = "create";
    const CMD_ADD: &str = "add";
    const CMD_LIST: &str = "list";
    const CMD_IMPORT: &str = "import";

    let matches = Command::new("neu-backend")
        .version("0.1.0")
        .author("Neu Team <hello.neu@gmail.com>")
        .arg(
            Arg::new("database")
                .long("db")
                .value_name("ADDR")
                .help("Sets an address of db connection")
                .required(true),
        )
        .subcommand(Command::new(CMD_CREATE).about("create users table"))
        .subcommand(
            Command::new(CMD_ADD)
                .about("add user to the table")
                .arg(
                    Arg::new("name")
                        .help("Sets the name of a user")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("email")
                        .help("Set the email of a user")
                        .index(2)
                        .required(true),
                ),
        )
        .subcommand(Command::new(CMD_LIST).about("print list of users"))
        .subcommand(
            Command::new(CMD_IMPORT)
                .about("import users from json file")
                .arg(Arg::new("name").help("add file").value_name("FILE").required(true)),
        )
        .get_matches();

    let addr = matches.clone();
    let addr = addr.get_one::<String>("database").unwrap();
    let manager = PostgresConnectionManager::new(addr.parse().unwrap(), NoTls);
    // let mut conn = Client::connect(&addr, NoTls).unwrap();
    let pool = r2d2::Pool::new(manager)?;
    let mut conn = pool.get()?;

    match matches.subcommand() {
        Some((CMD_CREATE, _)) => {
            create_table(&mut conn)?;
            // create_table(&mut conn).unwrap();
        }
        Some((CMD_ADD, matched)) => {
            let name = matched.get_one::<String>("name").unwrap().to_owned();
            let email = matched.get_one::<String>("email").unwrap().to_owned();
            let user = User { name, email };
            create_user(&mut conn, &user)?;
        }
        Some((CMD_LIST, _)) => {
            println!("list");
            let users = list_users(&mut conn)?;
            for user in users {
                println!("Name: {:20}    Email: {:20}", user.name, user.email);
            }
        }
        Some((CMD_IMPORT, matched)) => {
            let name = matched.get_one::<String>("name").unwrap();

            let file = File::open(name).expect("error opening file");

            let reader = BufReader::new(file);

            // Deserialize the JSON data
            let data: Value = serde_json::from_reader(reader).expect("error while reading json");

            // Iterate through the JSON object
            if let Some(items) = data.as_array() {
                let mut user_collections: Vec<(&str, &str)> = Vec::new();
                user_collections
                    .par_iter()
                    .map(|item| {
                        let mut conn = pool.get().expect("error getting connection");
                        let name = item.0;
                        let email = item.1;

                        let user = User {
                            name: name.to_string(),
                            email: email.to_string(),
                        };

                        create_user(&mut conn, &user)
                    })
                    .for_each(drop);
                for item in items {
                    // Access individual fields or values within each item
                    let name = item.get("name").expect("error getting name");
                    let email = item.get("email").expect("error getting email");

                    //  push to vector
                    user_collections.push((name.as_str().unwrap(), email.as_str().unwrap()));
                }

                for (name, email) in user_collections {
                    let user = User::new(name.to_string(), email.to_string());
                    create_user(&mut conn, &user)?;
                }
            }
        }
        _ => {
            println!("no subcommand");
        }
    }

    Ok(())
}

fn create_table(conn: &mut Client) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE users ( id SERIAL PRIMARY KEY, name VARCHAR NOT NULL, email VARCHAR UNIQUE )",
        &[],
    )
    .map(drop)
}

// This function demonstrates how to insert data into the database.
// TODO: return the id of the inserted row or the email address so that we can send the user a confirmation email.
fn create_user(conn: &mut Client, user: &User) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.email, &user.email],
    )
    .map(drop)
}

fn list_users(conn: &mut Client) -> Result<Vec<User>, Error> {
    let res = conn
        .query("SELECT name, email FROM users", &[])?
        .into_iter()
        .map(|row| {
            User {
                name: row.get(0),
                email: row.get(1),
                //    password: "".to_string(),
            }
        })
        .collect();
    Ok(res)
}
