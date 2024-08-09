pub mod constants;
pub mod piece;
pub mod tile;
use std::collections::HashMap;

use constants::*;
use piece::*;
use raylib::prelude::*;
use tile::*;

type PiecesImagesType = HashMap<(PieceType, Side), Texture2D>;

pub struct GameMoveRecord {
    pub kind: PieceType,
    pub side: Side,
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub taken_piece: Option<PieceType>,
}

pub struct Game {
    pub tiles: [[Tile; CHESSBOARD_SIZE]; CHESSBOARD_SIZE],
    pub pieces_images: PiecesImagesType,
    pub hovered_piece_coords: Option<(usize, usize)>,
    pub move_records: Vec<GameMoveRecord>,
    pub is_check: Option<Side>,
    pub victor: Option<Side>,
}

impl Game {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        custom_imgs: Option<PiecesImagesType>,
        custom_tile_color_schema: Option<TileColorSchema>,
    ) -> Self {
        let mut tiles = [[Tile::new(); CHESSBOARD_SIZE]; CHESSBOARD_SIZE];

        let color_schema = custom_tile_color_schema.unwrap_or(DEFAULT_TILE_COLOR_SCHEMA);
        for y in 0..CHESSBOARD_SIZE {
            for x in 0..CHESSBOARD_SIZE {
                if (x + y % 2) % 2 == 0 {
                    tiles[y][x].color = color_schema.0;
                } else {
                    tiles[y][x].color = color_schema.1;
                }
            }
        }

        let mut game = Game {
            tiles,
            pieces_images: custom_imgs.unwrap_or(getDefaultPieceImages(rl, thread)),
            hovered_piece_coords: None,
            move_records: Vec::new(),
            is_check: None,
            victor: None,
        };
        game.reset();

        game
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let size = Self::get_tile_actuall_size() as usize;

        for (x, y, _) in self.tiles_iter() {
            d.draw_rectangle(
                LEFT_SIDE_PADDING + (x * size) as i32,
                (y * size) as i32,
                size as i32,
                size as i32,
                self.tiles[y][x].bg.unwrap_or(self.tiles[y][x].color),
            );
        }
    }
    fn get_tile_actuall_size() -> i32 {
        CHESSBOARD_WIDTH / (CHESSBOARD_SIZE as i32)
    }
    pub fn get_side_on_move(&self) -> Side {
        match self.move_records.last() {
            Some(mr) => {
                if mr.side == Side::White {
                    Side::Black
                } else {
                    Side::White
                }
            }
            _ => Side::White,
        }
    }
    //TODO:
    pub fn render_available_moves(&mut self, d: &mut RaylibDrawHandle) {
        match self.hovered_piece_coords {
            Some(coords) => {
                let tile_size = CHESSBOARD_WIDTH / (CHESSBOARD_SIZE as i32);

                match self.is_check {
                    Some(_) => {
                        let moves = self.get_piece_available_moves_with_check((
                            coords.0 as i32,
                            coords.1 as i32,
                        ));
                        for mov in moves {
                            d.draw_circle(
                                LEFT_SIDE_PADDING + (mov.0 as i32) * tile_size + tile_size / 2,
                                (mov.1 as i32) * tile_size + tile_size / 2,
                                10.0,
                                Color::GRAY,
                            );
                        }
                    }
                    None => {
                        let moves =
                            self.get_piece_available_moves((coords.0 as i32, coords.1 as i32));

                        for mov in moves {
                            let board_copy = self.tiles.clone();
                            self.tiles[mov.1][mov.0].piece = self.tiles[coords.1][coords.0].piece;
                            self.tiles[coords.1][coords.0].piece = None;
                            if self.is_check(self.get_side_on_move(), self.tiles) {
                                self.tiles = board_copy;
                                continue;
                            }
                            self.tiles = board_copy;

                            d.draw_circle(
                                LEFT_SIDE_PADDING + (mov.0 as i32) * tile_size + tile_size / 2,
                                (mov.1 as i32) * tile_size + tile_size / 2,
                                10.0,
                                Color::GRAY,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
    pub fn render_piece_at_coords(
        &mut self,
        d: &mut RaylibDrawHandle,
        (piece, side): (PieceType, Side),
        (x, y): (f32, f32),
    ) {
        let piece_img = self.pieces_images.get(&(piece, side));
        let tile_size = Self::get_tile_actuall_size();
        match piece_img {
            Some(img) => d.draw_texture_ex(
                img,
                Vector2 {
                    x: x - (tile_size / 2) as f32,
                    y: y - (tile_size / 2) as f32,
                },
                0.0,
                1.0,
                Color::WHITE,
            ),
            None => {}
        }
    }
    pub fn reset(&mut self) {
        use PieceType::*;
        let backrow: [PieceType; CHESSBOARD_SIZE] =
            [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
        let frontrow: [PieceType; CHESSBOARD_SIZE] = [Pawn; CHESSBOARD_SIZE];

        for (index, piece) in backrow.iter().enumerate() {
            self.tiles[0][index].piece = Some(Piece::new(*piece, Side::Black));
        }

        for (index, piece) in frontrow.iter().enumerate() {
            self.tiles[1][index].piece = Some(Piece::new(*piece, Side::Black));
        }

        for (index, piece) in frontrow.iter().enumerate() {
            self.tiles[CHESSBOARD_SIZE - 2][index].piece = Some(Piece::new(*piece, Side::White));
        }

        for (index, piece) in backrow.iter().enumerate() {
            self.tiles[CHESSBOARD_SIZE - 1][index].piece = Some(Piece::new(*piece, Side::White));
        }
    }
    pub fn get_tile_on_coords_mut(
        &mut self,
        (x, y): (f32, f32),
    ) -> Option<(&mut Tile, (usize, usize))> {
        let tile_x = (x / Self::get_tile_actuall_size() as f32) as i32;
        let tile_y = (y / Self::get_tile_actuall_size() as f32) as i32;

        if tile_x >= 0
            && tile_x < CHESSBOARD_SIZE as i32
            && tile_y >= 0
            && tile_y < CHESSBOARD_SIZE as i32
        {
            return Some((
                &mut self.tiles[tile_y as usize][tile_x as usize],
                (tile_x as usize, tile_y as usize),
            ));
        }

        None
    }
    pub fn get_tile_on_coords(&self, (x, y): (f32, f32)) -> Option<(&Tile, (usize, usize))> {
        let tile_x = (x / Self::get_tile_actuall_size() as f32) as i32;
        let tile_y = (y / Self::get_tile_actuall_size() as f32) as i32;

        if Self::is_tile_in_board((tile_x, tile_y)) {
            return Some((
                &self.tiles[tile_y as usize][tile_x as usize],
                (tile_x as usize, tile_y as usize),
            ));
        }

        None
    }
    //TODO:
    pub fn highlight_tile_by_coords(&mut self, (x, y): (f32, f32)) {
        let tile = self.get_tile_on_coords_mut((x, y));

        match tile {
            Some(t) => {
                t.0.bg = Some(Color::RED);
            }
            None => {}
        }
    }
    //TODO:
    pub fn start_drag_event(&mut self, (x, y): (f32, f32)) {
        let tile = self.get_tile_on_coords((x, y));

        let t = if let Some(tile) = tile {
            tile
        } else {
            self.hovered_piece_coords = None;
            return;
        };

        let piece = if let Some(piece) = t.0.piece {
            piece
        } else {
            self.hovered_piece_coords = None;
            return;
        };

        let last_move = self.move_records.last();
        match last_move {
            Some(lm) => {
                if piece.side != lm.side {
                    self.hovered_piece_coords = Some(t.1);
                } else {
                    self.hovered_piece_coords = None;
                }
            }
            None => {
                if piece.side == Side::White {
                    self.hovered_piece_coords = Some(t.1)
                } else {
                    self.hovered_piece_coords = None;
                }
            }
        }
    }

    //TODO:
    pub fn end_drag_event(&mut self, (x, y): (f32, f32)) {
        let tile_with_piece = if let Some(coords) = self.hovered_piece_coords {
            (self.tiles[coords.1][coords.0].piece, coords)
        } else {
            return;
        };

        let piece = if let Some(p) = tile_with_piece.0 {
            p
        } else {
            return;
        };

        let available_moves = self
            .get_piece_available_moves((tile_with_piece.1 .0 as i32, tile_with_piece.1 .1 as i32));

        let tile_to_drop = self.get_tile_on_coords((x, y));
        let coords = if let Some(tile) = tile_to_drop {
            tile.1
        } else {
            return;
        };
        if available_moves.contains(&coords) {
            let taken_piece_type: Option<PieceType> = if let Some(t) = tile_to_drop {
                match t.0.piece {
                    Some(p) => Some(p.kind),
                    None => None,
                }
            } else {
                None
            };

            let mut moved_piece = if let Some(p) = tile_with_piece.0 {
                p
            } else {
                return;
            };

            let move_record = GameMoveRecord {
                from: (tile_with_piece.1 .0, tile_with_piece.1 .0),
                to: coords,
                kind: piece.kind,
                side: piece.side,
                taken_piece: taken_piece_type,
            };

            let sides = if move_record.side == Side::Black {
                (Side::White, Side::Black)
            } else {
                (Side::Black, Side::White)
            };

            let board_copy = self.tiles.clone();
            self.tiles[coords.1][coords.0].piece = Some(moved_piece);
            self.tiles[tile_with_piece.1 .1][tile_with_piece.1 .0].piece = None;
            if self.is_check(sides.1, self.tiles) {
                self.tiles = board_copy;
                self.hovered_piece_coords = None;
                return;
            }

            moved_piece.did_move = true;
            self.move_records.push(move_record);

            if self.is_check(sides.0, self.tiles) {
                self.is_check = Some(sides.0);

                let mut can_continue_playing = false;
                'a: for y in 0..self.tiles.len() {
                    for x in 0..self.tiles.len() {
                        match self.tiles[y][x].piece {
                            Some(piece) => {
                                if piece.side == sides.0 {
                                    println!("{:?}", piece.side);
                                    let moves = self
                                        .get_piece_available_moves_with_check((x as i32, y as i32));
                                    if moves.len() > 0 {
                                        println!("{}{}: {:?}", x, y, moves);
                                        can_continue_playing = true;
                                        break 'a;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if !can_continue_playing {
                    self.victor = Some(sides.0);
                }
            } else {
                self.is_check = None;
            }
        }

        self.hovered_piece_coords = None;
    }
    fn get_pieces_linear_moves<'a>(
        &'a self,
        available_moves: &'a mut Vec<(usize, usize)>,
        (temp_x, temp_y): (i32, i32),
        (move_x, move_y): (i32, i32),
        piece_side: Side,
    ) {
        if Self::is_tile_in_board((temp_x as i32, temp_y as i32)) {
            let p = self.is_piece_on_coords((temp_x, temp_y));
            match p.1 {
                Some(side) => {
                    if side != piece_side {
                        available_moves.push((temp_x as usize, temp_y as usize));
                    }
                }
                None => {
                    available_moves.push((temp_x as usize, temp_y as usize));
                    Self::get_pieces_linear_moves(
                        self,
                        available_moves,
                        ((temp_x + move_x), (temp_y + move_y)),
                        (move_x, move_y),
                        piece_side,
                    )
                }
            }
        }
    }
    pub fn is_check(
        &self,
        check_for: Side,
        board: [[Tile; CHESSBOARD_SIZE]; CHESSBOARD_SIZE],
    ) -> bool {
        let mut king: Option<(usize, usize)> = None;

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                let piece = if let Some(p) = board[y][x].piece {
                    p
                } else {
                    continue;
                };
                if piece.kind == PieceType::King && piece.side == check_for {
                    king = Some((x, y));
                }
            }
        }

        match king {
            Some(k) => {
                for y in 0..board.len() {
                    for x in 0..board[y].len() {
                        match board[y][x].piece {
                            Some(p) => {
                                if p.side == check_for {
                                    continue;
                                }
                            }
                            None => {
                                continue;
                            }
                        }
                        let available_moves = self.get_piece_available_moves((x as i32, y as i32));
                        if available_moves.contains(&(k.0, k.1)) {
                            return true;
                        }
                    }
                }
            }
            None => {
                return false;
            }
        }

        false
    }
    fn get_distance_between_direct_coords(
        (x1, y1): (usize, usize),
        (x2, y2): (usize, usize),
    ) -> i64 {
        let a: i64 = (x2 as i64) - (x1 as i64);
        let b: i64 = (y2 as i64) - (y1 as i64);
        let c = ((a.pow(2) + b.pow(2)) as f64) / f64::sqrt(2.);
        if c.fract() != 0.0 {
            return c as i64;
        }
        c as i64
    }
    pub fn get_piece_available_moves(&self, (x, y): (i32, i32)) -> Vec<(usize, usize)> {
        let mut available_moves: Vec<(usize, usize)> = Vec::new();
        let piece = self.tiles[y as usize][x as usize].piece;

        let piece = if let Some(piece) = piece {
            piece
        } else {
            return available_moves;
        };

        // println!(
        //     "Piece: {:#?}, side: {:#?}, position: {} {}",
        //     piece.kind, piece.side, x, y
        // );

        match piece.kind {
            PieceType::Pawn => match piece.side {
                Side::Black => {
                    if Self::is_tile_in_board((x, y + 1)) && !self.is_piece_on_coords((x, y + 1)).0
                    {
                        available_moves.push((x as usize, (y + 1) as usize));

                        if piece.did_move == false
                            && Self::is_tile_in_board((x, y + 2))
                            && !self.is_piece_on_coords((x, y + 2)).0
                        {
                            available_moves.push((x as usize, (y + 2) as usize));
                        }
                    }

                    let piece_on_coords = if Self::is_tile_in_board((x - 1, y + 1)) {
                        self.is_piece_on_coords((x - 1, y + 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if Self::is_tile_in_board((x - 1, y + 1))
                                && self.is_piece_on_coords((x - 1, y + 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x - 1) as usize, (y + 1) as usize))
                            }
                        }
                        None => {}
                    }

                    let piece_on_coords = if Self::is_tile_in_board((x + 1, y + 1)) {
                        self.is_piece_on_coords((x + 1, y + 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if Self::is_tile_in_board((x + 1, y + 1))
                                && self.is_piece_on_coords((x + 1, y + 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x + 1) as usize, (y + 1) as usize))
                            }
                        }
                        None => {}
                    }
                }
                Side::White => {
                    if Self::is_tile_in_board((x, y - 1)) && !self.is_piece_on_coords((x, y - 1)).0
                    {
                        available_moves.push((x as usize, (y - 1) as usize));

                        if piece.did_move == false
                            && Self::is_tile_in_board((x, y - 2))
                            && !self.is_piece_on_coords((x, y - 2)).0
                        {
                            available_moves.push((x as usize, (y - 2) as usize));
                        }
                    }

                    let piece_on_coords = if Self::is_tile_in_board((x - 1, y - 1)) {
                        self.is_piece_on_coords((x - 1, y - 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if Self::is_tile_in_board((x - 1, y - 1))
                                && self.is_piece_on_coords((x - 1, y - 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x - 1) as usize, (y - 1) as usize))
                            }
                        }
                        None => {}
                    }

                    let piece_on_coords = if Self::is_tile_in_board((x + 1, y - 1)) {
                        self.is_piece_on_coords((x + 1, y - 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if Self::is_tile_in_board((x + 1, y - 1))
                                && self.is_piece_on_coords((x + 1, y - 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x + 1) as usize, (y - 1) as usize))
                            }
                        }
                        None => {}
                    }
                }
            },
            PieceType::Rook => match piece.side {
                Side::Black => {
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y),
                        (1, 0),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y),
                        (-1, 0),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y + 1),
                        (0, 1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y - 1),
                        (0, -1),
                        Side::Black,
                    );
                }
                Side::White => {
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y),
                        (1, 0),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y),
                        (-1, 0),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y + 1),
                        (0, 1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y - 1),
                        (0, -1),
                        Side::White,
                    );
                }
            },
            PieceType::Bishop => match piece.side {
                Side::Black => {
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y + 1),
                        (1, 1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y - 1),
                        (1, -1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y + 1),
                        (-1, 1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y - 1),
                        (-1, -1),
                        Side::Black,
                    );
                }
                Side::White => {
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y + 1),
                        (1, 1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y - 1),
                        (1, -1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y + 1),
                        (-1, 1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y - 1),
                        (-1, -1),
                        Side::White,
                    );
                }
            },
            PieceType::Queen => match piece.side {
                Side::Black => {
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y + 1),
                        (1, 1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y - 1),
                        (1, -1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y + 1),
                        (-1, 1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y - 1),
                        (-1, -1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y),
                        (1, 0),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y),
                        (-1, 0),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y + 1),
                        (0, 1),
                        Side::Black,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y - 1),
                        (0, -1),
                        Side::Black,
                    );
                }
                Side::White => {
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y + 1),
                        (1, 1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y - 1),
                        (1, -1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y + 1),
                        (-1, 1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y - 1),
                        (-1, -1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x + 1, y),
                        (1, 0),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x - 1, y),
                        (-1, 0),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y + 1),
                        (0, 1),
                        Side::White,
                    );
                    self.get_pieces_linear_moves(
                        &mut available_moves,
                        (x, y - 1),
                        (0, -1),
                        Side::White,
                    );
                }
            },
            PieceType::Knight => match piece.side {
                Side::Black => {
                    let coords = [
                        (x - 1, y - 2),
                        (x + 1, y - 2),
                        (x + 2, y - 1),
                        (x + 2, y + 1),
                        (x + 2, y + 1),
                        (x + 1, y + 2),
                        (x - 1, y + 2),
                        (x - 2, y + 1),
                        (x - 2, y - 1),
                    ];
                    for coords in coords {
                        if Self::is_tile_in_board(coords)
                            && self.is_piece_on_coords(coords).1 != Some(Side::Black)
                        {
                            available_moves.push((coords.0 as usize, coords.1 as usize));
                        }
                    }
                }
                Side::White => {
                    let coords = [
                        (x - 1, y - 2),
                        (x + 1, y - 2),
                        (x + 2, y - 1),
                        (x + 2, y + 1),
                        (x + 2, y + 1),
                        (x + 1, y + 2),
                        (x - 1, y + 2),
                        (x - 2, y + 1),
                        (x - 2, y - 1),
                    ];
                    for coords in coords {
                        if Self::is_tile_in_board(coords)
                            && self.is_piece_on_coords(coords).1 != Some(Side::White)
                        {
                            available_moves.push((coords.0 as usize, coords.1 as usize));
                        }
                    }
                }
            },
            PieceType::King => {
                let coords_around_king = [
                    (x + 1, y),
                    (x + 1, y + 1),
                    (x, y + 1),
                    (x - 1, y + 1),
                    (x - 1, y),
                    (x - 1, y - 1),
                    (x + 1, y),
                    (x, y - 1),
                    (x + 1, y - 1),
                ];
                match piece.side {
                    Side::Black => {
                        let mut enemy_king_coords = None;
                        for (king_x, king_y, tile) in self.tiles_iter() {
                            match tile.piece {
                                Some(piece) => {
                                    if piece.kind == PieceType::King && piece.side == Side::White {
                                        enemy_king_coords = Some((king_x, king_y));
                                        break;
                                    }
                                }
                                None => {}
                            }
                        }

                        match enemy_king_coords {
                            Some(enemy_king_coords) => {
                                for coords in coords_around_king {
                                    if Self::is_tile_in_board(coords)
                                        && Self::get_distance_between_direct_coords(
                                            (coords.0 as usize, coords.1 as usize),
                                            enemy_king_coords,
                                        ) > 1
                                        && self.is_piece_on_coords(coords).1 != Some(Side::Black)
                                    {
                                        available_moves
                                            .push((coords.0 as usize, coords.1 as usize));
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                    Side::White => {
                        let mut enemy_king_coords = None;
                        for (king_x, king_y, tile) in self.tiles_iter() {
                            match tile.piece {
                                Some(piece) => {
                                    if piece.kind == PieceType::King && piece.side == Side::Black {
                                        enemy_king_coords = Some((king_x, king_y));
                                        break;
                                    }
                                }
                                None => {}
                            }
                        }

                        match enemy_king_coords {
                            Some(enemy_king_coords) => {
                                for coords in coords_around_king {
                                    if Self::is_tile_in_board(coords)
                                        && Self::get_distance_between_direct_coords(
                                            (coords.0 as usize, coords.1 as usize),
                                            enemy_king_coords,
                                        ) > 1
                                        && self.is_piece_on_coords(coords).1 != Some(Side::White)
                                    {
                                        available_moves
                                            .push((coords.0 as usize, coords.1 as usize));
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        available_moves
    }
    pub fn get_piece_available_moves_with_check(
        &mut self,
        (x, y): (i32, i32),
    ) -> Vec<(usize, usize)> {
        let mut available_moves = Vec::new();
        let moves = self.get_piece_available_moves((x, y));

        for (move_x, move_y) in moves {
            let board_copy = self.tiles.clone();
            self.tiles[move_y][move_x].piece = self.tiles[y as usize][x as usize].piece;
            self.tiles[y as usize][x as usize].piece = None;
            if !self.is_check(self.get_side_on_move(), self.tiles) {
                available_moves.push((move_x, move_y));
            }
            self.tiles = board_copy;
        }

        println!("{:?}", available_moves);

        available_moves
    }
    fn is_piece_on_coords(&self, (x, y): (i32, i32)) -> (bool, Option<Side>) {
        let piece = self.tiles[y as usize][x as usize].piece;

        match piece {
            Some(piece) => (true, Some(piece.side)),
            None => (false, None),
        }
    }
    fn is_tile_in_board((x, y): (i32, i32)) -> bool {
        return x >= 0 && y >= 0 && x < CHESSBOARD_SIZE as i32 && y < CHESSBOARD_SIZE as i32;
    }

    pub fn tiles_iter(&self) -> TilesIter {
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
