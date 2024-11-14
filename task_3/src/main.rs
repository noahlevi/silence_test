use warp::Filter;
use std::sync::Arc;
use state::State;
use handlers::handle_sync;

mod state;
mod handlers;

#[tokio::main]
async fn main() {
    let state = Arc::new(State::default());

    let sync_route = warp::path!("wait-for-second-party" / String)
        .and(warp::post())
        .and(with_state(state.clone()))
        .and_then(handle_sync);

    let routes = sync_route.with(warp::cors().allow_any_origin());

    // Run the server
    println!("Runing server on localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_state(
    state: Arc<State>,
) -> impl Filter<Extract = (Arc<State>,), Error = warp::Rejection> + Clone {
    warp::any().map(move || state.clone()).boxed()
}