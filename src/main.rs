use std::{io, sync::Arc};
use juniper::http::GraphQLRequest;
use crate::schema::{create_schema, Schema};
use actix_web::{middleware, route, App, HttpResponse, HttpServer, Responder};
use actix_web::web::{Data, Json};

mod schema;

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: Data<Schema>, data: Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}


#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = Arc::new(create_schema());

    log::info!("starting HTTP server on port 3000");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .wrap(middleware::Logger::default())
    })
        .workers(2)
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}