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
    let rooms: Arc<Mutex<Option<Vec<Room>>>> = Arc::new(Mutex::new(Some(Vec::<Room>::new())));
    //assert_eq!(*rooms.lock().unwrap(), None); // Stands for  a pseudo None (Some(Vec[])) in the first run to instanciate the allocator<T>
    play_arounds(rooms);
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

fn play_arounds(rooms: Arc<Mutex<Option<Vec<Room>>>>) {
    for _tryout in 0..=10 {
        /*
        First unwrap gaves me the mutex guard.
        The second one gives me the Option => None (As Vec<Room>)
        Maybe I need someway to do mmmmm a default? And the pushes?
        */
        if let Some(temporary) = &mut *rooms.lock().unwrap() {
            // unwrap or else.
            temporary.push(crate::room::create_room::init(1, _tryout));
        }
    }
    println!("{:?}", *rooms.lock().unwrap());
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
