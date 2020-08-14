use std::collections::{HashMap};
use std::cell::RefCell;


#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum CellPaint
{
    Null,
    Open,
    Accessible,
    Closed // walls only
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Cell
{
    coord : (u32, u32),
    paint : CellPaint
}

impl Cell
{
    fn new(coord: (u32, u32)) -> Cell
    {
        Cell{coord, paint : CellPaint::Null}
    }
}

fn main()
{
    let w : u32 = 8;
    let h : u32 = 8;
    let loop_density = 20;
    let open_space_chance = 100;
    
    let virt_w : u32 = w*2-1;
    let virt_h : u32 = h*2-1;
    
    let cells = RefCell::new(HashMap::new());
    let cell_insert = |x, y|
    {
        let mut cells = cells.borrow_mut();
        let coord = (x, y);
        cells.insert(coord, Cell::new(coord))
    };
    
    for x in 0..virt_w
    {
        for y in 0..virt_h
        {
            cell_insert(x, y);
        }
    }
    let start_x = fastrand::u32(..w*2)/2*2;
    let start_y = fastrand::u32(..h*2)/2*2;
    
    cells.borrow_mut().get_mut(&(start_x, start_y)).unwrap().paint = CellPaint::Open;
    
    let repaint_walls = ||
    {
        let mut cells = cells.borrow_mut();
        // paint Null walls next to open cells as Accessible
        for x in 0..virt_w
        {
            for y in 0..virt_h
            {
                if x%2 == y%2 // skip non-walls
                {
                    continue;
                }
                // skip already-opened walls
                let self_paint = &cells.get(&(x, y)).unwrap().paint;
                if *self_paint == CellPaint::Open
                {
                    continue;
                }
                let a_paint;
                let b_paint;
                if x%2 == 1 // wall with a cell on the left and a cell on the right
                {
                    a_paint = &cells.get(&(x-1, y)).unwrap().paint;
                    b_paint = &cells.get(&(x+1, y)).unwrap().paint;
                }
                else
                {
                    a_paint = &cells.get(&(x, y-1)).unwrap().paint;
                    b_paint = &cells.get(&(x, y+1)).unwrap().paint;
                }
                
                match (a_paint, b_paint)
                {
                    (CellPaint::Open, CellPaint::Open) => cells.get_mut(&(x, y)).unwrap().paint = CellPaint::Closed,
                    (_, CellPaint::Open) | (CellPaint::Open, _) => cells.get_mut(&(x, y)).unwrap().paint = CellPaint::Accessible,
                    _ => {}
                };
            }
        }
    };
    let open_wall = ||
    {
        let mut cells = cells.borrow_mut();
        let mut open_walls = Vec::<(u32, u32)>::new();
        for (coord, cell) in cells.iter()
        {
            if cell.paint == CellPaint::Accessible
            {
                open_walls.push(*coord);
            }
        }
        if open_walls.is_empty()
        {
            return;
        }
        let coord = open_walls.get(fastrand::usize(..open_walls.len())).unwrap();
        cells.get_mut(&coord).unwrap().paint = CellPaint::Open;
        
        let (x, y) = *coord;
        if x%2 == 1
        {
            cells.get_mut(&(x-1, y)).unwrap().paint = CellPaint::Open;
            cells.get_mut(&(x+1, y)).unwrap().paint = CellPaint::Open;
        }
        else
        {
            cells.get_mut(&(x, y-1)).unwrap().paint = CellPaint::Open;
            cells.get_mut(&(x, y+1)).unwrap().paint = CellPaint::Open;
        }
    };
    let is_done = ||
    {
        let cells = cells.borrow();
        
        let mut truth = true;
        for x in 0..virt_w
        {
            for y in 0..virt_h
            {
                if x%2 == 1 && y%2 == 1
                {
                    continue;
                }
                let paint = cells.get(&(x, y)).unwrap().paint;
                if paint == CellPaint::Null || paint == CellPaint::Accessible
                {
                    truth = false;
                }
            }
        }
        truth
    };
    while !is_done()
    {
        repaint_walls();
        open_wall();
    }
    for y in 0..virt_h
    {
        for x in 0..virt_w
        {
            // randomly add in loops
            if x%2 != y%2 && fastrand::u32(..100) < loop_density
            {
                cells.borrow_mut().get_mut(&(x, y)).unwrap().paint = CellPaint::Open;
            }
        }
    }
    for y in 0..virt_h
    {
        for x in 0..virt_w
        {
            if x%2 == 1 && y%2 == 1
            {
                let a_paint = cells.borrow().get(&(x-1, y)).unwrap().paint;
                let b_paint = cells.borrow().get(&(x+1, y)).unwrap().paint;
                let c_paint = cells.borrow().get(&(x, y-1)).unwrap().paint;
                let d_paint = cells.borrow().get(&(x, y+1)).unwrap().paint;
                match (a_paint, b_paint, c_paint, d_paint)
                {
                    (CellPaint::Open, CellPaint::Open, CellPaint::Open, CellPaint::Open) =>
                    {
                        if fastrand::u32(..100) < open_space_chance
                        {
                            cells.borrow_mut().get_mut(&(x, y)).unwrap().paint = CellPaint::Open;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
    let random_cell = ||
    {
        (fastrand::u32(..w*2)/2*2, fastrand::u32(..w*2)/2*2)
    };
    let cell_dist = |a : (u32, u32), b : (u32, u32)|
    {
        ((a.0 as i32 - b.0 as i32).abs(), (a.1 as i32 - b.1 as i32).abs())
    };
    let entrance = random_cell();
    let mut exit = random_cell();
    let mut dist = cell_dist(entrance, exit);
    while (dist.0 as u32) < w/2 || (dist.1 as u32) < h/2
    {
        exit = random_cell();
        dist = cell_dist(entrance, exit);
    }
    
    for y in 0..virt_h
    {
        for x in 0..virt_w
        {
            let cell = cells.borrow().get(&(x, y)).unwrap().paint;
            if cell == CellPaint::Open
            {
                if (x, y) == entrance
                {
                    print!("█E");
                }
                else if (x, y) == exit
                {
                    print!("X█");
                }
                else
                {
                    print!("██");
                }
            }
            else
            {
                print!("  ");
            }
        }
        println!();
    }
}
