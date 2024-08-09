use raylib::prelude::*;

use super::{Piece, PiecesImagesType, CHESSBOARD_SIZE};

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub bg: Option<Color>,
    pub color: Color,
    pub piece: Option<Piece>,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            color: Color::BLACK,
            bg: None,
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
    pub fn clear_bg(&mut self) {
        self.bg = None;
    }
}

pub struct TilesIter<'a> {
    pub tiles: &'a [[Tile; CHESSBOARD_SIZE]; CHESSBOARD_SIZE],
    pub index_y: usize,
    pub index_x: usize,
}

impl<'a> Iterator for TilesIter<'a> {
    type Item = (usize, usize, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_y >= CHESSBOARD_SIZE {
            return None;
        }

        let tile = &self.tiles[self.index_y][self.index_x];
        let tuple = (self.index_x, self.index_y, tile);

        self.index_x += 1;
        if self.index_x >= CHESSBOARD_SIZE {
            self.index_x = 0;
            self.index_y += 1;
        }

        return Some(tuple);
    }
}

pub struct TilesIterMut<'a> {
    pub tiles: &'a mut [[Tile; CHESSBOARD_SIZE]; CHESSBOARD_SIZE],
    pub index_y: usize,
    pub index_x: usize,
}

impl<'a> Iterator for TilesIterMut<'a> {
    type Item = (usize, usize, &'a mut Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_y >= CHESSBOARD_SIZE {
            return None;
        }

        let tuple = unsafe {
            (
                self.index_x,
                self.index_y,
                &mut *(&mut self.tiles[self.index_x][self.index_y] as *mut Tile),
            )
        };

        self.index_x += 1;
        if self.index_x >= CHESSBOARD_SIZE {
            self.index_x = 0;
            self.index_y += 1;
        }

        return Some(tuple);
    }
}
