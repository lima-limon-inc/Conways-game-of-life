use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum State {
    Alive,
    Dead,
}

type Position = (usize, usize);

pub struct Grid {
    cells: Vec<Vec<State>>,
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut representation = String::new();
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if self.get_state((j, i)) == State::Alive {
                    representation.push('A');
                } else {
                    representation.push('D');
                }
            }

            representation.push('\n');
        }
        write!(f, "{}", representation)
    }
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            cells: vec![vec![State::Dead; size]; size],
        }
    }

    pub fn get_size(&self) -> usize {
        self.cells.len()
    }

    pub fn change_state(&mut self, (x, y): Position, state: State) {
        let outer = &mut self.cells[x];
        let inner = &mut outer[y];

        *inner = state;
    }

    pub fn toggle_state(&mut self, position: Position) {
        match self.get_state(position) {
            State::Alive => self.change_state(position, State::Dead),
            State::Dead => self.change_state(position, State::Alive),
        }
    }

    pub fn get_state(&self, (x, y): Position) -> State {
        let outer = &self.cells[x];
        outer[y]
    }

    pub fn kill_all(&mut self) {
        let new_cells = vec![vec![State::Dead; self.cells.len()]; self.cells.len()];
        self.cells = new_cells;
    }

    fn determine_new_state(&self, position: Position, current_state: &State) -> State {
        let alive_neighbours = self.alive_neighbors_amount(position);

        match (*current_state, alive_neighbours) {
            (State::Alive, 2 | 3) => State::Alive,
            (State::Dead, 3) => State::Alive,
            _ => State::Dead,
        }
    }

    //TODO: I think this funciton's name could be improved
    fn coordinate_from_position(&self, position: usize) -> Position {
        let n = self.cells.len();

        let x = position as u32 % n as u32;
        let x = x as usize;
        let y: f32 = position as f32 / n as f32;
        let y = y.floor() as u32;
        let y = y as usize;

        (x, y)
    }

    // TODO: THis could use the Display trait instead of being
    // a separate function
    // #[cfg(test)]
    // fn show_display(&self) {}

    fn alive_neighbors_amount(&self, (pos_x, pos_y): Position) -> u32 {
        let neighbor_change: [(i32, i32); 8] = [
            (0, 1),
            (1, 0),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ];

        let neighbors = neighbor_change
            .iter()
            .map(|(change_x, change_y)| (change_x + pos_x as i32, change_y + pos_y as i32))
            .filter(|(x, y)| {
                let limit: i32 = self.cells.len().try_into().unwrap();
                *x < limit && *y < limit
            })
            .filter(|(x, y)| *x >= 0 && *y >= 0);

        let alive_neighbors = neighbors
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|cell| self.get_state(*cell) == State::Alive)
            .count();

        alive_neighbors as u32
    }

    pub fn update(&mut self) {
        let new_states: Vec<_> = self
            .cells
            .iter()
            .flatten()
            .enumerate()
            //Turn enumerate into coordinates
            .map(|(flat_position, _)| self.coordinate_from_position(flat_position))
            .map(|a| (a, self.get_state(a)))
            .map(|(coordinate, state)| (coordinate, self.determine_new_state(coordinate, &state)))
            .collect();

        // Apply side effects
        for (coordinate, state) in &new_states {
            self.change_state(*coordinate, *state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_true() {
        assert!(true);
    }

    #[test]
    fn update_cells() {
        let mut grid = Grid::new(5);
        assert_eq!(grid.get_state((0, 0)), State::Dead);
        grid.change_state((3, 3), State::Alive);
        assert_eq!(grid.get_state((3, 3)), State::Alive);
        grid.change_state((3, 3), State::Dead);
        assert_eq!(grid.get_state((3, 3)), State::Dead);
        grid.change_state((3, 3), State::Alive);
        assert_eq!(grid.get_state((3, 3)), State::Alive);
    }

    #[test]
    fn translate_pos_into_coordinate() {
        let grid = Grid::new(5);
        // Matrix of size 5
        // 0  1  2  3  4
        // 5  6  7  8  9
        // 10 11 12 13 14
        // 15 16 17 18 19
        // 20 21 22 23 24
        assert_eq!((0, 0), grid.coordinate_from_position(0));
        assert_eq!((3, 2), grid.coordinate_from_position(13));

        assert_eq!((1, 3), grid.coordinate_from_position(16));

        assert_eq!((4, 0), grid.coordinate_from_position(4));

        assert_eq!((4, 4), grid.coordinate_from_position(24));
    }

    #[test]
    fn get_live_neighbors() {
        let mut grid = Grid::new(5);
        // Matrix of size 5
        // 0  1  2  3  4
        // 5  6  7  8  9
        // 10 11 12 13 14
        // 15 16  A 18  A
        // 20 21 22  A 24

        grid.change_state((4, 3), State::Alive);
        grid.change_state((3, 4), State::Alive);
        grid.change_state((2, 3), State::Alive);

        let alive = grid.alive_neighbors_amount((3, 3));
        assert_eq!(3, alive);
    }

    #[test]
    fn get_live_neighbors_all_dead() {
        let mut grid = Grid::new(5);
        // Matrix of size 5
        // 0  1  2  3  4
        // 5  6  7  8  9
        // 10 11 12 13 14
        // 15 16  A 18  A
        // 20 21 22  A 24

        let alive = grid.alive_neighbors_amount((0, 1));
        assert_eq!(0, alive);
    }

    #[test]
    fn kill_cell_overpopulation() {
        let mut grid = Grid::new(5);
        // Matrix of size 5
        // 0  1  2  3  4
        // 5  6  7  8  9
        // 10 11 12 A 14
        // 15 16  A A>D  A
        // 20 21 22  A 24

        //18 should die

        grid.change_state((3, 2), State::Alive);
        grid.change_state((2, 3), State::Alive);
        grid.change_state((3, 4), State::Alive);
        grid.change_state((4, 3), State::Alive);

        grid.change_state((3, 3), State::Alive);

        grid.update();
        assert_eq!(grid.get_state((3, 3)), State::Dead);
    }

    #[test]
    fn kill_cell_starvation() {
        let mut grid = Grid::new(5);
        // Matrix of size 5
        // 0  1  2  3  4
        // 5  6  7  8  9
        // 10 11 12 13 14
        // 15 16  A 18 19
        // 20 21 22 23 24

        //17 should die, due to starvation

        grid.change_state((2, 3), State::Alive);

        grid.update();
        assert_eq!(grid.get_state((2, 3)), State::Dead);
    }

    #[test]
    fn keep_cell_alive_2_neighbours() {
        let mut grid = Grid::new(5);
        // Matrix of size 5
        // A  1  2  3  4
        // A  6  7  8  9
        // A 11 12 13 14
        // 15 16 17 18 19
        // 20 21 22 23 24

        //5 should live, due to 2 neighbours

        grid.change_state((0, 0), State::Alive);
        grid.change_state((0, 2), State::Alive);

        grid.change_state((0, 1), State::Alive);

        // grid.show_display();
        println!("{:?}", grid);
        grid.update();
        assert_eq!(grid.get_state((0, 1)), State::Alive);
    }

    #[test]
    fn revive_cell_reproduction() {
        let mut grid = Grid::new(5);
        // Matrix of size 5
        // A  1  2  3  4
        // 5  A  7  8  9
        // A 11 12 13 14
        // 15 16 17 18 19
        // 20 21 22 23 24

        //5 should revive, due to 3 neighbours

        grid.change_state((0, 0), State::Alive);
        grid.change_state((0, 2), State::Alive);
        grid.change_state((1, 1), State::Alive);

        // It should start out dead. We kill the cell just in case.
        grid.change_state((0, 1), State::Dead);

        println!("{:?}", grid);
        grid.update();
        assert_eq!(grid.get_state((0, 1)), State::Alive);
    }
}
