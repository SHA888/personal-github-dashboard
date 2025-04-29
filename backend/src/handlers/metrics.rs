// TODO: Implement metrics handler module for repository analytics endpoints.

use actix_web::{HttpResponse, Responder, get};
use metrics_exporter_prometheus::PrometheusHandle;
use once_cell::sync::OnceCell;

static PROMETHEUS_HANDLE: OnceCell<PrometheusHandle> = OnceCell::new();

/// Sets the global Prometheus metrics handle if it has not already been set.
///
/// Subsequent calls have no effect if the handle is already initialized.
pub fn set_prometheus_handle(handle: PrometheusHandle) {
    let _ = PROMETHEUS_HANDLE.set(handle);
}

#[get("/metrics")]
/// Handles HTTP GET requests to the `/metrics` endpoint, returning Prometheus metrics data.
///
/// If the Prometheus metrics handle has been initialized, this returns the current metrics as plain text.
/// Otherwise, it returns an empty response body.
///
/// # Examples
///
/// ```
/// // Register the handler in your Actix-web app:
/// // app.route("/metrics", web::get().to(metrics));
/// // Then, a GET request to /metrics returns Prometheus metrics.
/// ```
pub async fn metrics() -> impl Responder {
    let metrics = PROMETHEUS_HANDLE
        .get()
        .map(|h| h.render())
        .unwrap_or_default();
    HttpResponse::Ok().content_type("text/plain").body(metrics)
}
