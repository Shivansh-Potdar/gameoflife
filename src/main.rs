extern crate das_grid;

fn main() -> Result<(), das_grid::GridErr>{
    let mut main_g = das_grid::Grid::new(10, 10, 0);
    
    let mut empty_cell = 0;
    let mut liv_n = 0;
    let mut l_c = 0;

    loop {

        liv_n = 0;

        //Shape
        match main_g.set((0, 1), &1) {
            Ok(_) => (),
            Err(_) => println!("Could not set initial poitions, please check code.")
        };

        match main_g.set((1, 2), &1) {
            Ok(_) => (),
            Err(_) => println!("Could not set initial poitions, please check code.")
        };

        match main_g.set((2, 1), &1) {
            Ok(_) => (),
            Err(_) => println!("Could not set initial poitions, please check code.")
        };

        //For loop to check and update states
        for (x, y) in main_g.enumerate(){
            if x!= 0 && y != 0 || x != 1 && y != 1{
                //check neighbours above
                match main_g.get((x-1, y)) {
                    Ok(&1) => {liv_n+=1;},
                    _=> { empty_cell+=1; print!("");}
                };

                //check neighbours left
                match main_g.get((x, y-1)) {
                    Ok(&1) => {liv_n+=1;},
                    _=> { empty_cell+=1; print!("");}
                };

                //check neighbours below
                match main_g.get((x+1, y)) {
                    Ok(&1) => {liv_n+=1;},
                    _=> { empty_cell+=1; print!("");}
                };

                //check neighbours right
                match main_g.get((x, y+1)) {
                    Ok(&1) => {liv_n+=1;},
                    _=> { empty_cell+=1; print!("");}
                };
            } else {
                //check neighbours below
                match main_g.get((x+1, y)) {
                    Ok(&1) => {liv_n+=1; },
                    _=> { empty_cell+=1; print!("");}
                };

                //check neighbours right
                match main_g.get((x, y+1)) {
                    Ok(&1) => {liv_n+=1;},
                    _=> { empty_cell+=1; print!("");}
                };
            }

            match liv_n {
                i if i >= 4 => main_g.set((x, y), &0)?,
                i if i <= 1 => main_g.set((x, y), &0)?,
                2 | 3 => main_g.set((x, y), &1)?,

                _=> println!("no match")
            }
        };

        for (x, y) in main_g.enumerate() {
            if main_g.get((x, y)) == Ok(&1) {
                println!("x {} y {}", x, y);
                println!("{:?}", main_g);
            }
        };

        l_c+=1;

        if l_c == 3{
            break;
        }
    }
    println!("{}", liv_n);

    //pause code
    pause();

    Ok(())
}

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}