use serde::Serialize;

#[derive(Serialize)]

pub struct ApiStatus {
    pub name :String,
    pub status: String,
    pub version:String,
}