use rocket::serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: Ruleset,
    pub timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ruleset {
    pub name: String,
    pub version: String,
    pub settings: RulesetSettings
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RulesetSettings {
    pub foodSpawnChance: u32,
    pub minimumFood: u32,
    pub hazardDamagePerTurn: i32,
    pub royale: RoyaleSettings,
    pub squad: SquadSettings
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct RoyaleSettings {
    pub shrinkEveryNTurns: u32
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct SquadSettings {
    pub allowBodyCollisions: bool,
    pub sharedElimination: bool,
    pub sharedHealth: bool,
    pub sharedLength: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    pub height: u32,
    pub width: u32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub hazards: Vec<Coord>,
}

#[derive(Debug)]
pub struct Board2dCell {
    pub coord: Coord,
    pub snake: Option<Battlesnake>, // will want to be an Option reference to the snake later after we figure lifetimes out
    pub hazard: i32, // hazard can be negative, as with healing pools
    pub food: bool
}

#[derive(Debug)]
pub struct Board2d {
    pub height: u32,
    pub width: u32,
    cells: Vec<Board2dCell>
}

impl Board2d {
    pub fn new(board: &Board, hazard_damage: i32) -> Self { // Board2d constructor
        let size = board.height * board.width;
        Self {
            height: board.height,
            width: board.width,
            cells: {
                let mut arr: Vec<Board2dCell> = Vec::new();
                for index in 0..size {
                    let x = index / board.height;
                    let y = index % board.height;
                    arr.push(Board2dCell {
                        coord: Coord { x, y },
                        snake: None,
                        hazard: 0,
                        food: false
                    });
                }

                for food in &board.food { // iterate food to add them to the array
                    let index = (food.x * board.width + food.y) as usize;
                    arr[index].food = true;
                }

                for hazard in &board.hazards { // for the moment hazard lives as an i32, may be positive or negative, & sums up all hazards found on a single coord
                    let index = (hazard.x * board.width + hazard.y) as usize;
                    arr[index].hazard += hazard_damage;
                }

                for snake in &board.snakes { // iterate snakes on board, then iterate their bodies to add to board
                    for body in &snake.body {
                        let index = (body.x * board.width + body.y) as usize;
                        arr[index].snake = Some(Battlesnake {
                            id: snake.id.clone(),
                            name: snake.name.clone(),
                            health: snake.health,
                            body: snake.body.clone(),
                            head: snake.head.clone(),
                            length: snake.length,
                            latency: snake.latency.clone(),
                            shout: snake.shout.clone()
                        })
                    }
                }

                arr
            }
        }
    }
    pub fn get_cell(&self, c: Coord) -> &Board2dCell { // should this return an Option?
        let index = (c.x * self.width + c.y) as usize;
        &self.cells[index]
    }
}

impl fmt::Display for Board2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str: String = String::from(""); 
        for j in (0..self.height).rev() {
            for i in 0..self.width {
                //let index = (i * self.width + j) as usize;
                //let cell = &self.cells[index];
                let cell = &self.get_cell(Coord { x: i, y: j });
                // if let Some(snake) = &cell.snake {
                if let Some(_) = &cell.snake {
                    str.push_str("s ");
                } else if cell.food {
                    if cell.hazard > 0 {
                        str.push_str("F ");
                    } else {
                        str.push_str("f ");
                    }
                } else if cell.hazard > 0 {
                    str.push_str("h ");
                } else {
                    //str.push_str(&format!("({},{}) ", i, j)); // useful for debugging coords of board
                    str.push_str("x ");
                }
            }
            if j != 0 {
                str.push_str("\n");
            }
        }

        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: u32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: u32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: u32,
    pub board: Board,
    pub you: Battlesnake,
}