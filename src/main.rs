use std::ops::Range;
use futures;
use hyper;
use rand;
use serde_derive::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize)]
struct RngResponse {
    value: f64,
}

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

fn main() {
    
}