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
                    self.tiles[y][x].bg.unwrap_or(self.tiles[y][x].color),
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
    pub fn highlight_tile_by_position(&mut self, (x, y): (f32, f32)) {
        let tileX = (x / (WINDOW_WIDTH as f32 / Self::SIZE as f32)) as i32;
        let tileY = (y / (WINDOW_HEIGHT as f32 / Self::SIZE as f32)) as i32;

        if tileX >= 0 && tileX < Self::SIZE as i32 && tileY >= 0 && tileY < Self::SIZE as i32 {
            self.tiles[tileY as usize][tileX as usize].bg = Some(Color::RED);
        }
    }
    pub fn start_drag_event(&mut self, (x, y): (f32, f32)) {}
    pub fn get_piece_available_moves(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut available_moves: Vec<(usize, usize)> = Vec::new();
        let piece = self.tiles[y][x].piece;

        let piece = if let Some(piece) = piece {
            piece
        } else {
            return available_moves;
        };

        match piece.kind {
            PieceType::Pawn => match piece.side {
                Side::Black => {
                    if self.is_coord_in_board((x, y + 1)) && self.is_piece_on_cords((x, y + 1)).0 {
                        available_moves.push((x, y + 1));
                    }
                    if piece.did_move == false
                        && self.is_coord_in_board((x, y + 2))
                        && self.is_piece_on_cords((x, y + 2)).0
                    {
                        available_moves.push((x, y + 2));
                    }
                    let piece_on_coords = self.is_piece_on_cords((x - 1, y + 1));
                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x - 1, y + 1))
                                && self.is_piece_on_cords((x - 1, y + 1)).0
                                && piece.side != p
                            {
                                available_moves.push((x - 1, y + 1))
                            }
                        }
                        None => {}
                    }
                    let piece_on_coords = self.is_piece_on_cords((x + 1, y + 1));
                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x + 1, y + 1))
                                && self.is_piece_on_cords((x + 1, y + 1)).0
                                && piece.side != p
                            {
                                available_moves.push((x + 1, y + 1))
                            }
                        }
                        None => {}
                    }
                }
                Side::White => {
                    if self.is_coord_in_board((x, y - 1)) && self.is_piece_on_cords((x, y - 1)).0 {
                        available_moves.push((x, y - 1));
                    }
                    if piece.did_move == false
                        && self.is_coord_in_board((x, y - 2))
                        && self.is_piece_on_cords((x, y - 2)).0
                    {
                        available_moves.push((x, y - 2));
                    }
                    let piece_on_coords = self.is_piece_on_cords((x - 1, y - 1));
                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x - 1, y - 1))
                                && self.is_piece_on_cords((x - 1, y - 1)).0
                                && piece.side != p
                            {
                                available_moves.push((x - 1, y - 1))
                            }
                        }
                        None => {}
                    }
                    let piece_on_coords = self.is_piece_on_cords((x + 1, y - 1));
                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x + 1, y - 1))
                                && self.is_piece_on_cords((x + 1, y - 1)).0
                                && piece.side != p
                            {
                                available_moves.push((x + 1, y - 1))
                            }
                        }
                        None => {}
                    }
                }
            },
            PieceType::Rook => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Bishop => todo!(),
            PieceType::Queen => todo!(),
            PieceType::King => todo!(),
        }

        available_moves
    }
    fn is_piece_on_cords(&self, (x, y): (usize, usize)) -> (bool, Option<Side>) {
        let piece = self.tiles[y][x].piece;

        match piece {
            Some(piece) => (true, Some(piece.side)),
            None => (false, None),
        }
    }
    fn is_coord_in_board(&self, (x, y): (usize, usize)) -> bool {
        return x < Self::SIZE && y < Self::SIZE;
    }

    pub fn tiles_iter(&mut self) -> TilesIter {
        let iter = TilesIter {
            index_x: 0,
            index_y: 0,
            tiles: &self.tiles,
        };

        return iter;
    }
    pub fn tiles_iter_mut(&mut self) -> TilesIterMut {
        let iter = TilesIterMut {
            index_x: 0,
            index_y: 0,
            tiles: &mut self.tiles,
        };

        return iter;
    }
}

#[derive(Copy, Clone)]
struct Tile {
    pub bg: Option<Color>,
    color: Color,
    piece: Option<Piece>,
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

struct TilesIter<'a> {
    tiles: &'a [[Tile; Game::SIZE]; Game::SIZE],
    index_y: usize,
    index_x: usize,
}

impl<'a> Iterator for TilesIter<'a> {
    type Item = (usize, usize, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_y >= Game::SIZE {
            return None;
        }

        let tile = &self.tiles[self.index_y][self.index_x];
        let tuple = (self.index_x, self.index_y, tile);

        self.index_x += 1;
        if self.index_x >= Game::SIZE {
            self.index_x = 0;
            self.index_y += 1;
        }

        return Some(tuple);
    }
}

struct TilesIterMut<'a> {
    tiles: &'a mut [[Tile; Game::SIZE]; Game::SIZE],
    index_y: usize,
    index_x: usize,
}

impl<'a> Iterator for TilesIterMut<'a> {
    type Item = (usize, usize, &'a mut Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_y >= Game::SIZE {
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
        if self.index_x >= Game::SIZE {
            self.index_x = 0;
            self.index_y += 1;
        }

        return Some(tuple);
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

    let icon = Image::load_image("./static/other/logo.png").unwrap();
    rl.set_window_icon(icon);
    rl.set_window_title(&thread, "Chessio");

    let mut game = Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        // if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        //     println!("PRESSED");
        // }

        for (_, _, tile) in game.tiles_iter_mut() {
            tile.clear_bg();
        }

        let Vector2 {
            x: mouse_x,
            y: mouse_y,
        } = rl.get_mouse_position();

        game.highlight_tile_by_position((mouse_x, mouse_y));

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
