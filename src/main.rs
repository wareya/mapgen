use std::collections::{HashMap, HashSet};

const SLOWMOTION : bool = false;
const REALTIMEPRINT : bool = false;

const SUPERSLOW : u64 = 1000;
const KINDASLOW : u64 = 25;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
enum Cell
{
    Null,
    Open,
    Accessible,
    Closed,
    ForceClosed // walls only
}
struct Cells
{
    cells : HashMap<(u32, u32), Cell>,
    w : u32,
    h : u32
}

impl Cells
{
    fn new(w : u32, h : u32) -> Cells
    {
        let mut cells = HashMap::new();
        for x in 0..w
        {
            for y in 0..h
            {
                cells.insert((x, y), Cell::Null);
            }
        }
        Cells{cells, w, h}
    }
    fn get(&self, x : u32, y : u32) -> Cell
    {
        *self.cells.get(&(x, y)).unwrap()
    }
    fn contains(&self, x : u32, y : u32) -> bool
    {
        self.cells.contains_key(&(x, y))
    }
    fn set(&mut self, x : u32, y : u32, cell : Cell)
    {
        *self.cells.get_mut(&(x, y)).unwrap() = cell;
    }
    fn repaint_walls(&mut self)
    {
        // paint Null walls next to open cells as Accessible
        for x in 0..self.w
        {
            for y in 0..self.h
            {
                if x%2 == y%2 // skip non-walls
                {
                    continue;
                }
                // skip already-opened walls
                let self_paint = self.get(x, y);
                if self_paint == Cell::Open
                {
                    continue;
                }
                let a_paint;
                let b_paint;
                if x%2 == 1 // wall with a cell on the left and a cell on the right
                {
                    a_paint = self.get(x-1, y);
                    b_paint = self.get(x+1, y);
                }
                else
                {
                    a_paint = self.get(x, y-1);
                    b_paint = self.get(x, y+1);
                }
                
                match (a_paint, b_paint)
                {
                    (Cell::Open, Cell::Open) => self.set(x, y, Cell::Closed),
                    (Cell::Null, Cell::Open) | (Cell::Open, Cell::Null) => self.set(x, y, Cell::Accessible),
                    _ => {}
                };
            }
        }
    }
    fn open_wall(&mut self)
    {
        let mut open_walls = Vec::<(u32, u32)>::new();
        for x in 0..self.w
        {
            for y in 0..self.h
            {
                if self.get(x, y) == Cell::Accessible
                {
                    open_walls.push((x, y));
                }
            }
        }
        if open_walls.is_empty()
        {
            return;
        }
        let (x, y) = open_walls[fastrand::usize(..open_walls.len())];
        self.set(x, y, Cell::Open);
        if x%2 == 1
        {
            self.set(x-1, y, Cell::Open);
            self.set(x+1, y, Cell::Open);
        }
        else
        {
            self.set(x, y-1, Cell::Open);
            self.set(x, y+1, Cell::Open);
        }
    }
    fn is_done(&self, amount : u32 /* minimum cmopletion out of 100 */) -> bool
    {
        let mut filled = 0;
        let mut max_filled = 0;
        let mut accessible = 0;
        for x in 0..self.w
        {
            for y in 0..self.h
            {
                if x%2 == 1 && y%2 == 1
                {
                    continue;
                }
                let paint = self.get(x, y);
                
                if paint != Cell::ForceClosed
                {
                    max_filled += 1;
                }
                if paint == Cell::Open || paint == Cell::Closed 
                {
                    filled += 1;
                }
                if paint == Cell::Accessible
                {
                    accessible += 1;
                }
            }
        }
        if accessible == 0
        {
            println!("accessible is 0");
        }
        return filled*100/max_filled >= amount || accessible == 0;
    }
    fn print(&self, entrance : (u32, u32), exit : (u32, u32), expander_w : u32, expander_h : u32)
    {
        let mut output = "".to_string();
        for y in 0..self.h
        {
            for i in 0..if y%2 == 0 { expander_h } else { 1 }
            {
                output += &"    ";
                for x in 0..self.w
                {
                    let cell_paint = self.get(x, y);
                    for j in 0..if x%2 == 0 { expander_w } else { 1 }
                    {
                        if cell_paint == Cell::Open
                        {
                            if (x, y) == entrance && entrance == exit && i == expander_h/2 && j == expander_w/2
                            {
                                output += &"EX";
                            }
                            else if (x, y) == entrance && i == expander_h/2 && j == expander_w/2
                            {
                                output += &"EE";
                            }
                            else if (x, y) == exit && i == expander_h/2 && j == expander_w/2
                            {
                                output += &"XX";
                            }
                            else if x%2 == y%2 || i == expander_h/2 || j == expander_w/2
                            {
                                output += &"██";
                            }
                            else if x%2 == 1
                            {
                                if y > 0 && self.get(x, y-1) == Cell::Open
                                || y+1 < self.h && self.get(x, y+1) == Cell::Open
                                {
                                    output += &"██";
                                }
                                else
                                {
                                    output += &"  ";
                                }
                            }
                            else if y%2 == 1
                            {
                                if x > 0 && self.get(x-1, y) == Cell::Open
                                || x+1 < self.w && self.get(x+1, y) == Cell::Open
                                {
                                    output += &"██";
                                }
                                else
                                {
                                    output += &"  ";
                                }
                            }
                            else
                            {
                                output += &"  ";
                            }
                        }
                        else if cell_paint == Cell::Accessible
                        {
                            output += &"░░";
                        }
                        else if cell_paint == Cell::Null
                        {
                            output += &"..";
                        }
                        else
                        {
                            output += &"  ";
                        }
                    }
                }
                output += &"\n";
            }
        }
        output += &"\n";
        print!("{}", output);
        if SLOWMOTION && REALTIMEPRINT
        {
            std::thread::sleep(std::time::Duration::from_millis(KINDASLOW));
        }
    }
}

fn generate_map()
{
    #[allow(unused_mut)]
    #[allow(unused_assignments)]
    let mut seed = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()/100) as u64;
    
    println!("using seed {}", seed);
    if SLOWMOTION && REALTIMEPRINT
    {
        std::thread::sleep(std::time::Duration::from_millis(SUPERSLOW));
    }
    fastrand::seed(seed);
    
    let w : u32 = 16;
    let h : u32 = 16;
    
    let loop_density = 20;
    let loop_deletion_chance = 75;
    let max_island_cull = 35;
    let chance_preclosed = 20;
    let completion_amount_min = 80;
    let completion_amount_max = 100;
    let tiny_dead_end_deletion_rate = 80;
    let blockage_radius : i32 = 4;
    
    let expander_w = 1;
    let expander_h = 1;
    
    let completion_amount =
    if completion_amount_max == completion_amount_min
    {
        completion_amount_min
    }
    else
    {
        completion_amount_min + fastrand::u32(..(completion_amount_max-completion_amount_min))
    };
    
    
    let virt_w : u32 = w*2-1;
    let virt_h : u32 = h*2-1;
    
    let mut cells = Cells::new(virt_w, virt_h);
    
    for y in 0..h
    {
        for x in 0..w
        {
            if fastrand::u32(..100) < chance_preclosed
            {
                cells.set(x*2, y*2, Cell::ForceClosed);
                if x > 0 && cells.contains(x*2-1, y*2)
                {
                    cells.set(x*2-1, y*2, Cell::ForceClosed);
                }
                if cells.contains(x*2+1, y*2)
                {
                    cells.set(x*2+1, y*2, Cell::ForceClosed);
                }
                if y > 0 && cells.contains(x*2, y*2-1)
                {
                    cells.set(x*2, y*2-1, Cell::ForceClosed);
                }
                if cells.contains(x*2, y*2+1)
                {
                    cells.set(x*2, y*2+1, Cell::ForceClosed);
                }
                println!("set {},{} to closed", x*2, y*2);
            }
        }
    }
    
    if REALTIMEPRINT
    {
        cells.print((!0, !0), (!0, !0), expander_w, expander_h);
    }
    
    let start_x = fastrand::u32(..w)*2;
    let start_y = fastrand::u32(..h)*2;
    
    for x in std::cmp::max(0, start_x as i32 - blockage_radius) as u32..=std::cmp::min(virt_w-1, start_x + blockage_radius as u32)
    {
        for y in std::cmp::max(0, start_y as i32 - blockage_radius) as u32..=std::cmp::min(virt_h-1, start_y + blockage_radius as u32)
        {
            cells.set(x, y, Cell::Null);
        }
    }
    cells.set(start_x, start_y, Cell::Open);
    
    println!("set {},{} to open", start_x, start_y);
    cells.repaint_walls();
    
    println!("generating layout");
    
    if REALTIMEPRINT
    {
        cells.print((!0, !0), (!0, !0), expander_w, expander_h);
        if SLOWMOTION
        {
            std::thread::sleep(std::time::Duration::from_millis(SUPERSLOW));
        }
    }
    
    while !cells.is_done(completion_amount)
    {
        cells.open_wall();
        cells.repaint_walls();
        if REALTIMEPRINT
        {
            cells.print((!0, !0), (!0, !0), expander_w, expander_h);
        }
    }
    
    println!("placing entrance/exit");
    
    if SLOWMOTION && REALTIMEPRINT
    {
        std::thread::sleep(std::time::Duration::from_millis(SUPERSLOW));
    }
    
    let random_open_cell = ||
    {
        let mut cell = (fastrand::u32(..virt_w)/2*2, fastrand::u32(..virt_h)/2*2);
        while cells.get(cell.0, cell.1) != Cell::Open
        {
            cell = (fastrand::u32(..virt_w)/2*2, fastrand::u32(..virt_h)/2*2);
        }
        cell
    };
    let cell_dist = |a : (u32, u32), b : (u32, u32)|
    {
        (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
    };
    let entrance = random_open_cell();
    let mut exit = random_open_cell();
    let mut dist = cell_dist(entrance, exit);
    let mut min_dist = std::cmp::max(w, h)/2;
    while (dist as u32) < min_dist
    {
        exit = random_open_cell();
        dist = cell_dist(entrance, exit);
        if min_dist > 0
        {
            min_dist -= 1;
        }
    }
    if entrance == exit
    {
        println!("failed...");
    }
    
    if REALTIMEPRINT
    {
        cells.print(entrance, exit, expander_w, expander_h);
    }
    
    println!("placing loops");
    
    if REALTIMEPRINT && SLOWMOTION
    {
        std::thread::sleep(std::time::Duration::from_millis(SUPERSLOW));
    }
    
    for y in 0..virt_h
    {
        for x in 0..virt_w
        {
            // randomly add in loops
            if x%2 != y%2 && cells.get(x, y) == Cell::Closed && fastrand::u32(..100) < loop_density
            {
                cells.set(x, y, Cell::Open);
                if x%2 == 1
                {
                    cells.set(x-1, y, Cell::Open);
                    cells.set(x+1, y, Cell::Open);
                }
                if y%2 == 1
                {
                    cells.set(x, y-1, Cell::Open);
                    cells.set(x, y+1, Cell::Open);
                }
                if REALTIMEPRINT
                {
                    cells.print(entrance, exit, expander_w, expander_h);
                }
            }
        }
    }
    
    if REALTIMEPRINT
    {
        cells.print(entrance, exit, expander_w, expander_h);
    }
    
    println!("deleting islands");
    
    if REALTIMEPRINT && SLOWMOTION
    {
        std::thread::sleep(std::time::Duration::from_millis(SUPERSLOW));
    }
    
    
    let mut edge_walls = HashSet::new();
    for y in 0..virt_h
    {
        if cells.get(0, y) != Cell::Open
        {
            edge_walls.insert((0, y));
        }
        if cells.get(virt_w-1, y) != Cell::Open
        {
            edge_walls.insert((virt_w-1, y));
        }
    }
    for x in 0..virt_w
    {
        if cells.get(x, 0) != Cell::Open
        {
            edge_walls.insert((x, 0));
        }
        if cells.get(x, virt_h-1) != Cell::Open
        {
            edge_walls.insert((x, virt_h-1));
        }
    }
    
    let flood_fill_collection = |cells : &Cells, x : u32, y : u32| -> Vec<(u32, u32)>
    {
        let mut visited = HashSet::<(u32, u32)>::new();
        let mut frontier = HashSet::<(u32, u32)>::new();
        let visit = |frontier : &mut HashSet<_>, visited : &mut HashSet<_>, x, y|
        {
            let add_to_frontier = |frontier : &mut HashSet<_>, visited : &mut HashSet<_>, x, y|
            {
                if !cells.contains(x, y)
                {
                    return;
                }
                if cells.get(x, y) != Cell::Open && !visited.contains(&(x, y)) && !frontier.contains(&(x, y))
                {
                    frontier.insert((x, y));
                }
            };
            frontier.remove(&(x, y));
            visited.insert((x, y));
            if x > 0
            {
                add_to_frontier(frontier, visited, x-1, y);
            }
            add_to_frontier(frontier, visited, x+1, y);
            if y > 0
            {
                add_to_frontier(frontier, visited, x, y-1);
            }
            add_to_frontier(frontier, visited, x, y+1);
        };
        
        visit(&mut frontier, &mut visited, x, y);
        while frontier.len() > 0
        {
            let (x, y) = *frontier.iter().next().unwrap();
            visit(&mut frontier, &mut visited, x, y);
        }
        
        visited.into_iter().collect()
    };
    
    let mut excluded_walls = HashSet::new();
    for (x, y) in edge_walls
    {
        for wall in flood_fill_collection(&cells, x, y)
        {
            excluded_walls.insert(wall);
        }
    }
    
    let mut islands = Vec::new();
    
    for y in 0..virt_h
    {
        for x in 0..virt_w
        {
            if cells.get(x, y) != Cell::Open && !excluded_walls.contains(&(x, y))
            {
                let island = flood_fill_collection(&cells, x, y);
                islands.push(island.clone());
                for wall in island
                {
                    excluded_walls.insert(wall);
                }
            }
        }
    }
    
    for island in islands.iter()
    {
        if island.len() <= max_island_cull && fastrand::u32(..100) < loop_deletion_chance
        {
            for (x, y) in island.iter()
            {
                cells.set(*x, *y, Cell::Open);
            }
            if REALTIMEPRINT
            {
                cells.print(entrance, exit, expander_w, expander_h);
            }
        }
    }
    
    println!("deleting tiny dead ends");
    
    if REALTIMEPRINT && SLOWMOTION
    {
        std::thread::sleep(std::time::Duration::from_millis(SUPERSLOW));
    }
    
    
    for y in 0..h
    {
        let y = y*2;
        for x in 0..w
        {
            let x = x*2;
            if entrance == (x, y) || exit == (x, y)
            {
                continue;
            }
            if cells.get(x, y) == Cell::Open
            {
                let mut num_open_sides = 0;
                if x > 0 && cells.get(x-1, y) == Cell::Open
                {
                    num_open_sides += 1;
                }
                if x < virt_w-1 && cells.get(x+1, y) == Cell::Open
                {
                    num_open_sides += 1;
                }
                if y > 0 && cells.get(x, y-1) == Cell::Open
                {
                    num_open_sides += 1;
                }
                if y < virt_h-1 && cells.get(x, y+1) == Cell::Open
                {
                    num_open_sides += 1;
                }
                if num_open_sides == 1
                {
                    let mut open_paths = 0;
                    if fastrand::u32(..100) >= tiny_dead_end_deletion_rate
                    {
                        continue;
                    }
                    if x > 0 && cells.get(x-1, y) == Cell::Open
                    {
                        if y > 0 && cells.get(x-2, y-1) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if x > 2 && cells.get(x-3, y) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if y < virt_h-1 && cells.get(x-2, y+1) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if open_paths > 1
                        {
                            cells.set(x, y, Cell::Closed);
                            cells.set(x-1, y, Cell::Closed);
                            if REALTIMEPRINT
                            {
                                cells.print(entrance, exit, expander_w, expander_h);
                            }
                        }
                    }
                    else if x < virt_w-1 && cells.get(x+1, y) == Cell::Open
                    {
                        if y > 0 && cells.get(x+2, y-1) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if x < virt_w-3 && cells.get(x+3, y) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if y < virt_h-1 && cells.get(x+2, y+1) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if open_paths > 1
                        {
                            cells.set(x, y, Cell::Closed);
                            cells.set(x+1, y, Cell::Closed);
                            if REALTIMEPRINT
                            {
                                cells.print(entrance, exit, expander_w, expander_h);
                            }
                        }
                    }
                    else if y > 0 && cells.get(x, y-1) == Cell::Open
                    {
                        if x > 0 && cells.get(x-1, y-2) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if y > 2 && cells.get(x, y-3) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if x < virt_w-1 && cells.get(x+1, y-2) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if open_paths > 1
                        {
                            cells.set(x, y, Cell::Closed);
                            cells.set(x, y-1, Cell::Closed);
                            if REALTIMEPRINT
                            {
                                cells.print(entrance, exit, expander_w, expander_h);
                            }
                        }
                    }
                    else if y < virt_w-1 && cells.get(x, y+1) == Cell::Open
                    {
                        if x > 0 && cells.get(x-1, y+2) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if y < virt_h-3 && cells.get(x, y+3) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if x < virt_w-1 && cells.get(x+1, y+2) == Cell::Open
                        {
                            open_paths += 1;
                        }
                        if open_paths > 1
                        {
                            cells.set(x, y, Cell::Closed);
                            cells.set(x, y+1, Cell::Closed);
                            if REALTIMEPRINT
                            {
                                cells.print(entrance, exit, expander_w, expander_h);
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("done");
    
    cells.print(entrance, exit, expander_w, expander_h);
}

fn main()
{
    generate_map();
    println!();
}
