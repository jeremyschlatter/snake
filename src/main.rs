extern crate piston_window;
extern crate rand;

use std::collections::VecDeque;

use piston_window::*;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let initial_direction = Direction::Right;
    let mut direction = initial_direction;
    let initial_body: VecDeque<(i32, i32)> = VecDeque::from(vec![
        (0, 0),
        (-1, 0),
        (-2, 0),
        (-3, 0),
        (-4, 0),
        (-5, 0),
        (-6, 0),
        (-7, 0),
        (-8, 0),
        (-9, 0),
    ]);
    let mut body = initial_body.clone();
    let seconds_between_updates = 0.05;
    let mut next_update = seconds_between_updates;
    let (board_width, board_height) = (21, 21);
    let cell_size = 20.0;

    let mut rng = thread_rng();
    let mut reset_apple = |body: &VecDeque<(i32, i32)>| loop {
        let apple = (
            rng.gen_range(-(board_width / 2) + 1, board_width / 2),
            rng.gen_range(-(board_height / 2) + 1, board_height / 2),
        );
        if !body.iter().any(|&cell| cell == apple) {
            return apple;
        }
    };
    let mut apple = reset_apple(&body);

    while let Some(e) = window.next() {
        if let Some(args) = e.update_args() {
            next_update -= args.dt;
            if next_update < 0.0 {
                next_update = seconds_between_updates;
            } else {
                continue;
            }
            //let speed = 200.0 * args.dt;
            let (old_x, old_y) = body[0];
            let new_head = match direction {
                Direction::Down => (old_x, old_y + 1),
                Direction::Up => (old_x, old_y - 1),
                Direction::Left => (old_x - 1, old_y),
                Direction::Right => (old_x + 1, old_y),
            };
            let (new_x, new_y) = new_head;
            body.push_front((new_x, new_y));
            if apple == new_head {
                apple = reset_apple(&body);
            } else {
                body.pop_back();
            }
            if body.iter().skip(1).any(|&cell| cell == new_head)
                || new_x.abs() > board_width / 2
                || new_y.abs() > board_height / 2
            {
                // Dead.
                body = initial_body.clone();
                direction = initial_direction;
                apple = reset_apple(&body);
            }
        }
        if let Some(button) = e.release_args() {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::Down => direction = Direction::Down,
                    Key::Up => direction = Direction::Up,
                    Key::Left => direction = Direction::Left,
                    Key::Right => direction = Direction::Right,
                    _ => (),
                }
            }
        }
        if let Some(args) = e.render_args() {
            let other = &body;
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            window.draw_2d(&e, |c, g| {
                let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

                clear(BLACK, g);
                rectangle(
                    WHITE,
                    [0.0, 0.0, args.width as f64, args.height as f64],
                    c.transform,
                    g,
                );
                rectangle(
                    BLACK,
                    [
                        x - cell_size * (board_width / 2) as f64,
                        y - cell_size * (board_height / 2) as f64,
                        cell_size * board_width as f64,
                        cell_size * board_width as f64,
                    ],
                    c.transform,
                    g,
                );

                let (apple_x, apple_y) = apple;
                rectangle(
                    RED,
                    [
                        x + cell_size * apple_x as f64,
                        y + cell_size * apple_y as f64,
                        cell_size,
                        cell_size,
                    ],
                    c.transform,
                    g,
                );

                for (cx, cy) in other {
                    rectangle(
                        WHITE,
                        [
                            x + cell_size * *cx as f64,
                            y + cell_size * *cy as f64,
                            cell_size,
                            cell_size,
                        ],
                        c.transform,
                        g,
                    );
                }
            });
        }
    }
}
