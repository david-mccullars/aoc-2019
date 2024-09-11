#[allow(unused_imports)]
use advent_of_code::*;

use hashbrown::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use Direction::*;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let scan = Scan::new(input);
    scan.item_combinations()
        .into_par_iter()
        .find_map_first(|items_needed| {
            let mut droid = Droid::new(input);
            droid.init();
            droid.collect(items_needed, &scan);
            droid.attempt_password(&scan)
        })
}

pub fn part_two(_input: &str) -> Option<&str> {
    Some("CLAIM THE FINAL GOLD STAR!!!")
}

const UNSAFE: [&str; 5] = [
    "molten lava",
    "photons",
    "escape pod",
    "infinite loop",
    "giant electromagnet",
];

#[derive(Default)]
struct Scan {
    rooms: HashMap<String, Vec<Direction>>,
    items: HashMap<String, String>,
    password_route: Vec<Direction>,
}

impl Scan {
    fn new(input: &str) -> Self {
        let mut scan = Scan::default();
        let mut droid = Droid::new(input);
        scan.scan_room(droid.init(), &mut vec![], &mut droid);
        scan
    }

    fn scan_room(&mut self, status: Status, route: &mut Vec<Direction>, droid: &mut Droid) {
        self.rooms.insert(status.room.clone(), route.clone());
        for item in &status.items {
            if !UNSAFE.contains(&item.as_str()) {
                self.items.insert(item.clone(), status.room.clone());
            }
        }

        let prev = if route.is_empty() {
            None
        } else {
            Some(route[route.len() - 1])
        };
        for door in status.doors {
            if prev != Some(door.invert()) {
                route.push(door);
                let status2 = droid.mv(door);
                if status2.password_required {
                    // We were force moved, so just keep track of this route
                    self.password_route = route.clone();
                } else {
                    self.scan_room(status2, route, droid);
                    droid.mv(door.invert());
                }
                route.pop();
            }
        }
    }

    fn item_combinations(&self) -> Vec<Vec<&String>> {
        self.items.keys().powerset().collect()
    }
}

struct Droid(Intcode);

impl Droid {
    fn new(input: &str) -> Self {
        Self(Intcode::new(input))
    }

    fn run(&mut self, cmd: &str) -> Status {
        Status::parse(&self.0.run_ascii_and_capture(cmd))
    }

    fn init(&mut self) -> Status {
        self.run("")
    }

    fn mv(&mut self, dir: Direction) -> Status {
        self.run(match dir {
            North => "north\n",
            South => "south\n",
            East => "east\n",
            West => "west\n",
        })
    }

    fn mv_all(&mut self, dirs: impl Iterator<Item = Direction>) -> Option<Status> {
        dirs.map(|dir| self.mv(dir)).last()
    }

    fn take(&mut self, item: &String) {
        self.0.run_ascii_and_capture(&format!("take {}\n", item));
    }

    fn collect(&mut self, items: Vec<&String>, scan: &Scan) {
        for item in items {
            let room = scan.items.get(item).unwrap();
            let route = scan.rooms.get(room).unwrap();
            self.mv_all(route.iter().copied());
            self.take(item);
            self.mv_all(route.iter().rev().map(|d| d.invert()));
        }
    }

    fn attempt_password(&mut self, scan: &Scan) -> Option<u32> {
        self.mv_all(scan.password_route.iter().copied())
            .and_then(|s| s.password)
    }
}

#[derive(Clone)]
struct Status {
    room: String,
    doors: Vec<Direction>,
    items: Vec<String>,
    password: Option<u32>,
    password_required: bool,
}

impl Status {
    fn parse(text: &str) -> Self {
        match Self::parser().parse(text) {
            Ok(status) => status,
            Err(err) => panic!("Failure parsing text: {}\n{}", err, text),
        }
    }

    fn parser() -> impl Parser<Output = Status> {
        let password = parser!(
            line("A loud, robotic voice says \"Analysis complete! You may proceed.\" and you enter the cockpit.")
            line("Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.")
            password:line("\"Oh, hello! You should be able to get in by typing " u32 " on the keypad at the main airlock.\"")
            => password
        );

        let password_required = parser!(
            line("A loud, robotic voice says \"Alert! Droids on this ship are " any_char+ " than the detected value!\" and you are ejected back to the checkpoint.")
            lines(string(any_char*))
            => true
        );

        let prompt = parser!(
            lines("")
            line("Command?")
        );

        parser!(
            lines("")

            room:line("== " string(any_char+) " ==")
            line(string(any_char+))

            line("")

            line("Doors here lead:")
            doors:line("- " { "north" => North, "south" => South, "east" => East, "west" => West })+

            line("")

            line("Items here:")?
            items:line("- " string(any_char+))*

            lines("")

            password:password?
            password_required:password_required?
            prompt?
            => Status { room, doors, items, password, password_required: password_required.unwrap_or(false) }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(134349952));
    }
}
