use std::collections::HashMap;

use raylib::prelude::*;

type PiecesImagesType = HashMap<(PieceType, Side), Texture2D>;

struct Game {
    tiles: [[Tile; Self::SIZE]; Self::SIZE],
    pieces_images: PiecesImagesType,
    hovered_piece_coords: Option<(usize, usize)>,
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
    pub fn render_available_moves(&mut self) {
        match self.hovered_piece_coords {
            Some(coords) => {
                let moves = self.get_piece_available_moves((coords.0 as i32, coords.1 as i32));

                for mov in moves {
                    self.tiles[mov.1][mov.0].bg = Some(Color::YELLOW);
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
                    x: x - ((WINDOW_WIDTH / Self::SIZE as i32) / 2) as f32,
                    y: y - ((WINDOW_HEIGHT / Self::SIZE as i32) / 2) as f32,
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

        // self.tiles[4][4].piece = Some(Piece::new(PieceType::King, Side::White));
        // self.tiles[2][4].piece = Some(Piece::new(PieceType::King, Side::Black));
    }
    pub fn get_tile_on_coords(&self, (x, y): (f32, f32)) -> Option<Tile> {
        let tile_x = (x / (WINDOW_WIDTH as f32 / Self::SIZE as f32)) as i32;
        let tile_y = (y / (WINDOW_HEIGHT as f32 / Self::SIZE as f32)) as i32;

        if tile_x >= 0 && tile_x < Self::SIZE as i32 && tile_y >= 0 && tile_y < Self::SIZE as i32 {
            return Some(self.tiles[tile_y as usize][tile_x as usize]);
        }

        None
    }
    pub fn highlight_tile_by_position(&mut self, (x, y): (f32, f32)) {
        let tile_x = (x / (WINDOW_WIDTH as f32 / Self::SIZE as f32)) as i32;
        let tile_y = (y / (WINDOW_HEIGHT as f32 / Self::SIZE as f32)) as i32;

        if tile_x >= 0 && tile_x < Self::SIZE as i32 && tile_y >= 0 && tile_y < Self::SIZE as i32 {
            self.tiles[tile_y as usize][tile_x as usize].bg = Some(Color::RED);
        }
    }
    pub fn start_drag_event(&mut self, (x, y): (f32, f32)) {
        let tile_x = (x / (WINDOW_WIDTH as f32 / Self::SIZE as f32)) as i32;
        let tile_y = (y / (WINDOW_HEIGHT as f32 / Self::SIZE as f32)) as i32;

        if tile_x >= 0 && tile_x < Self::SIZE as i32 && tile_y >= 0 && tile_y < Self::SIZE as i32 {
            self.hovered_piece_coords = Some((tile_x as usize, tile_y as usize));
        } else {
            self.hovered_piece_coords = None
        }
    }
    fn get_pieces_linear_moves<'a>(
        &'a self,
        available_moves: &'a mut Vec<(usize, usize)>,
        (temp_x, temp_y): (i32, i32),
        (move_x, move_y): (i32, i32),
        piece_side: Side,
    ) {
        if Self::is_coord_in_board(&self, (temp_x as i32, temp_y as i32)) {
            let p = self.is_piece_on_cords((temp_x, temp_y));
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
                    if self.is_coord_in_board((x, y + 1)) && !self.is_piece_on_cords((x, y + 1)).0 {
                        available_moves.push((x as usize, (y + 1) as usize));

                        if piece.did_move == false
                            && self.is_coord_in_board((x, y + 2))
                            && !self.is_piece_on_cords((x, y + 2)).0
                        {
                            available_moves.push((x as usize, (y + 2) as usize));
                        }
                    }

                    let piece_on_coords = if self.is_coord_in_board((x - 1, y + 1)) {
                        self.is_piece_on_cords((x - 1, y + 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x - 1, y + 1))
                                && self.is_piece_on_cords((x - 1, y + 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x - 1) as usize, (y + 1) as usize))
                            }
                        }
                        None => {}
                    }

                    let piece_on_coords = if self.is_coord_in_board((x + 1, y + 1)) {
                        self.is_piece_on_cords((x + 1, y + 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x + 1, y + 1))
                                && self.is_piece_on_cords((x + 1, y + 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x + 1) as usize, (y + 1) as usize))
                            }
                        }
                        None => {}
                    }
                }
                Side::White => {
                    if self.is_coord_in_board((x, y - 1)) && !self.is_piece_on_cords((x, y - 1)).0 {
                        available_moves.push((x as usize, (y - 1) as usize));

                        if piece.did_move == false
                            && self.is_coord_in_board((x, y - 2))
                            && !self.is_piece_on_cords((x, y - 2)).0
                        {
                            available_moves.push((x as usize, (y - 2) as usize));
                        }
                    }

                    let piece_on_coords = if self.is_coord_in_board((x - 1, y - 1)) {
                        self.is_piece_on_cords((x - 1, y - 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x - 1, y - 1))
                                && self.is_piece_on_cords((x - 1, y - 1)).0
                                && piece.side != p
                            {
                                available_moves.push(((x - 1) as usize, (y - 1) as usize))
                            }
                        }
                        None => {}
                    }

                    let piece_on_coords = if self.is_coord_in_board((x + 1, y - 1)) {
                        self.is_piece_on_cords((x + 1, y - 1))
                    } else {
                        (false, None)
                    };

                    match piece_on_coords.1 {
                        Some(p) => {
                            if self.is_coord_in_board((x + 1, y - 1))
                                && self.is_piece_on_cords((x + 1, y - 1)).0
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
                            && self.is_piece_on_cords(coords).1 != Some(Side::Black)
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
                            && self.is_piece_on_cords(coords).1 != Some(Side::White)
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
                                for cords in coords_around_king {
                                    if self.is_coord_in_board(cords)
                                        && Self::get_distance_between_direct_coords(
                                            (cords.0 as usize, cords.1 as usize),
                                            enemy_king_coords,
                                        ) > 1
                                        && self.is_piece_on_cords(cords).1 != Some(Side::Black)
                                    {
                                        available_moves.push((cords.0 as usize, cords.1 as usize));
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
                                for cords in coords_around_king {
                                    if self.is_coord_in_board(cords)
                                        && Self::get_distance_between_direct_coords(
                                            (cords.0 as usize, cords.1 as usize),
                                            enemy_king_coords,
                                        ) > 1
                                        && self.is_piece_on_cords(cords).1 != Some(Side::White)
                                    {
                                        available_moves.push((cords.0 as usize, cords.1 as usize));
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        // println!("{:#?}", available_moves);

        available_moves
    }
    fn is_piece_on_cords(&self, (x, y): (i32, i32)) -> (bool, Option<Side>) {
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
        let Vector2 {
            x: mouse_x,
            y: mouse_y,
        } = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            game.start_drag_event((mouse_x, mouse_y));
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            game.hovered_piece_coords = None;
        }

        for (_, _, tile) in game.tiles_iter_mut() {
            tile.clear_bg();
        }

        game.highlight_tile_by_position((mouse_x, mouse_y));
        game.render_available_moves();

        let mut d = rl.begin_drawing(&thread);

        game.render(&mut d);

        for y in 0..Game::SIZE {
            for x in 0..Game::SIZE {
                if game.hovered_piece_coords == Some((x, y)) {
                    continue;
                }

                game.tiles[y][x].render(
                    &mut d,
                    (
                        x as i32 * (WINDOW_WIDTH / Game::SIZE as i32),
                        y as i32 * (WINDOW_HEIGHT / Game::SIZE as i32),
                    ),
                    &game.pieces_images,
                );
            }
        }

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
    }
}
