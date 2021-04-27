use serde::Deserialize;
/// /move request sent by the battlesnake engine.
#[derive(Debug, Deserialize, Clone)]
pub struct Move {
    game: SentGame,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

/// Game ID and more info, such as the timeout.
#[derive(Debug, Deserialize, Clone)]
struct SentGame {
    id: String,
    timeout: u128,
}

/// Board that is sent by the engine in the request.
#[derive(Debug, Deserialize, Clone)]
struct Board {
    height: u8,
    width: u8,
    food: Vec<Pos>,
    hazards: Vec<Pos>,
    snakes: Vec<Battlesnake>,
}

/// Position, makes it much easier to work w/.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone, Copy)]
struct Pos {
    x: u8,
    y: u8,
}

/// the battlesnake struct, sent by the engine.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
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
pub struct SmallBoard {
    pub snakes: Vec<SmallSnake>,
    pub you: String,
    food: Vec<Pos>,
    size: u8,
}

/// Small storage space snake.
/// Used in the search.
#[derive(Debug)]
pub struct SmallSnake {
    pub id: String,
    health: u8,
    body: Vec<Pos>,
    head: Pos,
}
impl From<&Battlesnake> for SmallSnake {
    fn from(bsnake: &Battlesnake) -> Self {
        SmallSnake {
            id: bsnake.id.clone(),
            health: bsnake.health.clone(),
            body: bsnake.body.clone(),
            head: bsnake.head.clone(),
        }
    }
}
/// Small Move,
/// Holds the direction (UDLR)
/// and the ID.
#[derive(Debug)]
struct SmallMove {
    id: String,
    dir: String,
}

impl SmallBoard {
    pub fn from(item: Move) -> Self {
        SmallBoard {
            snakes: item
                .board
                .snakes
                .iter()
                .map(|x| SmallSnake::from(x))
                .collect(),
            food: item.board.food,
            size: item.board.height,
            you: item.you.id,
        }
    }
}
