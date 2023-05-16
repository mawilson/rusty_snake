use crate::classes::{Board2d, Coord};

// returns true if a move doesn't result in (almost) certain death, false if it does
// ways to die include off-board (non-wrapped), out of health (including hazard death), snake body
pub fn safe_move(x: u32, y: u32, b: &Board2d) -> bool {
    if x > (b.width - 1) {
        return false;
    } else if y > (b.height - 1) {
        return false;
    }
    let cell = b.get_cell(Coord { x, y });
    if let Some(_) = &cell.snake {
        return false;
    }
    return true;
}

// pub fn move_snake(b: &Board2d, snake: &String, move: &String) {

// }