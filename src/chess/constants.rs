use std::collections::HashMap;

use raylib::{color::Color, RaylibHandle, RaylibThread};

use super::{PieceType, PiecesImagesType, Side, TileColorSchema};

pub const CHESSBOARD_WIDTH: i32 = 1000;
pub const CHESSBOARD_HEIGHT: i32 = 1000;
pub const LEFT_SIDE_PADDING: i32 = 50;
pub const WINDOW_WIDTH: i32 = 1400;
pub const WINDOW_HEIGHT: i32 = 1050;
pub const X_AXIS_LABELS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
pub const Y_AXIS_LABELS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];
pub const CHESSBOARD_SIZE: usize = 8;

// Piece image sets

pub fn getDefaultPieceImages(rl: &mut RaylibHandle, thread: &RaylibThread) -> PiecesImagesType {
    HashMap::from([
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
    ])
}

// Board sets

pub const DEFAULT_TILE_COLOR_SCHEMA: TileColorSchema = (Color::WHITE, Color::BLACK);
