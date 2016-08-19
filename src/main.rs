use std::boxed::Box;
use std::fmt;
use std::io;
use std::ops::{Index, IndexMut};

extern crate rand;

/// A structure containing the state of the Life game.
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

    /// Places the given pattern into the grid at the given point,
    /// with 0,0 being the top-left.
    fn put_pattern(&mut self, x:i32, y:i32, pattern: Vec<Vec<bool>>) {
        for yn in 0..(pattern.len()) {
            for xn in 0..(pattern[yn].len()) {
                let nx = x + (xn as i32);
                let ny = y + (yn as i32);
                self[(ny, nx)] = pattern[yn][xn];
            }
        }
    }

    /// Places a glider in the specified part of the grid,
    /// coordinates starting from the top-left.
    fn glider(&mut self, x:i32, y:i32) {
        //  x
        //   x
        // xxx
        /*
        self[(x,y+1)] = true;
        self[(x+1,y+2)] = true;
        self[(x+2,y)] = true;
        self[(x+2,y+1)] = true;
        self[(x+2,y+2)] = true;
         */
        let g = vec![vec![false, true,  false],
                     vec![false, false, true ],
                     vec![true , true,  true ]
        ];
        self.put_pattern(x, y, g);
    }

    /// "Lightweight spaceship"
    fn lwss(&mut self, x:i32, y:i32) {
        //  xxxx
        // x   x
        //     x
        // x  x 
        let ss = vec![vec![false, true,  true,  true,  true ],
                      vec![true,  false, false, false, true ],
                      vec![false, false, false, false, true ],
                      vec![true,  false, false, true,  false]
        ];
        self.put_pattern(x, y, ss);
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
        let s = self.sizei();
        // Annoyingly, Rust conforms with C and actually uses remainder
        // instead of modulus.  This is kind of dumb, but oh well.
        // But cases where n < -size might theoretically happen, which could
        // still be a bit problematic... gotta check that.
        // This idiom appears to get us the actual modulus, but surprisingly
        // is only a *little* faster than the while-loop solution below.
        // And built in release mode, the while loop is *much* faster
        // presumably because each loop at most does one increment, and
        // usually nothing, while the math happens all the time.
        //let xwrap = ((x % s) + s) % s;
        //let ywrap = ((y % s) + s) % s;
        //&self.cells[xwrap as usize][ywrap as usize]
        
        while x < 0 {
            x += s;
        }
        while y < 0 {
            y += s;
        }
        while x >= s {
            x -= s;
        }
        while y >= s {
            y -= s;
        }
         
        // x and y should now be positive integers in [0,self.size())
        //println!("x {} y {}", xwrap, ywrap);
        &self.cells[x as usize][y as usize]

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
    let cells = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for (xoff,yoff) in cells {
        if grid[(x+xoff,y+yoff)] {
            accm += 1;
        }
    }
*/
    

    // This feels very crude, but, oh well.
    // Keeping the loop unrolled seems to actually be a minor performance win.

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

fn main_interactive() {
    //let mut g = Box::new(LifeGrid::random(10));
    let mut g = Box::new(LifeGrid::new(20));
    g.lwss(5,5);
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

fn main() {
    /*
    let mut g = Box::new(LifeGrid::new(10));
    g.lwss(0,0);
    for _ in 0..10 {
        println!("{}", g);
        g = step(&g);
    }
     */
    main_interactive()
}
