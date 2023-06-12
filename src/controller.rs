use postgres::{Client, Error};
use serde_derive::{Deserialize, Serialize};

pub async fn sign_up(conn: &mut Client, user: &User) -> Result<(), Error> {
	conn.execute(
		"INSERT INTO users (fname, lname, email, password) VALUES ($1, $2, $3, $4)",
		&[&user.fname, &user.lname, &user.email, &user.password],
	)
	.map(drop)
}