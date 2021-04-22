use serde::Deserialize;
use serde_json::json;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;

#[tokio::main]
async fn main() {
    let index = warp::path::end().map(|| {
        warp::reply::json(&json!({
            "apiversion": "1",
            "color": "",
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
        .and_then(|sent_move: Move| async move {
            // move logic
            let possible_moves = ["up", "down", "left", "right"];
            Ok(warp::reply::json(&json!({
                "move": possible_moves[3],
                "shout": ""
            }))) as Result<_, Rejection>
        });
    let routes = index
        .or(start)
        .or(end)
        .or(get_move);
    use dotenv;
    dotenv::dotenv().expect(".env file not found");
    let port = std::env::var("PORT")
        .expect("PORT Environment Variable not set")
        .parse()
        .expect("PORT is not a valid port number");
    println!("Listening on port {}", port);    
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
    
}

#[derive(Debug, Deserialize)]
struct Move {
    game: SentGame,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[derive(Debug, Deserialize)]
struct SentGame {
    id: String,
    timeout: u128,
}

#[derive(Debug, Deserialize)]
struct Board {
    height: u8,
    width: u8,
    food: Vec<Pos>,
    hazards: Vec<Pos>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Pos {
    x : u16,
    y : u16,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Battlesnake {
    id: String,
    name: String,
    health: u8,
    body: Vec<Pos>,
    latency: String,
    head: Pos,
    length: u16,
    shout: String,
}
