use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct AddAuthorRequest {
	#[validate(length(min=1, message="Enter Author name"))]
	pub author_name : String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateAuthorURL {
	pub uuid: String,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Author {
	pub uuid: String,
	#[validate(length(min=1, message="Enter Author name"))]
	pub author_name: String,
}

impl Author {
	pub fn new(uuid: String, author_name: String) -> Author {
		Author { uuid, author_name }
	}
}