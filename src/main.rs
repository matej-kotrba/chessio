pub mod chess;

use chess::{constants::*, piece::Side, Game};
use raylib::prelude::*;

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

        for y in 0..CHESSBOARD_SIZE {
            for x in 0..CHESSBOARD_SIZE {
                if game.hovered_piece_coords == Some((x, y)) {
                    continue;
                }

                game.tiles[y][x].render(
                    &mut d,
                    (
                        LEFT_SIDE_PADDING + x as i32 * (CHESSBOARD_WIDTH / CHESSBOARD_SIZE as i32),
                        y as i32 * (CHESSBOARD_HEIGHT / CHESSBOARD_SIZE as i32),
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
