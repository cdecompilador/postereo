//! Shared data types across backend and frontend

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub sender: String,
    pub post_time: chrono::NaiveDateTime,
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PostBody {
    pub posts: Vec<Post>
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Error {
}
