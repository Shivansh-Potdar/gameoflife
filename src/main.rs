mod grid;
extern crate das_grid;

fn main() -> Result<(), das_grid::GridErr>{
    let mut main_g = das_grid::Grid::new(10, 10, 0);

    match main_g.set((0, 1), &1) {
        Ok(_) => println!("Initial positions set \r\n"),
        Err(_) => println!("Could not set initial poitions, please check code.")
    };

    print!("{:#?}", main_g);

    Ok(())
}