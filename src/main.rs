use std::ops::Range;
use futures::{future, Future, Stream};
use hyper::{ Body, Error, Method, Request, Response, Server, StatusCode };
use hyper::service::service_fn;
use rand;
use serde_derive::{Serialize, Deserialize};
use serde_json;

static INDEX: &[u8] = b"Random Microservice";

#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", rename_all = "lowercase")]
enum RngRequest {
    Uniform {
        #[serde(flatten)]
        range: Range<i32>,
    },
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Bernouli {
        p: f64,
    },
}

#[derive(Serialize)]
struct RngResponse {
    value: f64,
}

fn handle_request(request: RngRequest) -> RngResponse {
    let mut rng = rand::thread_rng();
    let value = {
        match request {
            RngRequest::Uniform { range } => {
                rng.sample(Uniform::from(range))
            }
        }
    }
}

fn microservice_handler(req: Request<Body>)
    -> Box<dyn Future<Item=Response<Body>, Error=Error> + Send>
{
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/random") => {
            Box::new(future::ok(Response::new(INDEX.into())))
        },
        (&Method::POST, "/random") => {
            let body = req.into_body().concat2()
                .map(|chunks| {
                    let res = serde_json::from_slice::<RngRequest>(chunks.as_ref())
                        .map(handle_request)
                        .and_then(|resp| serde_json::to_string(&resp));
                    match res {
                        Ok(body) => {
                            Response::new(body.into())
                        },
                        Err(err) => {
                            Response::builder()
                                .status(StatusCode::UNPROCESSABLE_ENTITY)
                                .body(err.to_string().into())
                                .unwrap()
                        },
                    }
                });
            Box::new(body)
        },
        _ => {
            let resp = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not Found".into())
                .unwrap();
            Box::new(future::ok(resp))
        },
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = Server::bind(&addr);
    let server = builder.serve(|| {
           service_fn(microservice_handler)
    });

    let server = server.map_err(drop);
    hyper::rt::run(server);
}