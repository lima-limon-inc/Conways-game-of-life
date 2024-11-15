#[derive(Debug, Clone, PartialEq)]
enum State {
    Alive,
    Dead
}

type Position = (u32, u32);

struct Grid {
    cells: Vec<Vec<State>>,
}


impl Grid {
    pub fn new(size: usize) -> Grid {
        Grid { cells: vec![vec![State::Dead; size]; size], } 
    }

    pub fn change_state(&mut self, position: Position, state: State) {
        let x: usize = position.0 as usize;
        let y: usize = position.1 as usize;

        let outer = &mut self.cells[y];
        let inner = &mut outer[x];

        *inner = state;
    }

    pub fn update(&mut self) {
    }

    //TODO: I think this funciton's name could be improved
    fn coordinate_from_position(&self, enumeration: u32) -> Position {

        let n = self.cells.len();

        let x = enumeration % n as u32;
        let y: f32 = enumeration as f32 / n as f32;
        let y = y.floor() as u32;

        (x, y)
    }

    fn update_cells(&mut self) {
        // let new_grid = HashSet::new();
        let iter = self.cells.iter().flatten().enumerate();
        let a: Vec<_>  = iter.map(|a| println!("Cell value {}: {:?}", a.0, a.1)).collect();
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
        grid.change_state((3,3), State::Alive);
        assert_eq!(grid.cells[3][3], State::Alive);
        grid.change_state((3,3), State::Dead);
        assert_eq!(grid.cells[3][3], State::Dead);
        grid.change_state((3,3), State::Alive);
        assert_eq!(grid.cells[3][3], State::Alive);

        grid.update_cells();
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


}
