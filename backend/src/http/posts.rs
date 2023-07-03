use axum::routing::*;
use axum::extract::*;

use crate::app_state::AppState;
use crate::http::Result;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api/posts", get(posts))
}

async fn posts(
    State(AppState {
        ref db_pool,
        ..
    }): State<AppState>
) -> Result<Json<shared::PostBody>> {
    let posts = sqlx::query_as!(
        shared::Post,
        r#"
            select sender, post_time, message from post
        "#
    )
    .fetch_all(db_pool).await?;

    Ok(
        Json(shared::PostBody {
            posts
        })
    )
}
