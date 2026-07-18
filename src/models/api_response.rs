use serde::Serialize;

#[derive (Serialize)]

pub struct ApiResponse {
    
    pub success : bool,
    pub message : String,
}