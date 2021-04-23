use serde::Deserialize;
use serde_json::json;
use warp::http::StatusCode;
use warp::Filter;
use warp::Rejection;
/// Given a board state, return the best move along w/ the confidence. (Basically the fitness of the move)
fn gen_move<'a>(possible_moves: &'a [&'static str]) -> (&'a str, f32) {
    return (&possible_moves[0], 0.0);
}
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
        .and_then(|sent_move: Move| async move {
            // move logic
            let possible_moves = ["up", "down", "left", "right"];
            let out_move = gen_move(&possible_moves);
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

/// /move request sent by the battlesnake engine.
#[derive(Debug, Deserialize)]
struct Move {
    game: SentGame,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

/// Game ID and more info, such as the timeout.
#[derive(Debug, Deserialize)]
struct SentGame {
    id: String,
    timeout: u128,
}

/// Board that is sent by the engine in the request.
#[derive(Debug, Deserialize)]
struct Board {
    height: u8,
    width: u8,
    food: Vec<Pos>,
    hazards: Vec<Pos>,
    snakes: Vec<Battlesnake>,
}

/// Position, makes it much easier to work w/.
#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Pos {
    x: u8,
    y: u8,
}

/// the battlesnake struct, sent by the engine.
#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Battlesnake {
    id: String,
    name: String,
    health: u8,
    body: Vec<Pos>,
    latency: i16,
    head: Pos,
    length: u16,
    shout: String,
}

/// Cast into from the board state.
/// This one is actually used.
/// Contains a list of all snakes, and the size of the board. This assumes that the board is a square in shape.
#[derive(Debug)]
struct SmallBoard {
    snakes: Vec<SmallSnake>,
    size: u8,
}

/// Small storage space snake.
/// Used in the search.
#[derive(Debug)]
struct SmallSnake {
    id: String,
    health: u8,
    body: Vec<Pos>,
    head: Pos,
}

/// Small Move,
/// Holds the direction (UDLR)
/// and the ID.
#[derive(Debug)]
struct SmallMove {
    id: String,
    dir: String,
}
