#![allow(dead_code, unused_doc_comments, unused_variables)]
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    http::Request,
    response::IntoResponse,
    routing::get,
    AddExtensionLayer, Router,
};
use hyper::http::Response;
use hyper::Body; // This allows applications to not use memory they don’t need, and allows exerting back-pressure on connections by only reading when asked.
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower::ServiceBuilder;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::TraceLayer;

mod room;
use room::*;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "example_websockets=debug,tower_http=debug")
    }
    /* Get the global Room listings into an Arc Mutex starting with None room. */
    let mut rooms: Arc<Mutex<Option<Vec<Room>>>> = Arc::new(Mutex::new(None));
    assert_eq!(*rooms.lock().unwrap(), None); // Tests for None

    rooms = Arc::new(Mutex::new(Some(vec![Room {
        client: 1,
        clerk: 2,
        id: 123_u64,
    }])));
    // rooms = play_arounds(rooms);
    // // Tests for 10 rooms
    // if let Some(rcollection) = &*rooms.lock().unwrap() {
    //     assert_eq!(rcollection.len(), 10);
    // }

    println!("{:?}", *rooms.lock().unwrap());

    tracing_subscriber::fmt::init();

    /**
    TODO:
    * * Implement a tower::service_fn closure with Returning Http codes over the Axum as Axum Valid Service
    * !Steps:  // Make Work then make it Good
    *  * First match the service Data Structure with multiple cases valids ( )
    *  * Switchs protocols to Https Protocols to WebSocketUpgrade - Maybe I'll need some redirect? ( )
    *  * dono
    *****/
    let room_service = tower::service_fn(|request: Request<Body>| async move {
        /**
         * !About -> Theres a Binding to be match, on the interop between this and Axum Services.
         *  * Maybe this should be service on a layer and not actually a route... ?
         *  * Maybe I could use .post_service to switch protocols?
         *
         */

        /**
         * !Atention -> I don't know the fuck I'm doing and this maybe (must) be wrong.
         * * Strongly looks like a fucking memory leak. Even the Box calls Leak (ノಠ益ಠ)ノ彡┻━┻
         */
        let room_info: &'static str = Box::leak(
            request
                .uri()
                .path()
                .split("/")
                .last()
                .unwrap()
                .to_string()
                .into_boxed_str(),
        );  // !TODO: Rewrite this when I discovers how to.

        let body_chunks: Vec<Result<&str, std::io::Error>> = vec![
            Ok("Hey there, either you don't have permissions to access the room or if you does, this room named: '"),
            Ok(room_info), 
            Ok("' does not exist anymore."),
        ];

        let body_stream = futures_util::stream::iter(body_chunks);

        let res = Response::builder()
                .status(417) // Expectation Failed, as always
                .body(Body::wrap_stream(body_stream))
                .expect("Unable to create `http::Response`");
        /**
         * !Ideas:
         * * Looks like "The response" don't matters alot? - Maybe just send over the body?
         * ! Notes :
         *  * The body Structure Comes from -> Hyper
         *  * The Response Comes from -> Hyper
         *  * Tower holds the closure
        //  * */

        //Ok::<_, Infallible>(Response::new(Body::empty())) // Why this works?
        Ok::<_, Infallible>(res) // And this not? Since the Type is the same?
    });

    // build our application with some routes
    let app = Router::new()
        .route("/room/:id", axum::routing::any_service(room_service))
        .route("/lcr", get(ws_handler))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::default().include_headers(true)),
                )
                .layer(AddExtensionLayer::new(rooms)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080)); //Rafa morreira Mano.

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
            collection.push(crate::room::create::new(1, _tryout));
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
        }
    }
}
