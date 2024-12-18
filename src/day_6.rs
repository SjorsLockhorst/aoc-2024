use std::{fmt, io::{self, Write}, thread, time::Duration, fs};

#[derive(Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Action {
    MovedToNew,
    MovedToVisited,
    Rotated,
    Ended,
}

struct GuardMap {
    map: Vec<Vec<char>>,
    guard_pos: (usize, usize),
    orientation: Orientation,
}

// Custom implementation of Debug for GuardMap
impl fmt::Debug for GuardMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Orientation: {:?}", self.orientation)?;
        writeln!(f, "Guard Position: {:?}", self.guard_pos)?;
        writeln!(f, "Map:")?;
        for row in &self.map {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl GuardMap {
    fn get_symbol_by_orientation(&mut self) -> char {
        match self.orientation {
            Orientation::Up => '^',
            Orientation::Down => 'v',
            Orientation::Left => '<',
            Orientation::Right => '>',
        }
    }

    fn next_step(&mut self) -> (usize, usize) {
        let (x, y) = self.guard_pos;

        match self.orientation {
            Orientation::Up => (x, y - 1),
            Orientation::Down => (x, y + 1),
            Orientation::Left => (x - 1, y),
            Orientation::Right => (x + 1, y),
        }
    }

    fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }

    fn draw_visited(&mut self) {
        let (guard_x, guard_y) = self.guard_pos;
        self.map[guard_y][guard_x] = 'X';
    }

    fn redraw_captain(&mut self, x: usize, y: usize) {
        self.map[y][x] = self.get_symbol_by_orientation();
    }

    fn move_captain(&mut self, x: usize, y: usize) {
        self.draw_visited();
        self.redraw_captain(x, y);
        self.guard_pos = (x, y);
    }

    fn simulate(&mut self) -> Action {
        let (guard_x, guard_y) = self.guard_pos;

        let (new_x, new_y) = self.next_step();

        match self.map[new_y][new_x] {
            '0' => {
                self.draw_visited();
                Action::Ended
            }
            '.' => {
                self.move_captain(new_x, new_y);
                Action::MovedToNew
            }
            '#' => {
                self.rotate();
                Action::Rotated
            }
            'X' => {
                self.move_captain(new_x, new_y);
                Action::MovedToVisited
            }
            _ => panic!("Unknown symbol was found on the map"),
        }
    }

    fn display(&self) {
        // Clear the terminal and move the cursor to the top-left
        print!("\x1B[2J\x1B[H");

        for row in &self.map {
            println!("{}", row.iter().collect::<String>());
        }
        io::stdout().flush().unwrap(); // Ensure the output is immediately flushed
    }
}

fn parse_map(contents: String) -> GuardMap {
    let mut map: Vec<Vec<char>> = Vec::new();

    let mut guard_x = usize::MAX;
    let mut guard_y = usize::MAX;

    for (i, line) in contents.lines().enumerate() {
        let mut row: Vec<char> = vec!['0'];
        for (j, char) in line.chars().enumerate() {
            row.push(char);
            if char == '^' {
                guard_x = i + 1;
                guard_y = j + 1;
            }
        }
        row.push('0');
        map.push(row);
    }

    let row_length = map[0].len();
    let padding_row = vec!['0'; row_length];
    map.insert(0, padding_row.clone());
    map.push(padding_row);

    if guard_x == usize::MAX || guard_y == usize::MAX {
        panic!("Couldn't find guard!");
    }

    GuardMap {
        map,
        guard_pos: (guard_y, guard_x),
        orientation: Orientation::Up,
    }
}

pub fn main() {
    let contents = fs::read_to_string("./inputs/day_6.txt").expect("Should be able to find file");

    let mut map = parse_map(contents);
    let mut total = 0;
    let mut step = 0;

    let mut next_action = map.simulate();

    loop {
        map.display(); // Render the map
        println!("Step: {}, Action: {:?}", step, next_action);

        match next_action {
            Action::Ended => {
                total += 1;
                break;
            }
            Action::MovedToNew  => {
                total += 1;
            }
            Action::MovedToVisited => {

            }
            Action::Rotated => {}
        }

        step += 1;
        next_action = map.simulate();
        // thread::sleep(Duration::from_millis(10)); // Add delay for animation effect
    }

    map.display();
    println!("Total places visited: {}", total);
}
