use serde::{Serialize, Deserialize};

#[derive (Serialize, Deserialize)]

pub struct Claims {
    pub sub :i32,
    pub exp :usize,
}