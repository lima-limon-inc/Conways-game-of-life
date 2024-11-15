use std::collections::HashSet;

type Limit = (i32, i32);

struct Grid {
    alive_cells: HashSet<(i32, i32)>,
    upper_left: Limit,
    lower_right: Limit,
}


impl Grid {
    pub fn new(lower_limit: i32) -> Grid {
        Grid {  alive_cells: HashSet::new(), upper_left: (0, 0), lower_right: (lower_limit, lower_limit) }
    }

}
