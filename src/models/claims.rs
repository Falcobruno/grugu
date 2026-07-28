use serde::{Serialize, Deserialize};

#[derive (Serialize, Deserialize, Clone )]

pub struct Claims {
    pub sub :i32,
    pub exp :usize,
}