extern crate das_grid;

fn main() -> Result<(), das_grid::GridErr>{
    let mut main_g = das_grid::Grid::new(10, 10, 0);

    //Glider Shape set
    match main_g.set((0, 1), &1) {
        Ok(_) => println!("Initial positions set \r\n"),
        Err(_) => println!("Could not set initial poitions, please check code.")
    };

    match main_g.set((1, 2), &1) {
        Ok(_) => println!("Initial positions set \r\n"),
        Err(_) => println!("Could not set initial poitions, please check code.")
    };

    match main_g.set((2, 2), &1) {
        Ok(_) => println!("Initial positions set \r\n"),
        Err(_) => println!("Could not set initial poitions, please check code.")
    };

    match main_g.set((2, 1), &1) {
        Ok(_) => println!("Initial positions set \r\n"),
        Err(_) => println!("Could not set initial poitions, please check code.")
    };

    match main_g.set((2, 0), &1) {
        Ok(_) => println!("Initial positions set \r\n"),
        Err(_) => println!("Could not set initial poitions, please check code.")
    };

    //For loop to check and update states
    for (x, y) in main_g.enumerate(){
        //println!("({}, {})", x, y);
        if main_g.get((x, y)) == Ok(&1) {
            println!("({}, {})", x, y);
        }
    };

    print!("{:?}", main_g);

    Ok(())
}