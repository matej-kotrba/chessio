use std::collections::HashMap;

use raylib::prelude::*;

type PiecesImagesType = HashMap<(PieceType, Side), Texture2D>;

struct Game {
    tiles: [[Tile; Self::SIZE]; Self::SIZE],
    pieces_images: PiecesImagesType,
}

impl Game {
    pub const SIZE: usize = 8;

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut tiles = [[Tile::new(); Self::SIZE]; Self::SIZE];

        for y in 0..Self::SIZE {
            for x in 0..Self::SIZE {
                if (x + y % 2) % 2 == 0 {
                    tiles[y][x].color = Color::WHITE;
                }
            }
        }

        let pieces_images: PiecesImagesType = HashMap::from([
            (
                (PieceType::Rook, Side::Black),
                rl.load_texture(thread, "./static/pieces/pngs/RookBlack.png")
                    .unwrap(),
            ),
            (
                (PieceType::Knight, Side::Black),
                rl.load_texture(thread, "./static/pieces/pngs/KnightBlack.png")
                    .unwrap(),
            ),
            (
                (PieceType::Bishop, Side::Black),
                rl.load_texture(thread, "./static/pieces/pngs/BishopBlack.png")
                    .unwrap(),
            ),
            (
                (PieceType::Queen, Side::Black),
                rl.load_texture(thread, "./static/pieces/pngs/QueenBlack.png")
                    .unwrap(),
            ),
            (
                (PieceType::King, Side::Black),
                rl.load_texture(thread, "./static/pieces/pngs/KingBlack.png")
                    .unwrap(),
            ),
            (
                (PieceType::Pawn, Side::Black),
                rl.load_texture(thread, "./static/pieces/pngs/PawnBlack.png")
                    .unwrap(),
            ),
            (
                (PieceType::Rook, Side::White),
                rl.load_texture(thread, "./static/pieces/pngs/RookWhite.png")
                    .unwrap(),
            ),
            (
                (PieceType::Knight, Side::White),
                rl.load_texture(thread, "./static/pieces/pngs/KnightWhite.png")
                    .unwrap(),
            ),
            (
                (PieceType::Bishop, Side::White),
                rl.load_texture(thread, "./static/pieces/pngs/BishopWhite.png")
                    .unwrap(),
            ),
            (
                (PieceType::Queen, Side::White),
                rl.load_texture(thread, "./static/pieces/pngs/QueenWhite.png")
                    .unwrap(),
            ),
            (
                (PieceType::King, Side::White),
                rl.load_texture(thread, "./static/pieces/pngs/KingWhite.png")
                    .unwrap(),
            ),
            (
                (PieceType::Pawn, Side::White),
                rl.load_texture(thread, "./static/pieces/pngs/PawnWhite.png")
                    .unwrap(),
            ),
        ]);

        let mut game = Game {
            tiles,
            pieces_images,
        };
        game.reset();

        game
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let size = WINDOW_WIDTH as usize / Self::SIZE;
        for y in 0..Self::SIZE {
            for x in 0..Self::SIZE {
                d.draw_rectangle(
                    (x * size) as i32,
                    (y * size) as i32,
                    size as i32,
                    size as i32,
                    self.tiles[y][x].color,
                );
            }
        }
    }
    pub fn reset(&mut self) {
        use PieceType::*;
        let backrow: [PieceType; Self::SIZE] =
            [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        let frontrow: [PieceType; Self::SIZE] = [Pawn; Self::SIZE];

        for (index, piece) in backrow.iter().enumerate() {
            self.tiles[0][index].piece = Some(Piece::new(*piece, Side::Black));
        }

        for (index, piece) in frontrow.iter().enumerate() {
            self.tiles[1][index].piece = Some(Piece::new(*piece, Side::Black));
        }

        for (index, piece) in frontrow.iter().enumerate() {
            self.tiles[Self::SIZE - 2][index].piece = Some(Piece::new(*piece, Side::White));
        }

        for (index, piece) in backrow.iter().enumerate() {
            self.tiles[Self::SIZE - 1][index].piece = Some(Piece::new(*piece, Side::White));
        }
    }
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

    pub fn render(
        &self,
        d: &mut RaylibDrawHandle,
        (x, y): (i32, i32),
        pieces_images: &PiecesImagesType,
    ) {
        match self.piece {
            Some(piece) => d.draw_texture_ex(
                pieces_images.get(&(piece.kind, piece.side)).unwrap(),
                Vector2 {
                    x: x as f32,
                    y: y as f32,
                },
                0.0,
                1.0,
                Color::WHITE,
            ),
            // Some(piece) => d.draw_text(&piece.to_string(), x, y, 18, Color::PURPLE),
            None => {}
        }
    }
}

#[derive(Copy, Clone)]
struct Piece {
    kind: PieceType,
    side: Side,
    did_move: bool,
}

impl Piece {
    pub fn new(kind: PieceType, side: Side) -> Self {
        Piece {
            kind,
            side,
            did_move: false,
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            PieceType::Pawn => {
                write!(f, "Pawn")
            }
            PieceType::Rook => {
                write!(f, "Rook")
            }
            PieceType::Knight => {
                write!(f, "Knight")
            }
            PieceType::Bishop => {
                write!(f, "Bishop")
            }
            PieceType::Queen => {
                write!(f, "Queen")
            }
            PieceType::King => {
                write!(f, "King")
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Side {
    Black,
    White,
}

const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 1000;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Chessio")
        .build();

    let game = Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            println!("PRESSED");
        }

        let mut d = rl.begin_drawing(&thread);

        game.render(&mut d);

        for y in 0..Game::SIZE {
            for x in 0..Game::SIZE {
                game.tiles[y][x].render(
                    &mut d,
                    (
                        x as i32 * (WINDOW_WIDTH / Game::SIZE as i32),
                        y as i32 * (WINDOW_HEIGHT / Game::SIZE as i32),
                    ),
                    &game.pieces_images,
                )
            }
        }
    }
}
