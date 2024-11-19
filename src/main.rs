use macroquad::prelude::*;

const UPDATE_INTERVAL: f64 = 0.3;

const SPACE_FOR_TEXT: i32 = 200;

mod conways;

use conways::*;

fn conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_height: 600 + SPACE_FOR_TEXT,
        window_width: 600,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut keys: [KeyCode; 10] = [KeyCode::Space; 10];
    let mut offset = 0;

    let mut grid = Grid::new(22);

    // Glider 
    grid.change_state((2, 1), State::Alive);
    grid.change_state((2, 2), State::Alive);
    grid.change_state((2, 3), State::Alive);
    grid.change_state((1, 3), State::Alive);
    grid.change_state((0, 2), State::Alive);

    let mut last_updated = get_time();

    let cube_width = 30.0;
    let cube_height = 30.0;

    let mut pause = true;

    loop {
        clear_background(GRAY);
        if let Some(key) = get_last_key_pressed() {
            keys[offset] = key;
            offset += 1;
            if offset == keys.len() {
                offset = 0;
            }
            if key == KeyCode::P {
                pause = !pause;
            }
        }

        for x in 0..grid.get_height() {
            for y in 0..grid.get_width() {
                let color = match grid.get_state((x, y)) {
                    State::Alive => BLACK,
                    State::Dead => WHITE,
                };

                let offset_x = 0.0 + (cube_width + 0.0) * x as f32;
                let offset_y = 0.0 + (cube_height + 0.0) * y as f32;

                draw_rectangle(offset_x, offset_y, cube_width, cube_height, color);
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_x = (mouse_x / cube_width).ceil() - 1.0;
            let mouse_y = (mouse_y / cube_height).ceil() - 1.0;
            grid.toggle_state((mouse_x as usize, mouse_y as usize));
        }

        if get_time() - last_updated > UPDATE_INTERVAL && !pause {
            last_updated = get_time();
            grid.update();
        }

        draw_text("P to (un)pause", 0.0, screen_height() - 50.0, 60.0, BLACK);

        pause = check_for_events(&keys, &mut grid, pause);

        next_frame().await
    }
}

fn check_for_events(keys: &[KeyCode; 10], grid: &mut Grid, pause: bool) -> bool {
    match keys {
        [KeyCode::Up, KeyCode::Up, KeyCode::Down, KeyCode::Down, KeyCode::Left, KeyCode::Right, KeyCode::Left, KeyCode::Right, KeyCode::A, KeyCode::B] =>
        {
            grid.kill_all();

            // Hmm, I wonder what this does 🤔
            grid.change_state((9, 1), State::Alive);
            grid.change_state((8, 1), State::Alive);
            grid.change_state((7, 2), State::Alive);
            grid.change_state((10, 2), State::Alive);
            grid.change_state((10, 3), State::Alive);
            grid.change_state((10, 4), State::Alive);
            grid.change_state((11, 5), State::Alive);
            grid.change_state((11, 6), State::Alive);
            grid.change_state((11, 7), State::Alive);
            grid.change_state((12, 8), State::Alive);
            grid.change_state((12, 9), State::Alive);
            grid.change_state((13, 10), State::Alive);
            grid.change_state((14, 10), State::Alive);
            grid.change_state((15, 9), State::Alive);

            grid.change_state((10, 8), State::Alive);
            grid.change_state((9, 9), State::Alive);
            grid.change_state((8, 10), State::Alive);

            grid.change_state((0, 15), State::Alive);
            grid.change_state((1, 15), State::Alive);
            grid.change_state((2, 15), State::Alive);
            grid.change_state((0, 14), State::Alive);
            grid.change_state((0, 13), State::Alive);
            grid.change_state((0, 12), State::Alive);
            grid.change_state((1, 12), State::Alive);
            grid.change_state((2, 12), State::Alive);

            grid.change_state((4, 14), State::Alive);
            grid.change_state((4, 13), State::Alive);
            grid.change_state((4, 12), State::Alive);
            grid.change_state((4, 15), State::Alive);
            grid.change_state((5, 15), State::Alive);
            grid.change_state((6, 15), State::Alive);

            grid.change_state((8, 14), State::Alive);
            grid.change_state((8, 13), State::Alive);
            grid.change_state((8, 12), State::Alive);
            grid.change_state((8, 15), State::Alive);
            grid.change_state((10, 14), State::Alive);
            grid.change_state((10, 13), State::Alive);
            grid.change_state((10, 12), State::Alive);
            grid.change_state((10, 15), State::Alive);
            grid.change_state((9, 12), State::Alive);
            grid.change_state((9, 14), State::Alive);

            grid.change_state((12, 13), State::Alive);
            grid.change_state((13, 12), State::Alive);
            grid.change_state((14, 12), State::Alive);
            grid.change_state((12, 15), State::Alive);
            grid.change_state((13, 15), State::Alive);
            grid.change_state((14, 14), State::Alive);

            grid.change_state((16, 13), State::Alive);
            grid.change_state((17, 12), State::Alive);
            grid.change_state((18, 12), State::Alive);
            grid.change_state((16, 15), State::Alive);
            grid.change_state((17, 15), State::Alive);
            grid.change_state((18, 14), State::Alive);

            true
        }
        _ => pause,
    }
}
