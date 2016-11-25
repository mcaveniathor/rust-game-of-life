extern crate rand;
//extern crate pancurses;
use std::thread;
use std::time::Duration;
use rand::Rng;
//use pancurses::*;
const GRID_SIZE: usize = 32;
const GENERATIONS: i32 = 1000;
//allow Cells to be copied, eg. into an array or as a parameter
#[derive(Copy, Clone)]
struct  Cell { 
    alive: u8, x:i32, y:i32
}

// Cell constructor defaults to dead
impl Cell {
    fn new() -> Cell {
        return Cell{alive:0, x:0, y:0}
    }
}

#[derive(Copy, Clone)]
struct Grid {
    inner: [[Cell; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    //creates a square array of new cells
    fn new() -> Grid {
        return Grid {inner: [[Cell::new(); GRID_SIZE]; GRID_SIZE]}
    }
    
    fn set_all_cells_random(&mut self){
        let mut rng = rand::thread_rng();
        for i in 1..GRID_SIZE{
            for j in 1..GRID_SIZE{
                self.inner[i][j].alive=rng.gen::<bool>() as u8;
            }
        }
    }
    fn print_all_cells(&self){
        for i in 0..GRID_SIZE{
            for j in 0..GRID_SIZE{
                if i==0 || j==0 || i==GRID_SIZE-1 || j==GRID_SIZE-1{
                    print!("*");
                }
                else{
                    if self.inner[i][j].alive == 1{
                        print!("1");
                    }
                    else{
                        print!("0");
                    }
                }
            }
            println!("");
        }
    }

    fn number_all_cells(&mut self){
        for i in 0..GRID_SIZE{                        
            for j in 0..GRID_SIZE{
                self.inner[i][j].x=i as i32;
                self.inner[i][j].y=j as i32;
            }
        }
    }

    fn update(&mut self){
        for f in 1..GRID_SIZE-1{
            for b in 1..GRID_SIZE-1{
                let mut count:usize=0;
                if f>=1 && b >=1 && self.inner[f-1][b-1].alive == 1 { count +=1;}
                if b >=1 && self.inner[f][b-1].alive == 1 { count +=1;}
                if f<=GRID_SIZE-2 && b >=1 && self.inner[f+1][b-1].alive == 1 { count +=1;}
                if f <=GRID_SIZE-2 && self.inner[f+1][b].alive == 1 { count +=1;}
                if f <= GRID_SIZE-2 && b<= GRID_SIZE-2 && self.inner[f+1][b+1].alive == 1 { count +=1;}
                if b <= GRID_SIZE-2 && self.inner[f][b+1].alive == 1 { count +=1; }
                if f >=1 && b <= GRID_SIZE-2 && self.inner[f-1][b+1].alive == 1 { count +=1;}
                //println!("{}",count);
                if self.inner[f-1][b].alive == 1 { count +=1;}                   
                if self.inner[f][b].alive == 1{
                    if count != 2 && count !=3{
                        self.inner[f][b].alive=0;
                    }
                }
                else {
                    if count == 3{
                        self.inner[f][b].alive=1;
                    }
                
                }
            }
        }
    }
}



fn main() {        
    let mut grid = Grid::new();
    grid.number_all_cells();;
    grid.set_all_cells_random();
    print!("{}[2J", 27 as char);
    for _ in 0..GENERATIONS{
        grid.print_all_cells();
        print!("{}[2J", 27 as char);
        grid.update();
        thread::sleep(Duration::from_millis(10));
    }
}





