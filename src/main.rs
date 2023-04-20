#[macro_use]
extern crate rocket;

use log::info;
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use serde::Serialize;
use serde_json::{Value};
use std::collections::HashMap;
use std::{env, fmt};

mod logic;
mod util;

// API and Response Objects
// See https://docs.battlesnake.com/api

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    height: u32,
    width: u32,
    food: Vec<Coord>,
    snakes: Vec<Battlesnake>,
    hazards: Vec<Coord>,
}

#[derive(Debug)]
pub struct Board2dCell {
    coord: Coord,
    snake: Option<String>, // optional snake ID
    hazard: i32, // hazard can be negative, as with healing pools
    food: bool
}

#[derive(Debug)]
pub struct Board2d {
    height: u32,
    width: u32,
    cells: Vec<Board2dCell>
}

impl Board2d {
    fn new(board: &Board) -> Self { // Board2d constructor
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
                arr
            }
        }
    }
    fn get_cell(&self, c: Coord) -> &Board2dCell { // should this return an Option?
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
                if let Some(snake) = &cell.snake {
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
                    str.push_str(&format!("({},{}) ", i, j)); // useful for debugging coords of board
                    //str.push_str("x ");
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
    id: String,
    name: String,
    health: u32,
    body: Vec<Coord>,
    head: Coord,
    length: u32,
    latency: String,
    shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coord {
    x: u32,
    y: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    game: Game,
    turn: u32,
    board: Board,
    you: Battlesnake,
}

#[get("/")]
fn handle_index() -> Json<Value> {
    Json(logic::info())
}

#[post("/start", format = "json", data = "<start_req>")]
fn handle_start(start_req: Json<GameState>) -> Status {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
fn handle_move(move_req: Json<GameState>) -> Json<Value> {
    let response = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    Json(response)
}

#[post("/end", format = "json", data = "<end_req>")]
fn handle_end(end_req: Json<GameState>) -> Status {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}

#[launch]
fn rocket() -> _ {
    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", &port);
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!("Starting Battlesnake Server...");

    rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "battlesnake/github/starter-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        )
}
