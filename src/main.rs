use actix_web::{web::{Path, Data}, get, patch, post, web::Json, HttpResponse, Responder, App, HttpServer};
use validator::Validate;
use uuid;

mod models;
mod db;
mod error;

use crate::models::author::{AddAuthorRequest, UpdateAuthorURL, Author};
use crate::error::author_error::AuthorError;


use crate::models::book::{AddBookRequest, UpdateBookURL, Book};
use crate::error::book_error::BookError;

use crate::db::{author_data_trait::AuthorDataTrait, book_data_trait::BookDataTrait, Database};


#[get("/authors")]
async fn get_authors(db: Data<Database>) -> Result<Json<Vec<Author>>, AuthorError> {
    
    let authors = Database::get_all_authors(&db).await;
    let authors = match authors {
        Some(found_author) => found_author,
        None => vec!(),
    };
    match authors.len() {
        0 => Err(AuthorError::NoAuthorFoundError),
        _ => { 
            println!("{:?}", authors);
            Ok(Json(authors))
        },    
    }
}


#[post("/add_author")]
async fn add_author(body : Json<AddAuthorRequest>, db : Data<Database>) -> Result<Json<Author>, AuthorError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let author_name = body.author_name.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizz = Database::add_author_to_db(&db, Author::new(
                String::from(new_uuid),
                author_name
            )).await;

            match new_pizz {
                Some(added_pizz) => {
                    Ok(Json(added_pizz))        
                },
                None => Err(AuthorError::AuthorAddError),        
            }

            
        },
        Err(_) => {
            Err(AuthorError::AuthorAddError)
        }
    }
}

#[patch("/update_a_author/{uuid}")]
async fn update_a_author(update_author_url: Path<UpdateAuthorURL>, db : Data<Database>) -> impl Responder {
    let uuid = update_author_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Author updated with uuid: {uuid}"))
}


#[post("/update_author_with_body")]
async fn update_author_body(update_author: Json<Author>, db : Data<Database>) -> Result<Json<Author>, AuthorError> {    
    let is_valid = update_author.validate();
    match is_valid {
        Ok(_) => {
            let update = Database::update_author(&db, 
                    Author {
                        uuid: update_author.uuid.clone(),
                        author_name: update_author.author_name.clone()
                    }
                ).await;
            Ok(update_author)
        },
        Err(_) => {
            Err(AuthorError::AuthorAddError)
        }
    }    
}

#[get("/books")]
async fn get_books(db: Data<Database>) -> Result<Json<Vec<Book>>, BookError> {
    
    let books = Database::get_all_books(&db).await;
    let books = match books {
        Some(found_book) => found_book,
        None => vec!(),
    };
    match books.len() {
        0 => Err(BookError::NoBookFoundError),
        _ => { 
            println!("{:?}", books);
            Ok(Json(books))
        },    
    }
}


#[post("/add_book")]
async fn add_book(body : Json<AddBookRequest>, db : Data<Database>) -> Result<Json<Book>, BookError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let book_name = body.book_name.clone();
            let book_genre = body.book_genre.clone();
            let book_rating = body.book_rating;
            let author_uuid = body.author_uuid.clone();
            let mut buffer = uuid::Uuid::encode_buffer();
            let new_uuid = uuid::Uuid::new_v4().simple().encode_lower(&mut buffer);

            let new_pizz = Database::add_book_to_db(&db, Book::new(
                String::from(new_uuid),
                book_name,
                book_genre,
                book_rating,
                author_uuid,
            )).await;

            match new_pizz {
                Some(added_pizz) => {
                    Ok(Json(added_pizz))        
                },
                None => Err(BookError::BookAddError),        
            }

            
        },
        Err(_) => {
            Err(BookError::BookAddError)
        }
    }
}


#[patch("/update_a_book/{uuid}")]
async fn update_a_book(update_book_url: Path<UpdateBookURL>, db : Data<Database>) -> impl Responder {
    let uuid = update_book_url.into_inner().uuid;
    HttpResponse::Ok().body(format!("Book updated with uuid: {uuid}"))
}


#[post("/update_book_with_body")]
async fn update_book_body(update_book: Json<Book>, db : Data<Database>) -> Result<Json<Book>, BookError> {    
    let is_valid = update_book.validate();
    match is_valid {
        Ok(_) => {
            let update = Database::update_book(&db, 
                    Book {
                        uuid: update_book.uuid.clone(),
                        book_name: update_book.book_name.clone(),
                        book_genre: update_book.book_genre.clone(),
                        book_rating: update_book.book_rating,
                        author_uuid: update_book.author_uuid.clone(),
                    }
                ).await;
            Ok(update_book)
        },
        Err(_) => {
            Err(BookError::BookAddError)
        }
    }    
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let db = Database::init()
    .await
    .expect("error connecting to database");

    //used by actix to wrap around your db
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_authors)
            .service(add_author)
            .service(update_a_author)
            .service(update_author_body)
            .service(get_books)
            .service(add_book)
            .service(update_a_book)
            .service(update_book_body)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}
