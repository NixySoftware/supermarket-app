#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body, extract::State, http::Request, response::IntoResponse, routing::get, Router,
    };
    use axum_login::tower_sessions::cookie::SameSite;
    use axum_login::tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
    use axum_login::AuthManagerLayerBuilder;
    use dotenvy::dotenv;
    use leptos::*;
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    use supermarket_web::app::*;
    use supermarket_web::auth::Backend;
    use supermarket_web::database::get_database_connection;
    use supermarket_web::fileserv::file_and_error_handler;
    use supermarket_web::state::AppState;

    async fn server_fns_handler(
        State(app_state): State<AppState>,
        req: Request<Body>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            move || {
                provide_context(app_state.clone());
            },
            req,
        )
        .await
    }

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    dotenv().ok();

    let database = get_database_connection().await.unwrap();

    let session_store = MemoryStore::default();
    let session_layer: SessionManagerLayer<MemoryStore> = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(
            axum_login::tower_sessions::cookie::time::Duration::days(1),
        ))
        .with_http_only(true)
        .with_secure(true)
        .with_same_site(SameSite::Lax);

    let backend = Backend::new(database.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app_state = AppState {
        leptos_options,
        database,
    };
    let app_state_context = app_state.clone();

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fns_handler).post(server_fns_handler),
        )
        .leptos_routes_with_context(
            &app_state,
            routes,
            move || {
                provide_context(app_state_context.clone());
            },
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(app_state)
        .layer(auth_layer);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
