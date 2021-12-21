 if let Ok(something) = room::handler(request.uri().to_string(), rooms).await {
            // Lets upgrade protocols to WebSockets.
            Response::builder()
                .status(103) // Early Hints - Shows that we are upgrading...
                .body(
                    "Hey there, we have authenticated you and now We'll enter the room!"
                        .to_string(),
                )
        } else {
            let body: Vec<u8> = format!(          
                "Hey there, either you don't have permissions to access this or if you does, this: {} do not exists anymore!",
                request.uri().to_string()
                ).as_bytes().to_owned();

            let res = Response::builder()
                .status(417) // Expectation Failed, as always
                .body(body)
                .expect("Unable to create `http::Response`");

        }


                 Ok::<_, Infallible>(Response::new(Body::empty()))



pub async fn handler(
    request: Request<Body>,
    rooms: &mut Mutex<Option<Vec<Room>>>
) -> Result<hyper::Response<hyper::Body>, std::convert::Infallible> {
     
}
