use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;



#[derive(Debug,Serialize)]
pub struct ApiResponse<T> {

    pub data : Option<T>,
    pub message : String,
}



impl<T> IntoResponse for ApiResponse<T>
where T : Serialize
 {

    fn into_response(self) -> axum::response::Response {

        (
            StatusCode::OK,
            Json(self)
        ).into_response()
        
    }

}