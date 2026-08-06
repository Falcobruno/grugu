use serde::{Serialize, Deserialize};

#[derive (Deserialize)]
pub struct User {
   pub name : String,
   pub age : u8,
   pub relationship_years: u8,
   pub password: String,

}

#[derive (Serialize)]
pub struct PublicUser{
    pub name :String,
    pub age :u8,
    pub relationship_years: u8,
}

#[derive (Deserialize)]
pub struct LoginRequest{
    pub name :String,
    pub password:String,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}