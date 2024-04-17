use crate::models::{Book, Author};
use crate::{db::Database};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::{Error};


#[async_trait]
pub trait BookDataTrait {
	async fn get_all_books(db: &Data<Database>) -> Option<Vec<Book>>;
	async fn add_book_to_db(db: &Data<Database>, new_book: Book) -> Option<Book>;
	async fn update_book(db: &Data<Database>, updated_book: Book) -> Option<Book>;
}

#[async_trait]
impl BookDataTrait for Database {

	async fn get_all_books(db: &Data<Database>) -> Option<Vec<Book>> {
		let result = db.client.select("book").await;
		match result {
			Ok(all_book) => Some(all_book),
			Err(_) => None,
		}
	}

	async fn add_book_to_db(db: &Data<Database>, new_book: Book) -> Option<Book> {
		
		let find_author : Result<Option<Author>, Error> = db.client.select(("author", &new_book.author_uuid)).await;

		let added_book = db
			.client
			.create(("book", new_book.uuid.clone()))
			.content(new_book)
			.await;

		match find_author {
			Ok(author) => {
				match added_book {
					Ok(added) => added,
					Err(_) => None,
				}
			},
			Err(_) => None,
		}
	}

	async fn update_book(db: &Data<Database>, updated_book: Book) -> Option<Book> {

		let find_author : Result<Option<Author>, Error> = db.client.select(("author", &updated_book.author_uuid)).await;

		let find_book : Result<Option<Book>, Error> = db.client.select(("book", &updated_book.uuid)).await;

		match find_author {			
			Ok(author) => {
				match find_book {
				Ok(found) => {
					match found {
						Some(_found_book) => {
							let updated_content = db.client.update(("book", &updated_book.uuid))
							.merge( Book {
								uuid: updated_book.uuid,
								book_name: updated_book.book_name,
								book_genre: updated_book.book_genre,
								book_rating: updated_book.book_rating,
								author_uuid: updated_book.author_uuid,
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
			},
			Err(_) => None,
		}


		
	}
}