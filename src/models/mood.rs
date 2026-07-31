use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoodRequest {
    pub text: String,
}