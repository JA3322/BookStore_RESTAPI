use crate::models::{Author};
use crate::{db::Database};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::{Error};


#[async_trait]
pub trait AuthorDataTrait {
	async fn get_all_authors(db: &Data<Database>) -> Option<Vec<Author>>;
	async fn add_author_to_db(db: &Data<Database>, new_author: Author) -> Option<Author>;
	async fn update_author(db: &Data<Database>, updated_author: Author) -> Option<Author>;
}

#[async_trait]
impl AuthorDataTrait for Database {

	async fn get_all_authors(db: &Data<Database>) -> Option<Vec<Author>> {
		let result = db.client.select("author").await;
		match result {
			Ok(all_author) => Some(all_author),
			Err(_) => None,
		}
	}

	async fn add_author_to_db(db: &Data<Database>, new_author: Author) -> Option<Author> {
		let added_author = db
			.client
			.create(("author", new_author.uuid.clone()))
			.content(new_author)
			.await;

		match added_author {
			Ok(added) => added,
			Err(_) => None,
		}
	}

	async fn update_author(db: &Data<Database>, updated_author: Author) -> Option<Author> {
		let find_author : Result<Option<Author>, Error> = db.client.select(("author", &updated_author.uuid)).await;

		match find_author {
			Ok(found) => {
				match found {
					Some(_found_author) => {
						let updated_content = db.client.update(("author", &updated_author.uuid))
						.merge( Author {
							uuid: updated_author.uuid,
							author_name: updated_author.author_name,
						}).await;
						match updated_content {
							Ok(updated) => updated,
							Err(_) => None
						}
					},
					None => None,
				}
			},
			Err(_) => None,
		}
	}
}