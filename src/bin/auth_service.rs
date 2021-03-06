use env_logger::Env;
use ostrich_api::{
    config::Config, get_user_service_health, handle_rejection, handlers, routes, services,
    with_config,
};
use std::{net::SocketAddr, sync::Arc};
use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("🔍 Booting up the Authentication Service!");

    let config = Arc::new(Config::new(false));
    let cognito =
        Arc::new(services::cognito::get_cognito_client(config.clone().aws_region.clone()).await);

    let auth = routes::auth::login(config.clone(), cognito.clone())
        .and_then(handlers::auth::login)
        .or(routes::auth::sign_up(config.clone(), cognito.clone())
            .and_then(handlers::auth::sign_up)
            .or(routes::auth::verify(config.clone(), cognito.clone())
                .and_then(handlers::auth::verify)
                .or(routes::auth::resend_code(config.clone(), cognito.clone())
                    .and_then(handlers::auth::resend_code))))
        .recover(handle_rejection);

    let with_control_origin = warp::reply::with::header("Access-Control-Allow-Origin", "*");
    let with_content_allow =
        warp::reply::with::header("Access-Control-Allow-Headers", "Content-Type");

    let end = warp::get()
        .and(warp::path("health"))
        .and(with_config(config.clone()))
        .and_then(get_user_service_health)
        .or(auth)
        .with(with_control_origin)
        .with(with_content_allow)
        .with(warp::log("auth"));

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    log::info!("Listening at {}", &config.app_addr);

    if config.clone().tls {
        log::info!("TLS Enabled!");

        warp::serve(end)
            .tls()
            .cert_path(config.clone().cert_path.as_ref().unwrap())
            .key_path(config.clone().key_path.as_ref().unwrap())
            .run(socket_address)
            .await;
    } else {
        warp::serve(end).run(socket_address).await;
    }
}
