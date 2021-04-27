use movegen::*;
use serde_json::json;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;
pub mod movegen;

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| {
        warp::reply::json(&json!({
            "apiversion": "1",
            "color": "#b7410e",
            "head": "",
            "tail": "",
        }))
    });
    let start = warp::path("start")
        .and(warp::post())
        .map(|| warp::reply::with_status("", StatusCode::IM_A_TEAPOT));
    let end = warp::path("end")
        .and(warp::post())
        .map(|| warp::reply::with_status("", StatusCode::IM_A_TEAPOT));
    let get_move = warp::path("move")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|sent_move: engine::Move| async move {
            // move logic
            let possible_moves = ["up", "down", "left", "right"];
            let out_move = search::gen_move(&sent_move);
            println!(
                "Turn : {}, Moving: {}, with confidence {}",
                3, out_move.0, out_move.1
            );
            Ok(warp::reply::json(&json!({
                "move": out_move,
                "shout": ""
            }))) as Result<_, Rejection>
        });
    let routes = index.or(start).or(end).or(get_move);
    use dotenv;
    dotenv::dotenv().expect(".env file not found");
    let port = std::env::var("PORT")
        .expect("PORT Environment Variable not set")
        .parse()
        .expect("PORT is not a valid port number");
    println!("Listening on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
