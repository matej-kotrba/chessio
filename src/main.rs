use raylib::prelude::*;

struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut tiles = [[Tile::new(); Board::SIZE]; Board::SIZE];

        for y in 0..Board::SIZE {
            for x in 0..Board::SIZE {
                if (x + y % 2) % 2 == 0 {
                    tiles[y][x].color = Color::WHITE;
                }
            }
        }

        Game {
            board: Board { tiles: tiles },
        }
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let size = WINDOW_WIDTH as usize / Board::SIZE;
        for y in 0..Board::SIZE {
            for x in 0..Board::SIZE {
                d.draw_rectangle(
                    (x * size) as i32,
                    (y * size) as i32,
                    size as i32,
                    size as i32,
                    self.board.tiles[y][x].color,
                );
            }
        }
    }
}

struct Board {
    tiles: [[Tile; Self::SIZE]; Self::SIZE],
}

impl Board {
    pub const SIZE: usize = 8;
}

#[derive(Copy, Clone)]
struct Tile {
    color: Color,
    piece: Option<Piece>,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            color: Color::BLACK,
            piece: None,
        }
    }
}

#[derive(Copy, Clone)]
struct Piece {}

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

fn main() {
    let game = Game::new();

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Chessio")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        game.render(&mut d);
    }
}
