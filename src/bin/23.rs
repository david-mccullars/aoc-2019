#[allow(unused_imports)]
use advent_of_code::*;
use std::collections::VecDeque;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<i64> {
    Network::new(input, 50).run()
}

pub fn part_two(input: &str) -> Option<i64> {
    Network::new(input, 50).run_with_nat_resume()
}

struct Network {
    computers: Vec<Computer>,
    router: Router,
}

impl Network {
    fn new(input: &str, count: usize) -> Self {
        let computers = (0..count).map(|addr| Computer::new(input, addr)).collect();
        let router = Router::new(count);
        Self { computers, router }
    }

    fn run(&mut self) -> Option<i64> {
        self._run(|_| true)
    }

    fn run_with_nat_resume(&mut self) -> Option<i64> {
        let mut prev_nat_msg = None;
        self._run(|router| {
            let nat_msg = router.nat();
            if nat_msg == prev_nat_msg {
                true
            } else {
                router.push(0, nat_msg.unwrap());
                prev_nat_msg = nat_msg;
                false
            }
        })
    }

    fn _run<F>(&mut self, mut stop: F) -> Option<i64>
    where
        F: FnMut(&mut Router) -> bool,
    {
        loop {
            for i in 0..self.computers.len() {
                self.computers[i].run(&mut self.router);
            }
            if self.router.is_empty() && stop(&mut self.router) {
                break;
            }
        }
        self.router.naty()
    }
}

struct Computer {
    address: usize,
    nic: Intcode,
}

impl Computer {
    fn new(input: &str, address: usize) -> Self {
        let mut nic = Intcode::new(input);
        nic.run_simple(&[address]);
        Self { address, nic }
    }

    fn run(&mut self, router: &mut Router) {
        let output = match router.pop(self.address) {
            Some(msg) => self.nic.run_simplen(&[msg.0, msg.1]),
            None => self.nic.run_simplen(&[-1]),
        };

        assert!(output.len() % 3 == 0);
        for c in output.chunks(3) {
            router.push(c[0] as usize, (c[1], c[2]));
        }
    }
}

type Message = (i64, i64);

struct Router {
    queues: Vec<VecDeque<Message>>,
    nat: Option<Message>,
}

impl Router {
    fn new(size: usize) -> Self {
        Self {
            queues: (0..size).map(|_| VecDeque::new()).collect(),
            nat: None,
        }
    }

    fn pop(&mut self, address: usize) -> Option<Message> {
        assert!(address < self.queues.len());
        self.queues[address].pop_front()
    }

    fn nat(&mut self) -> Option<Message> {
        self.nat
    }

    fn naty(&mut self) -> Option<i64> {
        self.nat.map(|msg| msg.1)
    }

    fn push(&mut self, address: usize, msg: Message) {
        if address == 255 {
            self.nat = Some(msg);
        } else {
            assert!(address < self.queues.len());
            self.queues[address].push_back(msg);
        }
    }

    fn is_empty(&self) -> bool {
        self.queues.iter().all(|q| q.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23954));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17265));
    }
}
