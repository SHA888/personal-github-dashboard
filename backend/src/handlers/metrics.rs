// TODO: Implement metrics handler module for repository analytics endpoints.

use actix_web::{get, HttpResponse, Responder};
use metrics_exporter_prometheus::PrometheusHandle;
use once_cell::sync::OnceCell;

static PROMETHEUS_HANDLE: OnceCell<PrometheusHandle> = OnceCell::new();

pub fn set_prometheus_handle(handle: PrometheusHandle) {
    let _ = PROMETHEUS_HANDLE.set(handle);
}

#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    let metrics = PROMETHEUS_HANDLE
        .get()
        .map(|h| h.render())
        .unwrap_or_default();
    HttpResponse::Ok().content_type("text/plain").body(metrics)
}
