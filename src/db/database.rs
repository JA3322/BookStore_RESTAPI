use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::models::Author;
use crate::models::Book;

#[derive(Clone)]
pub struct Database {
	pub client: Surreal<Client>,
	pub name_space: String,
	pub db_name: String,
}

impl Database {
	pub async fn init() -> Result<Self, Error> {
		let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
		client.signin(Root {
			username: "root",
			password: "pwd_root"
		})
		.await?;
		client.use_ns("surreal").use_db("book_store").await.unwrap();
		Ok(Database {
			client,
			name_space: String::from("surreal"),
			db_name: String::from("book_store")
		})
	}

}