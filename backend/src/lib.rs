pub mod api; // Previously routes
pub mod config;
pub mod db; // Previously models
pub mod github; // Previously services/github
pub mod middleware;
pub mod services; // Includes analytics and sync
pub mod state;
pub mod utils;
pub mod websocket;

pub use state::AppState;
