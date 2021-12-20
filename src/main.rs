use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

use std::sync::{Arc, Mutex};

mod room;
use room::Room;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_websockets=debug,tower_http=debug")
    }
    /* Get the global Room listings into an Arc Mutex starting with None room. */
    let mut rooms: Arc<Mutex<Option<Vec<Room>>>> = Arc::new(Mutex::new(None));
    assert_eq!(*rooms.lock().unwrap(), None); // Tests for None

    rooms = play_arounds(rooms);
    // Tests for 10 rooms
    if let Some(rcollection) = &*rooms.lock().unwrap() {
        assert_eq!(rcollection.len(), 10);
    }

    println!("{:?}", *rooms.lock().unwrap());

    tracing_subscriber::fmt::init();

    // build our application with some routes
    let app = Router::new()
        .route("/lcr", get(ws_handler)) // lcr - LogIn -> Create Room -> New Url.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 7777)); //Rafa morreira Mano.

    tracing::debug!("listening on {}", addr); // Further will be removed.

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn play_arounds(mut rooms: Arc<Mutex<Option<Vec<Room>>>>) -> Arc<Mutex<Option<Vec<Room>>>> {
    let pseudo_rooms = rooms.clone();
    for _tryout in 0..10 {
        if let Some(collection) = &mut *pseudo_rooms.lock().unwrap() {
            collection.push(crate::room::create_room::init(1, _tryout));
        } else {
            rooms = Arc::new(Mutex::new(Some(Vec::<Room>::new())));
            return play_arounds(rooms);
        };
    }
    pseudo_rooms
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        println!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            println!("Client says: {:?}", msg);
        } else {
            println!("client disconnected");
            return;
        }
    }
}
