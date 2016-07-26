use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

const GRID_SIZE: usize = 32;

//allow Cells to be copied, eg. into an array or as a parameter
#[derive(Copy, Clone)]
struct  Cell { 
    alive: i32, x:i32, y:i32
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
    
    fn print_all_cells(&self){
        for i in 0..GRID_SIZE{
            for j in 0..GRID_SIZE{
                if self.inner[i][j].alive == 1{
                    print!("O");
                }
                else{
                    print!("-");
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
}



fn main() {
    let grid = Arc::new(Mutex::new(Grid::new()));
    let (tx, rx)= mpsc::channel();
    grid.number_all_cells();;
    loop{
        grid.print_all_cells();
        for _ in 0..10{
            println!("");
        }
        for f in 1..GRID_SIZE{
            for b in 1..GRID_SIZE{
                let count =Arc::new(Mutex::new(0));
                let (grid, count, tx) = (grid.clone(), count.clone(), tx.clone());
                for n in 1..9{
                    thread::spawn(move || {
                        let mut grid=grid.lock().unwrap();
                        match n {
                            1=> if grid.inner[f-1][b-1].alive==1 {count+=1;}
                            2=> if grid.inner[f][b-1].alive==1 {count+=1;}
                            3=> if grid.inner[f+1][b-1].alive==1 {count+=1;}
                            4=> if grid.inner[f+1][b].alive==1 {count+=1;}
                            5=> if grid.inner[f+1][b+1].alive==1 {count+=1;}
                            6=> if grid.inner[f][b+1].alive==1 {count+=1;}
                            7=> if grid.inner[f-1][b+1].alive==1 {count+=1;}
                            8=> if grid.inner[f-1][b].alive==1 {count+=1;}
                        }
                        tx.send(()).unwrap();
                    }
                    )
                }
                for _ in 1..9{
                    rx.recv().unwrap();
                }
            }
        }
    }
}





