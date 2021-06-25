extern crate das_grid;

fn main() -> Result<(), das_grid::GridErr>{
    let mut grid = das_grid::Grid::new_from_vector(2, 2, vec![1, 2, 3, 4]);

    for (x, y) in grid.enumerate() {
        println!("x: {}, y: {}", x, y);
    }

    Ok(())
}