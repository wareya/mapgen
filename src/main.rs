use std::collections::{HashMap};
use std::cell::RefCell;


#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum CellKind
{
    Cell,
    Wall,
}
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

#[derive(Clone)]
#[derive(Debug)]
struct Cell
{
    coord : (u32, u32),
    kind : CellKind,
    paint : CellPaint
}

impl Cell
{
    fn new(coord: (u32, u32), kind : CellKind) -> Cell
    {
        Cell{coord, kind, paint : CellPaint::Null}
    }
}

fn main()
{
    let w : u32 = 8;
    let h : u32 = 8;
    
    let virt_w : u32 = w*2-1;
    let virt_h : u32 = h*2-1;
    
    let mut cells = HashMap::<(u32, u32), Cell>::new();
    let mut cell_insert = |x, y|
    {
        let coord = (x, y);
        match (x%2, y%2)
        {
            (0, 0) => cells.insert(coord, Cell::new(coord, CellKind::Cell)),
            (1, 1) => None,
            _ => cells.insert(coord, Cell::new(coord, CellKind::Wall))
        }
    };
    
    for x in 0..virt_w
    {
        for y in 0..virt_h
        {
            cell_insert(x, y);
        }
    }
    let start_x = fastrand::usize(..w as usize) as u32 * 2;
    let start_y = fastrand::usize(..h as usize) as u32 * 2;
    
    cells.get_mut(&(start_x, start_y)).unwrap().paint = CellPaint::Open;
    
    let cells = RefCell::new(cells);
    
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
            if cell.kind == CellKind::Wall && cell.paint == CellPaint::Accessible
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
    let cells = cells.into_inner();
    for x in 0..virt_w
    {
        for y in 0..virt_h
        {
            if x%2 == 1 && y%2 == 1
            {
                print!(" ");
                continue;
            }
            let cell = cells.get(&(x, y)).unwrap();
            if cell.paint == CellPaint::Open
            {
                print!("#");
            }
            else
            {
                print!(" ");
            }
        }
        println!();
    }
}
