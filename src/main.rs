// Title: Rust Postgres
   use postgres::{Client, NoTls, Error};
   use clap::{Parser, Command, ArgAction, Arg};

// #[derive(Parser, Debug)]
// #[command(author, about, version, long_about, name = "Neu Backend")]
// struct Args {
//     add: String,
//     create: String, 
// }

fn main() -> Result<(), Error>{
    

   const CMD_CREATE: &str = "create";
   const CMD_ADD: &str = "add";
   const CMD_LIST: &str = "list";


  let  matches = Command::new("neu-backend")
           .version("0.1.0")
           .author("Neu Team <hello.neu@gmail.com>")
           .arg(
               Arg::new("database")
               .long("db")
               .value_name("ADDR")
               .help("Sets an address of db connection").required(true)
               )
           .subcommand(Command::new(CMD_CREATE).about("create users table"))
           .subcommand(Command::new(CMD_ADD).about("add user to the table")
                       .arg(Arg::new("name")
                            .help("Sets the name of a user")
                            .index(1)
                            .required(true))
                       .arg(Arg::new("email")
                            .help("Sets the email of a user")
                            .index(2)
                            .required(true)
                         ))
           .subcommand(Command::new(CMD_LIST).about("print list of users"))
           .get_matches();

        let addr = matches.clone();
        let addr = addr.get_one::<String>("database").unwrap();
    let mut conn = Client::connect(&addr,
   NoTls).unwrap();


        match matches.subcommand() {
            Some((CMD_CREATE, _)) => {
                create_table(&mut conn)?;
                // create_table(&mut conn).unwrap();
            },
            Some((CMD_ADD, matched)) => {
           let name = matched.get_one::<String>("name").unwrap();
           let email = matched.get_one::<String>("email").unwrap();
           create_user(&mut conn, name, email)?;
            },
            Some((CMD_LIST, _)) => {
                println!("list");
                let users = list_users(&mut conn)?;
                // let users = list_users(&mut conn).unwrap();
                for user in users {
                    println!("user: {:?}", user);
                }
            },
            _ => {
                println!("no subcommand");
            },
        }

        Ok(())
    // println!("out: {}", out);
}

fn create_table(conn: &mut Client) -> Result<(), Error> {
       conn.execute("CREATE TABLE users ( id SERIAL PRIMARY KEY, name VARCHAR NOT NULL, email VARCHAR UNIQUE )", &[])
           .map(drop)
}
// This function demonstrates how to insert data into the database.
// TODO: return the id of the inserted row or the email address so that we can send the user a confirmation email.
fn create_user(conn: &mut Client, name: &str, email: &str) -> Result<(), Error> {
       conn.execute("INSERT INTO users (name, email) VALUES ($1, $2)",
                    &[&name, &email])
           .map(drop)
}


fn list_users(conn: &mut Client) -> Result<Vec<(String, String)>, Error> {
       let res = conn.query("SELECT name, email FROM users", &[])?.into_iter()
           .map(|row| (row.get(0), row.get(1)))
           .collect();
       Ok(res)
}
