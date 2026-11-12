use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;

#[derive(Clone)]
pub struct ServerState {
    pub greeting: String,
}

#[derive(Serialize)]
struct Health {
    ok: bool,
    greeting: String,
}

pub fn router(state: ServerState) -> Router {
    Router::new()
        .route("/health", get(health))
        .with_state(state)
}

async fn health(State(state): State<ServerState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Health {
            ok: true,
            greeting: state.greeting,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use axum::http::Request;
    use tower::ServiceExt;

    fn test_router() -> Router {
        router(ServerState {
            greeting: "hi".to_string(),
        })
    }

    #[tokio::test]
    async fn health_reports_ok_and_greeting() {
        let response = test_router()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["ok"], true);
        assert_eq!(body["greeting"], "hi");
    }

    #[tokio::test]
    async fn unknown_route_is_not_found() {
        let response = test_router()
            .oneshot(Request::builder().uri("/nope").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
