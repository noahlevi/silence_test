use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

#[derive(Default)]
pub struct State {
    // Change Notify to Arc<Notify>
    pub sync_points: Mutex<HashMap<String, Arc<Notify>>>,
}