use std::{net::SocketAddr, sync::Arc};

use my_http_server::MyHttpServer;
use rust_extensions::{ApplicationStates, Logger};

use super::is_alive_middleware::IsAliveMiddleware;

pub fn start_server(
    port: u16,
    app_name: String,
    app_version: String,
    app_states: Arc<dyn ApplicationStates + Send + Sync + 'static>,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
) {
    let mut http_server = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], port)));

    http_server.add_middleware(Arc::new(IsAliveMiddleware::new(app_name, app_version)));
    http_server.start(app_states, logger);
}
