// TODO: Implement metrics handler module for repository analytics endpoints.

use actix_web::{get, HttpResponse, Responder};
use metrics_exporter_prometheus::PrometheusHandle;

static mut PROMETHEUS_HANDLE: Option<PrometheusHandle> = None;

pub fn set_prometheus_handle(handle: PrometheusHandle) {
    unsafe {
        PROMETHEUS_HANDLE = Some(handle);
    }
}

#[get("/metrics")]
pub async fn metrics() -> impl Responder {
    let metrics = unsafe {
        PROMETHEUS_HANDLE
            .as_ref()
            .map(|h| h.render())
            .unwrap_or_default()
    };
    HttpResponse::Ok().content_type("text/plain").body(metrics)
}
