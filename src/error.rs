use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;



pub enum AppError {
    NotFound(String),
    Unauthorized(String),
    DublicateKey,
    DatabaseError(String),
    InternalServerError(String),
    WrongPassword,
    Forbidden(String),
    BadRequest(String),
    Conflict(String)
}

#[derive(Serialize)]
struct ApiError {

    message : String,
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {

        match &value {

            sqlx::Error::Database(db_error) => {

                if db_error.code().as_deref() == Some("23505") {
                    return AppError::DublicateKey
                }else {
                    return AppError::DatabaseError(db_error.to_string())
                } 
            }
            _ => {
                return AppError::DatabaseError(value.to_string())
            }

        }
        
    }
}


impl IntoResponse for AppError {

    fn into_response(self) -> axum::response::Response {

      
        let (status_code,message) = match self {

            Self::NotFound(s) => {

                (
                    StatusCode::NOT_FOUND,
                    s,
              
                )

            },
            Self::Unauthorized(s) => {

                (
                    StatusCode::UNAUTHORIZED,
                   s,
               
                )

            },
            Self::DublicateKey => {

                (
                    StatusCode::BAD_REQUEST,
                    String::from("dublicate value found in database"),
                 
                )

            },
            Self::DatabaseError(s) => {

                println!("databse error {}",s);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("database error"),
               
                )

            },
            Self::InternalServerError(s ) => {

                println!("internal server error {}",s);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("internal server error"),
      
                )

            },
            Self::WrongPassword => {

                (
                    StatusCode::BAD_REQUEST,
                    String::from("password is wrong"),
   
                )

            },
            Self::BadRequest(s) => {
                (
                    StatusCode::BAD_REQUEST,
                    s,
   
                )
            },
            Self::Conflict(s) => {
                (
                    StatusCode::CONFLICT,
                    s,
                )
            },
            Self::Forbidden(s) => {
                (
                    StatusCode::FORBIDDEN,
                    s
                )
            }

        };
       
        let mut response = Json(ApiError{
            message : message.to_string()
        }).into_response();

        *response.status_mut() = status_code;
        response
       
        
    }

}