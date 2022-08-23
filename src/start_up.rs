use std::{net::SocketAddr, sync::Arc};

use is_alive_middleware::IsAliveMiddleware;
use my_http_server::MyHttpServer;
use rust_extensions::ApplicationStates;

pub fn start_server(
    app_name: String,
    app_version: String,
    app_states: Arc<dyn ApplicationStates + Send + Sync + 'static>,
) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 8000)));

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(app_name, app_version)));
    http_server.start(app_states, my_logger::LOGGER.clone());
}
