use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct AddBookRequest {
	#[validate(length(min=1, message="Enter Book name"))]
	pub book_name : String,

	#[validate(length(min=1, message="Enter Book Genre"))]
	pub book_genre : String,
	
	pub book_rating : u8,

	#[validate(length(min=1, message="Enter Author uuid"))]
	pub author_uuid : String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateBookURL {
	pub uuid: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Book {
	pub uuid: String,
	#[validate(length(min=1, message="Enter Book name"))]
	pub book_name: String,
	pub book_genre: String,
	pub book_rating: u8,
	pub author_uuid : String,
}

impl Book {
	pub fn new(uuid: String, book_name: String, book_genre: String, book_rating: u8, author_uuid: String) -> Book {
		Book { uuid, book_name, book_genre, book_rating, author_uuid }
	}
}