mod http_service;
use crate::{Handler, Result, Settings};
pub use http_service::HttpService;

pub trait CloseService {
    fn close_service(&mut self) -> Result<()>;
}

pub trait Service<H: Handler> {
    /// Blocking call that starts the service.
    fn start_service(&self, handler: H) -> Result<()>;
    /// starts the service in another thread
    fn spawn_service(&self, handler: H) -> Result<Box<dyn CloseService>>;
}

pub fn create_service<H: Handler + 'static>(settings: Settings) -> impl Service<H> {
    HttpService::new(settings)
}
