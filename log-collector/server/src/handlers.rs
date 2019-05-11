use actix_web::{HttpResponse, Json, Query, State};
use actix_web::http::header;
use failure::Error;
use log::debug;

use crate::Server;
use crate::db;

/// POST /csv
pub fn handle_post_csv(server: State<Server>) -> Result<HttpResponse, Error> {
    let logs = Default::default();

    Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
}

/// POST /logs
pub fn handle_post_logs(
    server: State<Server>,
    log: Json<api::logs::post::Request>,
) -> Result<HttpResponse, Error> {
    use chrono::Utc;
    use crate::model::NewLog;

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc(),
    };
    let conn = server.pool.get()?;
    db::insert_log(&conn, &log)?;
    debug!("received log: {:?}", log);
    Ok(HttpResponse::Accepted().finish())
}


/// GET /logs
pub fn handle_get_logs(
    server: State<Server>,
    range: Query<api::logs::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);

    let logs = Default::default();

    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

/// GET /csv
pub fn handle_get_csv(
    server: State<Server>,
    range: Query<api::csv::get::Query>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", range);

    let csv: Vec<u8> = vec![];

    Ok(HttpResponse::Ok().set(header::ContentType(mime::TEXT_CSV)).body(csv))
}