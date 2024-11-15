#[derive(Debug, Clone, PartialEq, Copy)]
enum State {
    Alive,
    Dead,
}

type Position = (usize, usize);

struct Grid {
    cells: Vec<Vec<State>>,
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid {
            cells: vec![vec![State::Dead; size]; size],
        }
    }

    pub fn change_state(&mut self, position: Position, state: State) {
        let (x, y) = position;

        let outer = &mut self.cells[y];
        let inner = &mut outer[x];

        *inner = state;
    }

    fn get_cell(&self, position: Position) -> State {
        let (x, y) = position;

        let outer = &self.cells[y];
        let state = outer[x];
        return state;
    }

    // pub fn update(&mut self) {
    // }

    //TODO: I think this funciton's name could be improved
    fn coordinate_from_position(&self, enumeration: u32) -> Position {
        let n = self.cells.len();

        let x = enumeration % n as u32;
        let x = x as usize;
        let y: f32 = enumeration as f32 / n as f32;
        let y = y.floor() as u32;
        let y = y as usize;

        (x, y)
    }

    fn alive_neighbors_amount(&self, position: &Position) -> u32 {
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
            .map(|change| (change.0 + position.0 as i32, change.1 + position.1 as i32))
            .filter(|(x, y)| {
                let limit: i32 = self.cells.len().try_into().unwrap();
                *x < limit && *y < limit
            });

        let alive_neighbors = neighbors
            .map(|cell| (cell.0 as usize, cell.1 as usize))
            .filter(|cell| self.get_cell(*cell) == State::Alive)
            .count();

        alive_neighbors as u32
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
        grid.change_state((3, 3), State::Alive);
        assert_eq!(grid.get_cell((3, 3)), State::Alive);
        grid.change_state((3, 3), State::Dead);
        assert_eq!(grid.get_cell((3, 3)), State::Dead);
        grid.change_state((3, 3), State::Alive);
        assert_eq!(grid.get_cell((3, 3)), State::Alive);
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

        let alive = grid.alive_neighbors_amount(&(3, 3));
        assert_eq!(3, alive);
    }
}
