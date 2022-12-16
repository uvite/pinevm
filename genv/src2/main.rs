use axum::{
    routing::{get,post,},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use pine::types::Float;

use serde_json::json;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/vm", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 4444));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}


async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateVm>,
) -> impl IntoResponse {
    // insert your application logic here
    let mut v1 = Vec::new();
    let user = Newvm {
        id: 1337,

        code:  payload.code,
        close: payload.close,
    };
    v1.push(123);

    //let close= Some(payload.close);

    // let close= json!{payload.close} ;
    println!("asdf{:?}", v1);


    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateVm {
    code: String,
    // high: Vec<Float>,
    // low:Vec<Float>,
    close:Vec<Float>,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct Newvm {
    id: u64,
    code: String,
    close:Vec<Float>,

}
//
// impl Serialize for Newvm {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         let mut s = serializer.serialize_struct("Newvm", 3)?;
//         s.serialize_field("code", &self.code)?;
//         s.serialize_field("close", &self.close)?;
//         s.serialize_field("phones", &self.phones)?;
//         s.end()
//     }
// }