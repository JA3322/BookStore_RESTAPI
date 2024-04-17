use actix_web :: {
	http :: {header::ContentType, StatusCode},
	HttpResponse, ResponseError
};

use derive_more::Display;

#[derive(Display, Debug)]
pub enum AuthorError {
	NoSuchAuthorError = 0,
	AuthorAddError = 1,
	NoAuthorFoundError = 2,	
}

impl ResponseError for AuthorError {
	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::json())
			.body(self.to_string())
	}

	fn status_code(&self) -> StatusCode {
		match self {
			AuthorError::NoAuthorFoundError => StatusCode::NOT_FOUND,
			AuthorError::AuthorAddError => StatusCode::INTERNAL_SERVER_ERROR,
			AuthorError::NoSuchAuthorError => StatusCode::NOT_FOUND,
		}
	}
}