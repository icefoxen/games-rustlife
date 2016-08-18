use std::fmt;

struct LifeGrid {
    cells: Vec<Vec<bool>>,
}

impl LifeGrid {
    fn new(size:usize) -> LifeGrid {
        LifeGrid {
            cells: vec![vec![false; size]; size]
        }
    }
}

fn write_dashes(n:usize, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "+"));
    for _ in 0..n {
        try!(write!(f, "-"));
    }
    write!(f, "+\n")

}


impl fmt::Display for LifeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_dashes(self.cells.len(), f);
        for row in self.cells.iter() {
            try!(write!(f, "|"));
            for cell in row.iter() {
                let c = if *cell { 'x' } else { 'o' };
                try!(write!(f, "{}", c));
                
            }
            try!(write!(f, "|\n"));
        }

        write_dashes(self.cells.len(), f)
    }
}

fn step(grid:&LifeGrid) {
    println!("{}", grid);
}

fn main() {
    
    let g = LifeGrid::new(10);
    step(&g);
    println!("Hello, world!");
}
