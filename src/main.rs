use std::boxed::Box;
use std::fmt;
use std::io;
use std::ops::{Index, IndexMut};

extern crate rand;

struct LifeGrid {
    cells: Vec<Vec<bool>>,
}

impl LifeGrid {
    // We'll just say the grid is square by definition.
    fn new(size:usize) -> LifeGrid {
        LifeGrid {
            cells: vec![vec![false; size]; size]
        }
    }

    fn random(size:usize) -> LifeGrid {
        let mut grid = LifeGrid::new(size);
        for x in 0..(size as i32) {
            for y in 0..(size as i32) {
                grid[(x,y)] = rand::random();
            }
        }
        grid
    }

    fn glider(&mut self, x:i32, y:i32) {
        //  x
        //   x
        // xxx
        self[(x,y+1)] = true;
        self[(x+1,y+2)] = true;
        self[(x+2,y)] = true;
        self[(x+2,y+1)] = true;
        self[(x+2,y+2)] = true;
    }

    fn size(&self) -> usize {
        self.cells.len()
    }

    fn sizei(&self) -> i32 {
        self.size() as i32
    }
}

fn write_dashes(n:usize, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "+"));
    for _ in 0..n {
        try!(write!(f, "-"));
    }
    write!(f, "+\n")

}

impl Index<(i32, i32)> for LifeGrid {
    type Output = bool;
    fn index(&self, _index: (i32,i32)) -> &bool {
        let (mut x, mut y) = _index;
        while x < 0 {
            x += self.sizei();
        }
        while y < 0 {
            y += self.sizei();
        }
        while x >= self.sizei() {
            x -= self.sizei();
        }
        while y >= self.sizei() {
            y -= self.sizei()
        }
        // x and y should now be positive integers in [0,self.size())
        &self.cells[x as usize][y as usize]

        /*
        let (x,y) = _index;
        // We'll just wrap the grid.
        let xwrap = (x % self.sizei()) as usize;
        let ywrap = (y % self.sizei()) as usize;
        //println!("Indexing at {},{}", xwrap, ywrap);
        &self.cells[xwrap][ywrap]
        */
    }
}

impl IndexMut<(i32, i32)> for LifeGrid {
    fn index_mut(&mut self, _index: (i32,i32)) -> &mut bool {
        let (x,y) = _index;
        // We'll just wrap the grid.
        let xwrap = (x % self.sizei()) as usize;
        let ywrap = (y % self.sizei()) as usize;
        //println!("Indexing at {},{}", xwrap, ywrap);
        &mut self.cells[xwrap][ywrap]
    }
}


impl fmt::Display for LifeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write_dashes(self.cells.len(), f));
        for row in self.cells.iter() {
            try!(write!(f, "|"));
            for cell in row.iter() {
                let c = if *cell { 'x' } else { ' ' };
                try!(write!(f, "{}", c));
                
            }
            try!(write!(f, "|\n"));
        }

        write_dashes(self.cells.len(), f)
    }
}

fn neighbors_cell(grid:&LifeGrid, x: i32, y: i32) -> i32 {
    let mut accm = 0;
    /*
    if x-1 > 0 {
        // Check to the left
        if grid[(x-1,y)] {
            accm += 1;
        }
    }
    if y-1 > 0 {
        // Check above
    }
    if x+1 < grid.sizei() {
        // Check to the right
    }
    if y+1 < grid.sizei() {
        // Check below
    }
     */
    if grid[(x-1,y-1)] {
        accm += 1;
    }
    if grid[(x-1,y)] {
        accm += 1;
    }
    if grid[(x-1,y+1)] {
        accm += 1;
    }

    if grid[(x,y-1)] {
        accm += 1;
    }
    if grid[(x,y+1)] {
        accm += 1;
    }

    if grid[(x+1,y-1)] {
        accm += 1;
    }
    if grid[(x+1,y)] {
        accm += 1;
    }
    if grid[(x+1,y+1)] {
        accm += 1;
    }
    
    accm
}



fn step_cell(oldgrid:&LifeGrid, newgrid:&mut LifeGrid, x: i32, y: i32) {
    let neighbors = neighbors_cell(oldgrid, x, y);
    let cell_is_alive = oldgrid[(x,y)];
    if cell_is_alive {
        if neighbors < 2 {
            newgrid[(x,y)] = false;
        } else if neighbors > 3 {
            newgrid[(x,y)] = false;

        } else {
            newgrid[(x,y)] = true;
        }
    } else {
        if neighbors == 3 {
            newgrid[(x,y)] = true;
        } else {
            newgrid[(x,y)] = false;
        }
    }
}

fn step(grid:&LifeGrid) -> Box<LifeGrid> {
    let maxsize = grid.sizei();
    assert!(maxsize > 0);

    let mut newgrid = LifeGrid::new(maxsize as usize);
    for x in 0..maxsize {
        for y in 0..maxsize {
            step_cell(grid, &mut newgrid, x, y);
        }
    }
    Box::new(newgrid)
}

fn main() {
    let mut g = Box::new(LifeGrid::random(10));
    let mut g = Box::new(LifeGrid::new(10));
    g.glider(0,0);
    loop {
        println!("{}", g);
        println!("Hit enter to step, ctrl-d to exit");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                //println!("You entered {} characters: '{}'", n, input);
                g = step(&g);
                if n == 0 {
                    break;
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                break
            }
        }
    }
}
