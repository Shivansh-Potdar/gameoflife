pub struct Grid{
    x: u8,
    y: u8
}

impl Grid{
    fn new(x: u8, y: u8) -> Self{
        Self {
            x,
            y
        }
    }

    fn check_neghbours(cell_x: u8, cell_y: u8) -> u8{
        /* check up down and left directions for ell neighours by incremneting or decremint x and y values */
        0
    }

    fn cell_state(cell_x: u8, cell_y: u8) -> bool{
        /* return boolean of cell state depeing on neghbours alive-true, dead-false*/
        true
    }
}

pub fn run(the_grid: Grid){
    println!("{}, {}", the_grid.x, the_grid.y);
}