use actix_web :: {
	http :: {header::ContentType, StatusCode},
	HttpResponse, ResponseError
};

use derive_more::Display;

#[derive(Display, Debug)]
pub enum BookError {
	NoSuchBookError = 0,
	BookAddError = 1,
	NoBookFoundError = 2,	
}

impl ResponseError for BookError {
	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
		HttpResponse::build(self.status_code())
			.insert_header(ContentType::json())
			.body(self.to_string())
	}

	fn status_code(&self) -> StatusCode {
		match self {
			BookError::NoBookFoundError => StatusCode::NOT_FOUND,
			BookError::BookAddError => StatusCode::INTERNAL_SERVER_ERROR,
			BookError::NoSuchBookError => StatusCode::NOT_FOUND,
		}
	}
}