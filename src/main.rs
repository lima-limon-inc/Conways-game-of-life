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

    // grid.change_state((2, 2), State::Alive);
    // grid.change_state((2, 3), State::Alive);
    // grid.change_state((1, 3), State::Alive);
    // grid.change_state((0, 2), State::Alive);

    
    let mut last_updated = get_time();

    let cube_width  = 30.0;
    let cube_height = 30.0;

    loop {
        clear_background(GRAY);
        println!("{:?}", keys);

        if let Some(key) = get_last_key_pressed() {
	  keys[offset] = key;
	  offset += 1;
	  if offset == keys.len() {
	      offset = 0;
	  }
        }

        for x in 0..grid.get_height() {
	  for y in 0..grid.get_width() {
	      let color = match grid.get_state((x, y)) {
		State::Alive => BLACK,
		State::Dead  => WHITE,
	      };

	      let offset_x = 0.0 + (cube_width + 0.0) * x as f32;
	      let offset_y = 0.0 + (cube_height + 0.0) * y as f32;

	      // println!("{:?}", (x,y));
	      draw_rectangle(offset_x as f32,  offset_y as f32, cube_width, cube_height, color);
	  }
        }

        draw_text("P to pause", 0.0, screen_height() - 50.0, 60.0, BLACK);

        if get_time() - last_updated > UPDATE_INTERVAL {
            last_updated = get_time();
	  grid.update();
        }


        next_frame().await
    }
}
