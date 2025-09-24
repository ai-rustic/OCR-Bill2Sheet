use axum::extract::FromRef;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::{
    config::{ConnectionPool, UploadConfig},
    models::ProcessingEvent,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: ConnectionPool,
    pub upload_config: Arc<UploadConfig>,
    pub event_broadcaster: broadcast::Sender<ProcessingEvent>,
}

impl FromRef<AppState> for ConnectionPool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.pool.clone()
    }
}

impl FromRef<AppState> for Arc<UploadConfig> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.upload_config.clone()
    }
}

impl FromRef<AppState> for broadcast::Sender<ProcessingEvent> {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.event_broadcaster.clone()
    }
}
