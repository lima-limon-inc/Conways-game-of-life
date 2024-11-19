use macroquad::prelude::*;

const UPDATE_INTERVAL: f64 = 1.0;

mod conways;

use conways::*;


#[macroquad::main("BasicShapes")]
async fn main() {
    let mut grid = Grid::new(5);

    grid.change_state((1, 1), State::Alive);
    grid.change_state((1, 2), State::Alive);
    grid.change_state((1, 3), State::Alive);

    let mut last_updated = get_time();

    loop {
        clear_background(GRAY);

        for x in 0..grid.get_height() {
	  for y in 0..grid.get_width() {
	      let color = match grid.get_state((x, y)) {
		State::Alive => BLACK,
		State::Dead  => WHITE,
	      };

	      let offset_x = grid.get_width() + 100 * x;
	      let offset_y = grid.get_height() + 100 * y;

	      println!("{:?}", (x,y));
	      draw_rectangle(offset_x as f32,  offset_y as f32, 50.0, 50.0, color);
	  }
        }

        if get_time() - last_updated > UPDATE_INTERVAL {
            last_updated = get_time();
	  grid.update();
        }


        next_frame().await
    }
}
