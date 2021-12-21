use crate::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    pub client: isize, // id
    pub clerk: isize,  // id
    pub id: u64,       // room_id:  Room Id (Temporary)
}

pub mod create;

use tower::Service;
use http::{Request, Response, StatusCode};
use std::pin::Pin;
use std::future::Future;
use std::task::Context;
use std::task::Poll;

impl Service<Request<Body>> for Room {
    type Response = Request<Body>;
    type Error = http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self,  rooms: &mut Arc<Mutex<Option<Vec<Room>>>>, request: Request<Vec<u8>>) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
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

        //Ok::<_, Infallible>(Response::new(Body::empty())) // Why this works
        Ok::<_, Infallible>(res) // And this not? Since the Type is the same?
    }
}
