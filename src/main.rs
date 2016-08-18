use std::fmt;

struct LifeGrid {
    cells: Vec<Vec<bool>>
}

impl fmt::Display for LifeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "+---------------------+\n"));
        for row in self.cells.iter() {
            try!(write!(f, "| "));
            for cell in row.iter() {
                let c = if *cell { 'x' } else { ' ' };
                try!(write!(f, "{} ", c));
                
            }
            try!(write!(f, "|\n"));
        }
        write!(f, "+---------------------+\n")
    }
}

fn step(grid:&LifeGrid) {
    println!("{}", grid);
}

fn main() {
    
    let g = LifeGrid {cells: vec![vec![false;10]; 10]};
    step(&g);
    println!("Hello, world!");
}
