use serde::{Deserialize, Serialize};
use std::sync::RwLock;
use warp::Filter;

pub static SEED: RwLock<i64> = RwLock::new(0);

#[derive(Deserialize, Serialize)]
struct PatchBody {
    num: i64,
}

#[tokio::main]
async fn main() {
    let get = warp::get()
        .and(warp::path::end())
        .map(|| format!("{}", SEED.read().unwrap()));
    let post = warp::post()
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|body: PatchBody| {
            let mut w = SEED.write().unwrap();
            *w = body.num;
            format!("{}", body.num)
        });

    let routes = get.or(post);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
