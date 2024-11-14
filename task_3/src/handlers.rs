use super::state::State;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use warp::reply;
use warp::reply::Response;
use warp::Reply;

pub async fn handle_sync(
    unique_id: String,
    state: Arc<State>,
) -> Result<Response, warp::Rejection> {
    let notifier = {
        let mut sync_points = state.sync_points.lock().unwrap();
        // Insert a new Notify instance wrapped in Arc if it doesn't exist
        sync_points
            .entry(unique_id.clone())
            .or_insert_with(|| Arc::new(tokio::sync::Notify::new()))
            .clone()
    };


    // Wait for the second party for up to 10 seconds
    match timeout(Duration::from_secs(30), notifier.notified()).await {
        Ok(()) => {
            let mut sync_points = state.sync_points.lock().unwrap();
            sync_points.remove(&unique_id); // Remove the entry after notifying
            Ok(reply::with_status("Synchronized!", warp::http::StatusCode::OK).into_response())
        }
        Err(_) => {
            let mut sync_points = state.sync_points.lock().unwrap();
            sync_points.remove(&unique_id); // Remove the entry after timeout
            Ok(reply::with_status(
                "Timeout: Second party did not arrive.",
                warp::http::StatusCode::GATEWAY_TIMEOUT,
            )
            .into_response())
        }
    }
}