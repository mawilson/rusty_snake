use crate::classes::{Board2d, Coord, Direction, Battlesnake, GameStateC};

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

// returns the coordinate in the direction of dir from start_coord, given a board b with a width, height, & is_wrapped field
fn get_coord_after_move(start_coord: &Coord, dir: Direction, b: &Board2d) -> Option<Coord> {
    let x: u32;
    let y: u32;
    match dir {
        Direction::Up => {
            x = start_coord.x;
            if b.is_wrapped {
                if start_coord.y == (b.height - 1) {
                    y = 0; // wrapping up over x axis
                } else {
                    y = start_coord.y + 1;
                }
                Some(Coord { x, y })
            } else {
                if start_coord.y == (b.height - 1) {
                    None // cannot go up from top row in unwrapped
                } else {
                    Some(Coord { x, y: start_coord.y + 1 })
                }
            }
        },
        Direction::Down => {
            x = start_coord.x;
            if b.is_wrapped {
                if start_coord.y == 0 {
                    y = b.height - 1;
                } else {
                    y = start_coord.y - 1;
                }
                Some(Coord { x, y })
            } else {
                if start_coord.y == 0 {
                    None // cannot go down from bottom row in unwrapped
                } else {
                    Some(Coord { x, y: start_coord.y - 1 })
                }
            }
        },
        Direction::Left => {
            y = start_coord.y;
            if b.is_wrapped {
                if start_coord.x == 0 {
                    x = b.width - 1;
                } else {
                    x = start_coord.x - 1;
                }
                Some(Coord { x, y })
            } else {
                if start_coord.x == 0 {
                    None // cannot go left from left column in unwrapped
                } else {
                    Some(Coord { x: start_coord.x - 1, y })
                }
            }
        },
        Direction::Right => {
            y = start_coord.y;
            if b.is_wrapped {
                if start_coord.x == (b.width - 1) {
                    x = 0;
                } else {
                    x = start_coord.x + 1;
                }
                Some(Coord { x, y })
            } else {
                if start_coord.y == (b.width - 1) {
                    None // cannot go right from right column in unwrapped
                } else {
                    Some(Coord { x: start_coord.x + 1, y })
                }
            }
        }
    }
}

// Moves a snake, moving its head in dir, removing the old tail, eating if applicable, applying hazard damage. Snake must be mutable, but board2d does not change here
// returns a boolean based on whether snake is still alive or not (checks for valid cell to move to & health, but not other snakes)
pub fn move_snake(b: &Board2d, snake: &mut Battlesnake, dir: Direction) -> bool {
    let new_head = get_coord_after_move(&snake.head, dir, b);
    if let Some(head) = new_head { // if cell exists, move snake there
        let cell = b.get_cell(head);
        snake.body.pop(); // last element of snake always pops
        let new = [cell.coord.clone()];
        snake.body = snake.body.splice(0.., new).collect(); // prepends snake body with new head
        if cell.food {
            snake.health = 100; // eating tops health up to 100
            snake.length = snake.length + 1; // eating increments length
            let last = snake.body.last().cloned(); // clone last element of snake body (if it exists) & push it to end of snake body
            if let Some(last) = last {
                snake.body.push(last);
            }
        } else {
            let health_signed: i32 = snake.health as i32;
            if cell.hazard != 0 { // if hazard exists, either add or subtract it - need to deal with unsigned/signed nonsense here
                if cell.hazard > 0 { // hazard is harmful, subtract from health
                    if cell.hazard >= health_signed { // health cannot go below 0
                        snake.health = 0;
                    } else {
                        snake.health = (health_signed - cell.hazard) as u32; // should be safe since cell.hazard must be less than health_signed
                    }
                } else { // hazard is helpful, add to health, capping at 100
                    snake.health = (health_signed - cell.hazard) as u32; // safe because this is a double negative & actually an addition
                    if snake.health > 100 { snake.health = 100; }
                }
            } else {
                snake.health = snake.health - 1; // in the absence of any hazard or food, health goes down by 1
            }
        }

        if snake.health > 0 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

// takes a GameStateC & updates it, incrementing turn, removing dead snakes, updating hazard, food if possible
// pub fn update_game(game: &mut GameStateC) {
//     game.turn = game.turn + 1;
//     let live_snakes: Vec<Battlesnake> = Vec::new(); // will replace old game snakes with this one after checking to see which are still alive
//     for snake in &game.board2d.snakes {
//         if snake.health > 0 { // if snake health is 0, can ignore it as it has starved
//             for other_snakes in &game.board2d.snakes { // look through other snakes to see if I've collided with any of them

//             }
//         }
//     }
//     game.board2d = game.board2d;
// }