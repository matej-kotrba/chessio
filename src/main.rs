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
    pub fn render() -> Self {}
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

fn main() {
    let mut game = Game::new();

    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
