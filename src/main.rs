use std::{env, fmt};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Color {
    R,
    O,
    Y,
    G,
    B,
    P,
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

type Path = Vec<Point>;
type Maze = Vec<Vec<Color>>;
type Sequence = Vec<Color>;

//freq returns how many instances of n are in p, in a simple imperative style
fn freq(n: &Point, p: &Path) -> usize {
    let mut ret = 0;
    for point in p {
        if point == n {
            ret += 1;
        }
    }
    ret
}

//init reads the file given returns the search sequence and the maze
//discards junk in the input file
fn init() -> (Maze, Sequence) {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).expect("not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "could not read file",
    );

    let mut seq: Sequence = Vec::new();

    //grab line iterator
    let mut lines = contents.lines();

    //read the first line into a Sequence
    for c in lines.next().expect("empty input").chars() {
        match c {
            'R' => seq.push(Color::R),
            'O' => seq.push(Color::O),
            'Y' => seq.push(Color::Y),
            'G' => seq.push(Color::G),
            'B' => seq.push(Color::B),
            'P' => seq.push(Color::P),
            _ => continue,
        }
    }

    //read the rest into a Maze
    let mut maze: Maze = Vec::new();
    for (x, l) in lines.enumerate() {
        //create Color vector
        maze.push(Vec::new());
        for c in l.chars() {
            match c {
                'R' => maze[x].push(Color::R),
                'O' => maze[x].push(Color::O),
                'Y' => maze[x].push(Color::Y),
                'G' => maze[x].push(Color::G),
                'B' => maze[x].push(Color::B),
                'P' => maze[x].push(Color::P),
                _ => continue,
            }
        }
    }
    (maze, seq)
}

//neighbors returns the points orthogonal to point p
//assumes m is square
fn neighbors(p: &Point, m: &Maze) -> Vec<Point> {
    let mut ret = Vec::new();
    if p.x > 0 {
        ret.push(Point { x: p.x - 1, y: p.y });
    }
    if p.x < m.len() - 1 {
        ret.push(Point { x: p.x + 1, y: p.y });
    }
    if p.y > 0 {
        ret.push(Point { x: p.x, y: p.y - 1 });
    }
    if p.y < m.len() - 1 {
        ret.push(Point { x: p.x, y: p.y + 1 });
    }
    ret
}

//nth_in_sequence is sort of a `repeat`
//it returns what would be the nth val if the sequence repeated infinitely - 'fake lazy'
fn nth_in_sequence(s: &Sequence, n: usize) -> Color {
    s[n % s.len()].clone()
}

//find_path recursively steps through neighbors
//it completes when either the top row is found or there are no remaining moves
fn find_path(p: &mut Path, m: &Maze, s: &Sequence) -> Option<Path> {
    //grab neighbors around latest point in path
    let neighbors = neighbors(&p[p.len() - 1], m);
    let mut matches = Vec::new();
    for n in &neighbors {
        if m[n.y][n.x] == nth_in_sequence(s, p.len()) {
            if freq(n, p) >= 2 {
                continue;
            }
            //if we're at the top row, return the full path
            if n.y == 0 {
                p.push(*n);
                return Some(p.to_vec());
            }
            //otherwise push to matches
            matches.push(n);
        }
        //recur for every match
        for point in &matches {
            p.push(*(*point));
            match find_path(p, m, s) {
                None => continue,
                Some(result) => return Some(result),
            }
        }
    }
    //if none fit, return None, there is no path
    None
}

//solve returns a path from the bottom row to to the top, or None
fn solve(m: &Maze, s: &Sequence) -> Vec<Path> {
    let mut paths = Vec::new();
    //try each point in the bottom row
    for (x, _) in m[m.len() - 1].iter().enumerate() {
        //create path vector
        let mut path: Path = vec![
            Point {
                x: x,
                y: m.len() - 1,
            },
        ];
        match find_path(&mut path, m, s) {
            None => continue,
            Some(result) => paths.push(result),
        }
    }
    paths
}

fn member(c: &Point, p: &Path) -> bool {
    for n in p {
        if n == c {
            return true;
        }
    }
    false
}

//pretty_print takes a maze and a path, and outputs the maze with only the path filled in
fn pretty_print(m: &Maze, p: &Path) {
    for (x, l) in m.iter().enumerate() {
        for (y, _) in l.iter().enumerate() {
            if member(&Point { x: y, y: x}, p) {
                print!("{:?}", m[x][y]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let (maze, seq) = init();
    let paths = solve(&maze, &seq);
    pretty_print(&maze, &paths[0]);
}
