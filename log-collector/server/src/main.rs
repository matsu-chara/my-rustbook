use actix_web::App;
use actix_web::http::Method;

mod handlers;

#[derive(Clone)]
pub struct Server {}

pub fn app(server: Server) -> App<Server> {
    use crate::handlers::*;

    let app: App<Server> = App::with_state(server)
        .route("/logs", Method::POST, handle_post_logs)
        .route("/csv", Method::POST, handle_post_csv)
        .route("/csv", Method::GET, handle_get_csv)
        .route("/logs", Method::GET, handle_get_logs);

    app
}

fn main() {
    env_logger::init();

    let server = Server{};
    actix_web::server::new(move || app(server.clone()))
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}
