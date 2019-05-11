use actix_web::{fs, App, Error, HttpRequest, Path, Responder, server, State, HttpResponse, http, error};
use serde_derive::*;
use tera::{Context, Tera, compile_templates};

fn hello<S>(_req: &HttpRequest<S>) -> impl Responder {
    "Hello World!"
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

fn hello_name(to: Path<HelloPath>) -> impl Responder {
    format!("Hello {}!", &to.name)
}

//fn hello_name(req: &HttpRequest) -> Result<String, Error> {
//    let to = Path::<HelloPath>::extract(req)?;
//    Ok(format!("Hello {}!", &to.name))
//}
//

struct MyApp {
    server_name: String,
    template: Tera,
}

fn hello_with_state(app: State<MyApp>) -> Result<String, Error> {
    Ok(format!("Hello from {}!", &app.server_name))
}

fn hello_template(
    app: State<MyApp>,
    path: Path<HelloPath>,
) -> Result<HttpResponse, error::Error> {
    let mut context = Context::new();
    context.insert("name", &path.name);

    let body = app
        .template
        .render("index.html.tera", &context)
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;

    Ok(HttpResponse::Ok().body(body))
}

fn main() {
    server::new(|| {
        App::with_state(MyApp {
            server_name: "server with state".to_string(),
            template: compile_templates!("templates/**/*"),
        })
            .resource("/", |r| r.f(hello))
            .resource("/state", |r| r.with(hello_with_state))
            .handler("/fs", fs::StaticFiles::new(".").unwrap())
            .route("/template/{name}", http::Method::GET, hello_template)
            .resource("/{name}", |r| r.with(hello_name))
    })
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}
