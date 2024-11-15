use std::collections::HashSet;

type Position = (i32, i32);

struct Grid {
    alive_cells: HashSet<Position>,
    upper_left: Position,
    lower_right: Position,
}


impl Grid {
    pub fn new(lower_limit: i32) -> Grid {
        Grid {  alive_cells: HashSet::new(), upper_left: (0, 0), lower_right: (lower_limit, lower_limit) }
    }

}
