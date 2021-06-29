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
    let mut plc_x = 0;
    let mut plc_y = 0;
    for (x, y) in main_g.enumerate(){
        //add thread to do at same time 
        //Code to loop through rows
        while plc_x <= x {
            plc_x += 1;
            println!("This is  row: {}", plc_x);
        }
        //Code to loop through coloumns
        while plc_y <= y {
            plc_y += 1;
            println!("This is  coloumn: {}", plc_y);
        }
    }

    print!("{:?}", main_g);

    Ok(())
}