use super::{engine::*, *};
#[derive(Debug)]
struct Node {
    state: SmallBoard,
    pub score: f64,
    pub visits: u32,
    turn : bool,
    children: Vec<Node>,
}

impl Node{
    pub fn all_children(&mut self, eval : impl Fn(&SmallBoard) -> f64) {

    }
}

fn static_eval(state: &SmallBoard) -> f64 {
    0.0
}
/// Given a board state, return the best move along w/ the confidence. (Basically the fitness of the move)
pub fn gen_move(state: &engine::Move) -> (&'static str, f32) {
    let mut thing = SmallBoard::from(state.clone());
    // get deepest node, following the minimizing and maximizing. that is unexplored.

    // evaluate each of the nodes children
    // choose the one with the highest or lowest score.
    // back propogate that score. 
    // repeat 1-4 as many times as possible.
    return (&"up", 0.0);
}
fn search_eval(node: &mut Node, maximizing: bool) -> f64 {
    if node.children.len() == 0 {
        // has no children
        if node.state.is_end() {
            // is it an end state?
            if node.state.snakes[0].id.eq(&node.state.you) {
                return f64::MAX;
            }
            return f64::MIN;
        } else {
            // if it isnt, then populate the children of the node, and return the highest or lowest score.
            node.all_children(&static_eval);
            let mut maxormin;
            if maximizing {
                maxormin = f64::MIN;
            } else {
                maxormin = f64::MAX;
            }
            for x in node.children {
                if maximizing && maxormin < x.score {
                    maxormin = x.score;
                } else if !maximizing && maxormin > x.score {
                    maxormin = x.score;
                }
            }
            return maxormin;
        }
    } else {
        // node has children. pick the highst or lowest child, and run search eval. in doing so, increase your visits, and also the score.
        let mut maxormin;
        let mut nodetosearch;
        if maximizing {
            maxormin = f64::MIN;
        } else {
            maxormin = f64::MAX;
        }
        for x in node.children {
            if maximizing && maxormin < x.score {
                maxormin = x.score;
                nodetosearch = &mut x;
            } else if !maximizing && maxormin > x.score {
                maxormin = x.score;
                nodetosearch = &mut x;
            }
        }
        nodetosearch.visits += 1;
        nodetosearch.score += search_eval(nodetosearch, !maximizing);
        return nodetosearch.score;
    }
}
