use aoc_utils::DayInfo;
use aoc_utils::DaySolver;

use anyhow::anyhow;
use either::Either;
use fnv::FnvHashMap as HashMap;
use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;
use text_io::scan;

use super::ex20::{
    calculate_borders_change, get_next_rotation, get_next_stage, number_from_line, roatate_binary,
    RotationState, TileStage,
};

pub struct Day20b;

impl DaySolver for Day20b {
    type Output = u128;

    const INFO: DayInfo =
        DayInfo::with_day_and_file_and_variant("day_20b", "data_files/ex20.txt", "faster");

    fn solution(_s: &str) -> anyhow::Result<<Self>::Output> {
        let body: Vec<Tile> = _s
            .split("\n\n")
            .map(|tile| tile.parse::<Tile>().unwrap())
            .collect();

        let mut domain_generator = DomainGenerator::new();
        domain_generator.fill(&body);

        let grid = Grid { body: Vec::new() };

        match bt_iter_search(grid, &domain_generator.domains) {
            Either::Left(body) => {
                let res = &body[0];
                Ok(res[8].id * res[7].id * res[6].id * res[5].id)
            },

            Either::Right(_) => Err(anyhow!("Unable to solve")),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Tile {
    id: u128,
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let res = _s.lines().map(|line| line.trim()).enumerate().fold(
            Tile::new(),
            |mut tile, (line_number, line)| match line_number {
                0 => {
                    scan!(line.bytes() => "Tile {}:", tile.id);
                    tile
                },
                1 => {
                    tile.top = number_from_line(line);

                    let (left, right) = calculate_borders_change(line, line_number);
                    tile.left += left;
                    tile.right += right;

                    tile
                },
                10 => {
                    tile.bottom = number_from_line(line);

                    let (left, right) = calculate_borders_change(line, line_number);
                    tile.left += left;
                    tile.right += right;

                    tile
                },
                _ => {
                    let (left, right) = calculate_borders_change(line, line_number);
                    tile.left += left;
                    tile.right += right;
                    tile
                },
            },
        );

        Ok(res)
    }
}

impl Tile {
    fn new() -> Self {
        Self {
            id: 0,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        }
    }

    fn boarder_values(&self) -> Vec<u16> {
        vec![self.top, self.bottom, self.left, self.right]
    }

    fn rotate(&mut self, rotation_state: &RotationState) -> RotationState {
        let top_new = roatate_binary(self.right);
        let bottom_new = roatate_binary(self.left);
        let left_new = self.top;
        let right_new = self.bottom;

        self.top = top_new;
        self.bottom = bottom_new;
        self.left = left_new;
        self.right = right_new;

        match get_next_rotation(rotation_state) {
            Some(rotation) => rotation,

            None => RotationState::R0,
        }
    }

    fn flip_horizontal(&mut self) {
        (self.left, self.right) = (self.right, self.left);
        self.top = roatate_binary(self.top);
        self.bottom = roatate_binary(self.bottom);
    }

    fn flip_vertical(&mut self) {
        (self.top, self.bottom) = (self.bottom, self.top);
        self.right = roatate_binary(self.right);
        self.left = roatate_binary(self.left);
    }
}

struct TileIterator {
    tile: Tile,
    stage: TileStage,
    rotation_state: RotationState,
    has_next: bool,
}

impl TileIterator {
    fn new(tile: Tile) -> Self {
        Self {
            tile,
            stage: TileStage::Base,
            rotation_state: RotationState::R0,
            has_next: true,
        }
    }
}

impl Iterator for TileIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.has_next {
            return None;
        }

        let res = self.tile;

        self.rotation_state = self.tile.rotate(&self.rotation_state);

        if self.rotation_state == RotationState::R0 {
            match get_next_stage(&self.stage) {
                None => {
                    self.has_next = false;
                },

                Some(next_stage) => {
                    if next_stage == TileStage::HorizontallyFlipped {
                        self.tile.flip_horizontal();
                    } else if next_stage == TileStage::VerticallyFlipped {
                        self.tile.flip_horizontal();
                        self.tile.flip_vertical();
                    }

                    self.stage = next_stage;
                },
            }
        }

        Some(res)
    }
}

struct DomainGenerator {
    connections: HashMap<Tile, Vec<Tile>>,
    domains: Vec<Vec<Tile>>,
}

impl DomainGenerator {
    fn add(&mut self, checked: &Tile, new_neighbour: &Tile) {
        let checked_id = checked.id;
        let new_neighbour_id = new_neighbour.id;

        if checked_id != new_neighbour_id {
            for i in TileIterator::new(*checked) {
                if i.boarder_values().into_iter().any(|value_i| {
                    new_neighbour
                        .boarder_values()
                        .into_iter()
                        .any(|value_j| value_i == value_j)
                }) {
                    match self.connections.get_mut(checked) {
                        None => {
                            self.connections.insert(*checked, vec![*new_neighbour]);
                        },
                        Some(v) => {
                            v.push(*new_neighbour);
                        },
                    };

                    return;
                }
            }
        }
    }

    fn generate_domains(&mut self) {
        for key in self.connections.keys() {
            let conections = self.connections.get(key).unwrap().len();
            if conections == 4 {
                for index in 0..9 {
                    self.domains[index].push(*key);
                }
            } else if conections == 3 {
                for index in (0..9).filter(|index| *index != 4) {
                    self.domains[index].push(*key);
                }
            } else if conections == 2 {
                self.domains[0].push(*key);
                self.domains[2].push(*key);
                self.domains[6].push(*key);
                self.domains[8].push(*key);
            }
        }
    }

    fn new() -> Self {
        let connections: HashMap<Tile, Vec<Tile>> = HashMap::default();
        let domains = vec![Vec::new(); 9];

        Self {
            connections,
            domains,
        }
    }

    fn fill(&mut self, grid: &Vec<Tile>) {
        for i in grid {
            for j in grid {
                self.add(i, j);
            }
        }

        self.generate_domains();
    }
}

#[derive(Debug)]
struct Grid {
    body: Vec<Tile>,
}

/*
    0 1 2
    3 4 5
    6 7 8
*/

impl Grid {
    fn tile_placed(&self, tile_id: u128) -> bool {
        self.body.iter().any(|tile| tile.id == tile_id)
    }
}

impl BTState for Grid {
    type Choice = Tile;
    type Domain = Vec<Vec<Tile>>;

    fn is_goal(&mut self) -> bool {
        if self.body.len() != 9 {
            return false;
        }

        true
    }

    fn start_searching_with_choices(&self, domain: &Self::Domain) -> Option<Vec<Self::Choice>> {
        let mut res: Vec<Tile> = Vec::new();

        for i in domain[4].iter() {
            for j in TileIterator::new(*i) {
                res.push(j);
            }
        }

        Some(res)
    }

    fn choices(&self, domain: &Self::Domain) -> Vec<Self::Choice> {
        /*
        grid in task
        0 1 2
        3 4 5
        6 7 8


        grid real indexes:              4 - 1 - 3 - 5 - 7 - 0 - 2 - 6 - 8

        equiwalents stored in program:  0 - 1 - 2 - 3 - 4 - 5 - 6 - 7 - 8
        */

        match self.body.len() {
            1 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(1)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.bottom == self.body[0].top && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            2 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(2)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.right == self.body[0].left && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            3 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(3)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.left == self.body[0].right && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            4 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(4)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.top == self.body[0].bottom && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            5 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(5)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.bottom == self.body[2].top
                            && new_tile.right == self.body[1].left
                            && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            6 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(6)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.bottom == self.body[3].top
                            && new_tile.left == self.body[1].right
                            && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            7 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(7)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.top == self.body[2].bottom
                            && new_tile.right == self.body[4].left
                            && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            8 => {
                let mut res: Vec<Tile> = Vec::new();

                for i in domain[unhash(8)].iter() {
                    for j in TileIterator::new(*i).into_iter().filter(|&new_tile| {
                        new_tile.top == self.body[3].bottom
                            && new_tile.left == self.body[4].right
                            && !self.tile_placed(new_tile.id)
                    }) {
                        res.push(j);
                    }
                }

                res
            },

            9 => Vec::new(),

            _ => unreachable!(),
        }
    }

    fn choose(&mut self, choice: Self::Choice) {
        self.body.push(choice);
    }

    fn unchoose(&mut self, _choice: Self::Choice) {
        let _unused = self.body.pop();
    }
}

fn unhash(index: usize) -> usize {
    // translate index from grid body to real index of tile in grid
    match index {
        0 => 4,
        1 => 1,
        2 => 3,
        3 => 5,
        4 => 7,
        5 => 0,
        6 => 2,
        7 => 6,
        8 => 8,
        _ => unreachable!(),
    }
}

pub trait BTState {
    type Choice: Copy + Debug;
    type Domain;

    fn is_goal(&mut self) -> bool;
    fn start_searching_with_choices(&self, domain: &Self::Domain) -> Option<Vec<Self::Choice>>;
    fn choices(&self, domain: &Self::Domain) -> Vec<Self::Choice>;
    fn choose(&mut self, choice: Self::Choice);
    fn unchoose(&mut self, choice: Self::Choice);
}

pub fn bt_iter_search<BTS, Domain>(
    mut here: BTS,
    domain: &Domain,
) -> Either<Vec<Vec<BTS::Choice>>, BTS>
where
    BTS: BTState<Domain = Domain>,
{
    let mut bt_choices = vec![];
    let initial_choices = here
        .start_searching_with_choices(domain)
        .unwrap_or_else(|| here.choices(domain));
    let mut stack = vec![initial_choices];
    let mut solutions = vec![];

    while let Some(options) = stack.last_mut() {
        let choice = options.pop().unwrap();
        here.choose(choice);
        bt_choices.push(choice);
        if here.is_goal() {
            solutions.push(bt_choices.clone());
            return Either::Left(solutions); // return first solution
        }
        let choices = here.choices(domain);

        if choices.is_empty() {
            here.unchoose(bt_choices.pop().unwrap());
            while stack.last().map_or(false, |opts| opts.is_empty()) {
                stack.pop();
                if !bt_choices.is_empty() {
                    here.unchoose(bt_choices.pop().unwrap());
                }
            }
        } else {
            stack.push(choices);
        }
    }

    if !solutions.is_empty() {
        Either::Left(solutions)
    } else {
        Either::Right(here)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex20_data_from_default_file() {
        assert_eq!(Day20b::solve_default_file().unwrap(), 20899048083289);
    }
}
