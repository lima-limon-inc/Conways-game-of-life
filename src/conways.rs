#[derive(Debug, Clone)]
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

        let outer = &mut self.cells[x];
        let inner = &mut outer[y];

        *inner = state;
    }

    pub fn update(&mut self) {
    }

    fn update_cells(&mut self) {
        // let new_grid = HashSet::new();
        let iter = self.cells.iter().flatten().enumerate();
        let a: Vec<_>  = iter.map(|a| println!("Jamon {}: {:?}", a.0, a.1)).collect();

    }

}

