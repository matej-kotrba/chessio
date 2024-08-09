use std::collections::HashMap;

use raylib::prelude::*;

type PiecesImagesType = HashMap<(PieceType, Side), Texture2D>;

struct GameMoveRecord {
    kind: PieceType,
    side: Side,
    from: (usize, usize),
    to: (usize, usize),
    taken_piece: Option<PieceType>,
}

struct Game {
    tiles: [[Tile; Self::SIZE]; Self::SIZE],
    pieces_images: PiecesImagesType,
    hovered_piece_coords: Option<(usize, usize)>,
    move_records: Vec<GameMoveRecord>,
    is_check: Option<Side>,
    victor: Option<Side>,
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
            hovered_piece_coords: None,
            move_records: Vec::new(),
            is_check: None,
            victor: None,
        };
        game.reset();

        game
    }
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        let size = CHESSBOARD_WIDTH as usize / Self::SIZE;
        for y in 0..Self::SIZE {
            for x in 0..Self::SIZE {
                d.draw_rectangle(
                    LEFT_SIDE_PADDING + (x * size) as i32,
                    (y * size) as i32,
                    size as i32,
                    size as i32,
                    self.tiles[y][x].bg.unwrap_or(self.tiles[y][x].color),
                );
            }
        }
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
    pub fn render_available_moves(&mut self, d: &mut RaylibDrawHandle) {
        match self.hovered_piece_coords {
            Some(coords) => {
                let tile_size = CHESSBOARD_WIDTH / (Self::SIZE as i32);

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
        match piece_img {
            Some(img) => d.draw_texture_ex(
                img,
                Vector2 {
                    x: x - ((CHESSBOARD_WIDTH / Self::SIZE as i32) / 2) as f32,
                    y: y - ((CHESSBOARD_HEIGHT / Self::SIZE as i32) / 2) as f32,
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

        // self.tiles[4][4].piece = Some(Piece::new(PieceType::Pawn, Side::Black));
        self.tiles[4][7].piece = Some(Piece::new(PieceType::Bishop, Side::Black));
    }
    pub fn get_tile_on_coords_mut(
        &mut self,
        (x, y): (f32, f32),
    ) -> Option<(&mut Tile, (usize, usize))> {
        let tile_x = (x / (CHESSBOARD_WIDTH as f32 / Self::SIZE as f32)) as i32;
        let tile_y = (y / (CHESSBOARD_HEIGHT as f32 / Self::SIZE as f32)) as i32;

        if tile_x >= 0 && tile_x < Self::SIZE as i32 && tile_y >= 0 && tile_y < Self::SIZE as i32 {
            return Some((
                &mut self.tiles[tile_y as usize][tile_x as usize],
                (tile_x as usize, tile_y as usize),
            ));
        }

        None
    }
    pub fn get_tile_on_coords(&self, (x, y): (f32, f32)) -> Option<(&Tile, (usize, usize))> {
        let tile_x = (x / (CHESSBOARD_WIDTH as f32 / Self::SIZE as f32)) as i32;
        let tile_y = (y / (CHESSBOARD_HEIGHT as f32 / Self::SIZE as f32)) as i32;

        if tile_x >= 0 && tile_x < Self::SIZE as i32 && tile_y >= 0 && tile_y < Self::SIZE as i32 {
            return Some((
                &self.tiles[tile_y as usize][tile_x as usize],
                (tile_x as usize, tile_y as usize),
            ));
        }

        None
    }
    pub fn highlight_tile_by_position(&mut self, (x, y): (f32, f32)) {
        let tile = self.get_tile_on_coords_mut((x, y));

        match tile {
            Some(t) => {
                t.0.bg = Some(Color::RED);
            }
            None => {}
        }
    }
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
            None => self.hovered_piece_coords = Some(t.1),
        }
    }

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
                        let moves = self.get_piece_available_moves_with_check((x as i32, y as i32));
                        if moves.len() > 0 {
                            can_continue_playing = true;
                            break 'a;
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
        if Self::is_coord_in_board(&self, (temp_x as i32, temp_y as i32)) {
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
    pub fn is_check(&self, check_for: Side, board: [[Tile; Self::SIZE]; Self::SIZE]) -> bool {
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
                    if self.is_coord_in_board((x, y + 1)) && !self.is_piece_on_coords((x, y + 1)).0
                    {
                        available_moves.push((x as usize, (y + 1) as usize));

                        if piece.did_move == false
                            && self.is_coord_in_board((x, y + 2))
                            && !self.is_piece_on_coords((x, y + 2)).0
                        {
                            available_moves.push((x as usize, (y + 2) as usize));
                        }
                    }

                    let piece_on_coords = if self.is_coord_in_board((x - 1, y + 1)) {
                        self.is_piece_on_coords((x - 1, y + 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x - 1, y + 1))
                                && self.is_piece_on_coords((x - 1, y + 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x - 1) as usize, (y + 1) as usize))
                            }
                        }
                        None => {}
                    }

                    let piece_on_coords = if self.is_coord_in_board((x + 1, y + 1)) {
                        self.is_piece_on_coords((x + 1, y + 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x + 1, y + 1))
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
                    if self.is_coord_in_board((x, y - 1)) && !self.is_piece_on_coords((x, y - 1)).0
                    {
                        available_moves.push((x as usize, (y - 1) as usize));

                        if piece.did_move == false
                            && self.is_coord_in_board((x, y - 2))
                            && !self.is_piece_on_coords((x, y - 2)).0
                        {
                            available_moves.push((x as usize, (y - 2) as usize));
                        }
                    }

                    let piece_on_coords = if self.is_coord_in_board((x - 1, y - 1)) {
                        self.is_piece_on_coords((x - 1, y - 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x - 1, y - 1))
                                && self.is_piece_on_coords((x - 1, y - 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x - 1) as usize, (y - 1) as usize))
                            }
                        }
                        None => {}
                    }

                    let piece_on_coords = if self.is_coord_in_board((x + 1, y - 1)) {
                        self.is_piece_on_coords((x + 1, y - 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x + 1, y - 1))
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
                        if self.is_coord_in_board(coords)
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
                        if self.is_coord_in_board(coords)
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
                                    if self.is_coord_in_board(coords)
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
                                    if self.is_coord_in_board(coords)
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
        let check_for = if let Some(last) = self.move_records.last() {
            if last.side == Side::Black {
                Side::White
            } else {
                Side::Black
            }
        } else {
            Side::White
        };

        for (move_x, move_y) in moves {
            let board_copy = self.tiles.clone();
            self.tiles[move_y][move_x].piece = self.tiles[y as usize][x as usize].piece;
            self.tiles[y as usize][x as usize].piece = None;
            if !self.is_check(check_for, self.tiles) {
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
    fn is_coord_in_board(&self, (x, y): (i32, i32)) -> bool {
        return x >= 0 && y >= 0 && x < Self::SIZE as i32 && y < Self::SIZE as i32;
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Side {
    Black,
    White,
}

const CHESSBOARD_WIDTH: i32 = 1000;
const CHESSBOARD_HEIGHT: i32 = 1000;
const LEFT_SIDE_PADDING: i32 = 50;
const WINDOW_WIDTH: i32 = 1400;
const WINDOW_HEIGHT: i32 = 1050;
const X_AXIS_LABELS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];
const Y_AXIS_LABELS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Chessio")
        .build();

    let icon = Image::load_image("./static/other/logo.png").unwrap();
    rl.set_window_icon(icon);
    rl.set_window_title(&thread, "Chessio");

    let background_img = Image::load_image("./static/other/wooden-background.png").unwrap();
    let background_img_texture = rl
        .load_texture_from_image(&thread, &background_img)
        .unwrap();

    let mut game = Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        let Vector2 {
            x: mouse_x,
            y: mouse_y,
        } = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            game.start_drag_event((mouse_x - (LEFT_SIDE_PADDING as f32), mouse_y));
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            game.end_drag_event((mouse_x - (LEFT_SIDE_PADDING as f32), mouse_y));
        }

        for (_, _, tile) in game.tiles_iter_mut() {
            tile.clear_bg();
        }

        let mut d = rl.begin_drawing(&thread);

        d.draw_texture_ex(
            &background_img_texture,
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            1.0,
            Color::WHITE,
        );

        let (side_on_turn, side_on_turn_color) = match game.move_records.last() {
            Some(record) => match record.side {
                Side::Black => ("White", Color::WHITE),
                Side::White => ("Black", Color::BLACK),
            },
            None => ("White", Color::WHITE),
        };

        d.draw_text(
            "Turn:",
            CHESSBOARD_WIDTH + LEFT_SIDE_PADDING + 20,
            10,
            46,
            Color::WHITE,
        );
        d.draw_text(
            &(game.move_records.len() + 1).to_string(),
            CHESSBOARD_WIDTH + 220,
            10,
            46,
            Color::WHITE,
        );
        d.draw_text(
            "Move:",
            CHESSBOARD_WIDTH + LEFT_SIDE_PADDING + 20,
            60,
            46,
            Color::WHITE,
        );
        d.draw_text(
            side_on_turn,
            CHESSBOARD_WIDTH + 220,
            60,
            46,
            side_on_turn_color,
        );

        match game.is_check {
            Some(side) => {
                d.draw_text(
                    "CHECK",
                    CHESSBOARD_WIDTH + LEFT_SIDE_PADDING + 20,
                    WINDOW_HEIGHT - 50,
                    46,
                    if side == Side::Black {
                        Color::BLACK
                    } else {
                        Color::WHITE
                    },
                );
            }
            None => {}
        }

        match game.move_records.last() {
            Some(lm) => {
                let last_move_piece = format!("{:?} {:?}", lm.side, lm.kind);
                d.draw_text(
                    "Last move:",
                    CHESSBOARD_WIDTH + LEFT_SIDE_PADDING + 20,
                    110,
                    28,
                    Color::WHITE,
                );
                d.draw_text(
                    &last_move_piece,
                    CHESSBOARD_WIDTH + LEFT_SIDE_PADDING + 20,
                    140,
                    28,
                    Color::WHITE,
                );
                let last_move_text = format!(
                    "{}{} -> {}{}",
                    X_AXIS_LABELS[lm.from.0],
                    Y_AXIS_LABELS[lm.from.1],
                    X_AXIS_LABELS[lm.to.0],
                    Y_AXIS_LABELS[lm.to.1]
                );
                d.draw_text(
                    &last_move_text,
                    CHESSBOARD_WIDTH + LEFT_SIDE_PADDING + 20,
                    170,
                    28,
                    Color::WHITE,
                );
            }
            None => {}
        }

        for (x, y, _) in game.tiles_iter() {
            d.draw_rectangle(
                0,
                (y as i32) * (CHESSBOARD_HEIGHT / 8),
                LEFT_SIDE_PADDING,
                CHESSBOARD_HEIGHT / 8,
                Color::GRAY,
            );
            d.draw_text(
                Y_AXIS_LABELS[7 - y],
                LEFT_SIDE_PADDING / 2 - 9,
                (y as i32) * (CHESSBOARD_HEIGHT / 8) + (CHESSBOARD_HEIGHT / 8 / 2) - 14,
                28,
                Color::WHITE,
            );
        }

        for (x, y, _) in game.tiles_iter() {
            d.draw_rectangle(
                LEFT_SIDE_PADDING + (x as i32) * (CHESSBOARD_WIDTH / 8),
                CHESSBOARD_HEIGHT,
                CHESSBOARD_WIDTH / 8,
                WINDOW_HEIGHT - CHESSBOARD_HEIGHT,
                Color::GRAY,
            );
            d.draw_text(
                X_AXIS_LABELS[x],
                LEFT_SIDE_PADDING
                    + (x as i32) * (CHESSBOARD_WIDTH / 8)
                    + (CHESSBOARD_WIDTH / 8) / 2
                    - 9,
                WINDOW_HEIGHT - (WINDOW_HEIGHT - CHESSBOARD_HEIGHT) / 2 - 14,
                28,
                Color::WHITE,
            );
        }

        if game.hovered_piece_coords.is_none() {
            let hovered_tile =
                game.get_tile_on_coords_mut((mouse_x - (LEFT_SIDE_PADDING as f32), mouse_y));

            match hovered_tile {
                Some(t) => {
                    if t.0.piece.is_some() {
                        d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_POINTING_HAND);
                    } else {
                        d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
                    }
                }
                None => {
                    d.set_mouse_cursor(MouseCursor::MOUSE_CURSOR_DEFAULT);
                }
            }
        }

        let hovered_tile_coords: Option<(usize, usize)> = if let Some((_, (x, y))) =
            game.get_tile_on_coords((mouse_x - (LEFT_SIDE_PADDING as f32), mouse_y))
        {
            Some((x, y))
        } else {
            None
        };

        match hovered_tile_coords {
            Some(coords) => {
                let text = format!("{}{}", X_AXIS_LABELS[coords.0], Y_AXIS_LABELS[7 - coords.1]);
                d.draw_text(
                    &text,
                    WINDOW_WIDTH - 100,
                    WINDOW_HEIGHT - 88,
                    68,
                    Color::WHITE,
                );
            }
            _ => {}
        }

        game.highlight_tile_by_position((mouse_x - (LEFT_SIDE_PADDING as f32), mouse_y));
        game.render(&mut d);

        for y in 0..Game::SIZE {
            for x in 0..Game::SIZE {
                if game.hovered_piece_coords == Some((x, y)) {
                    continue;
                }

                game.tiles[y][x].render(
                    &mut d,
                    (
                        LEFT_SIDE_PADDING + x as i32 * (CHESSBOARD_WIDTH / Game::SIZE as i32),
                        y as i32 * (CHESSBOARD_HEIGHT / Game::SIZE as i32),
                    ),
                    &game.pieces_images,
                );
            }
        }

        game.render_available_moves(&mut d);

        match game.hovered_piece_coords {
            Some(coords) => {
                let tile_at_mouse_coords = game.tiles[coords.1][coords.0];
                // println!("{:#?}", tile_at_mouse_coords);
                match tile_at_mouse_coords.piece {
                    Some(piece) => game.render_piece_at_coords(
                        &mut d,
                        (piece.kind, piece.side),
                        (mouse_x, mouse_y),
                    ),
                    None => {}
                }
            }
            _ => {}
        }

        match game.victor {
            Some(v) => {
                d.draw_text(
                    "CHECKMATE",
                    LEFT_SIDE_PADDING + (CHESSBOARD_WIDTH / 2) - 50,
                    (CHESSBOARD_HEIGHT / 2) - 10,
                    48,
                    Color::RED,
                );
            }
            None => {}
        }
    }
}
