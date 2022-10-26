use actix_web::dev::ServerHandle;
use actix_web::{HttpResponse, post, web};
use actix_web_lab::extract::Path;
use parking_lot::Mutex;


#[post("/stop/{graceful}")]
pub async fn stop(Path(graceful): Path<bool>, stop_handle: web::Data<StopHandle>) -> HttpResponse {
    stop_handle.stop(graceful);
    HttpResponse::NoContent().finish()
}

#[derive(Default)]
pub struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
}


impl StopHandle {
    /// Sets the server handle to stop.
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    /// Sends stop signal through contained server handle.
    pub(crate) fn stop(&self, graceful: bool) {
        let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    }
}