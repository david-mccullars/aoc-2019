use num::{cast, NumCast};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;

pub trait IntcodeIO {
    fn input(&mut self) -> Option<i64>;

    fn output(&mut self, value: i64);
}

#[derive(PartialEq)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn from(mode: i64) -> Self {
        match mode % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Invalid mode {:?}", mode % 10),
        }
    }

    fn from3(mode: i64) -> (Self, Self, Self) {
        (
            Self::from(mode),
            Self::from(mode / 10),
            Self::from(mode / 100),
        )
    }
}

#[derive(PartialEq)]
enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    ShiftBase,
    Halt,
}

impl Instruction {
    fn from(value: i64) -> Self {
        match value % 100 {
            1 => Instruction::Add,
            2 => Instruction::Multiply,
            3 => Instruction::Input,
            4 => Instruction::Output,
            5 => Instruction::JumpIfTrue,
            6 => Instruction::JumpIfFalse,
            7 => Instruction::LessThan,
            8 => Instruction::Equals,
            9 => Instruction::ShiftBase,
            99 => Instruction::Halt,
            _ => panic!("Invalid instruction {:?}", value % 100),
        }
    }
}

#[derive(Clone, Default)]
pub struct Intcode {
    code: Vec<i64>,
    ptr: usize,
    relbase: i64,
    halt_after_output: bool,
}

impl Intcode {
    pub fn new(codestr: &str) -> Self {
        let code: Vec<i64> = codestr
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            code,
            ptr: 0,
            relbase: 0,
            halt_after_output: false,
        }
    }

    pub fn halt_after_output(&mut self, value: bool) {
        self.halt_after_output = value;
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn set_code(&mut self, position: usize, value: i64) {
        if position >= self.code.len() {
            self.code.resize(position + 10, 0);
        }
        self.code[position] = value;
    }

    pub fn run<I, O>(&mut self, mut input: I, mut output: O)
    where
        I: FnMut() -> Option<i64>,
        O: FnMut(i64),
    {
        loop {
            let (instruction, (m1, m2, m3)) = self.read_instruction();
            match instruction {
                Instruction::Add => {
                    let (v1, v2) = self.read2(m1, m2);
                    self.write(v1 + v2, m3);
                }
                Instruction::Multiply => {
                    let (v1, v2) = self.read2(m1, m2);
                    self.write(v1 * v2, m3);
                }
                Instruction::Input => {
                    if let Some(value) = input() {
                        self.write(value, m1);
                    } else {
                        self.ptr -= 1; // Reset the pointer
                        break;
                    }
                }
                Instruction::Output => {
                    let value = self.read(m1);
                    output(value);
                    if self.halt_after_output {
                        break;
                    }
                }
                Instruction::JumpIfTrue => {
                    let (v1, v2) = self.read2(m1, m2);
                    if v1 != 0 {
                        assert!(v2 >= 0, "Pointer is negative");
                        self.ptr = v2 as usize;
                    }
                }
                Instruction::JumpIfFalse => {
                    let (v1, v2) = self.read2(m1, m2);
                    if v1 == 0 {
                        assert!(v2 >= 0, "Pointer is negative");
                        self.ptr = v2 as usize;
                    }
                }
                Instruction::LessThan => {
                    let (v1, v2) = self.read2(m1, m2);
                    self.write(if v1 < v2 { 1 } else { 0 }, m3);
                }
                Instruction::Equals => {
                    let (v1, v2) = self.read2(m1, m2);
                    self.write(if v1 == v2 { 1 } else { 0 }, m3);
                }
                Instruction::ShiftBase => {
                    let v1 = self.read(m1);
                    self.relbase += v1;
                }
                Instruction::Halt => {
                    break;
                }
            }
        }
    }

    pub fn run_with_no_io(&mut self) {
        self.run(|| None, |_| {});
    }

    pub fn run_with_io(&mut self, io: &mut dyn IntcodeIO) {
        let m = Mutex::new(io);
        self.run(
            || m.lock().unwrap().input(),
            |v| m.lock().unwrap().output(v),
        );
    }

    pub fn run_simple<T: Copy + NumCast>(&mut self, input: &[T]) -> Option<i64> {
        let mut output = None;
        self.run(input_fn(input), |v| output = Some(v));
        output
    }

    pub fn run_simplen<T: Copy + NumCast>(&mut self, input: &[T]) -> Vec<i64> {
        let mut output = vec![];
        self.run(input_fn(input), |v| output.push(v));
        output
    }

    pub fn run_ascii(&mut self, input: &str) -> Option<i64> {
        let input: Vec<_> = input.chars().map(|c| c as i64).collect();
        self.run_simple(&input)
    }

    pub fn run_ascii_and_capture(&mut self, input: &str) -> String {
        let input: Vec<_> = input.chars().map(|c| c as i64).collect();
        let mut output = String::new();
        self.run(input_fn(&input), |v| output.push((v as u8) as char));
        output
    }

    pub fn run_async(&mut self, input: &Receiver<i64>, output: &Sender<i64>) {
        self.run(
            || input.recv().ok(),
            |v| output.send(v).expect("Failure sending output"),
        );
    }

    fn code_ati(&self, ptr: i64) -> i64 {
        assert!(ptr >= 0, "Pointer is negative");
        self.code_at(ptr as usize)
    }

    pub fn code_at(&self, ptr: usize) -> i64 {
        if ptr < self.code.len() {
            self.code[ptr]
        } else {
            0
        }
    }

    fn read_instruction(&mut self) -> (Instruction, (Mode, Mode, Mode)) {
        let value = self.code_at(self.ptr);
        self.ptr += 1;
        (Instruction::from(value), Mode::from3(value / 100))
    }

    fn read(&mut self, mode: Mode) -> i64 {
        let value = self.code_at(self.ptr);
        self.ptr += 1;
        match mode {
            Mode::Position => self.code_ati(value),
            Mode::Immediate => value,
            Mode::Relative => self.code_ati(value + self.relbase),
        }
    }

    fn read2(&mut self, m1: Mode, m2: Mode) -> (i64, i64) {
        let v1 = self.read(m1);
        let v2 = self.read(m2);
        (v1, v2)
    }

    fn write(&mut self, value: i64, mode: Mode) {
        let ptr = self.code_at(self.ptr);
        let ptr = match mode {
            Mode::Position => ptr,
            Mode::Immediate => ptr,
            Mode::Relative => ptr + self.relbase,
        };
        let _ = self.code_ati(ptr);
        self.set_code(ptr as usize, value);
        self.ptr += 1;
    }
}

pub fn input_fn<T: Copy + NumCast>(input: &[T]) -> impl FnMut() -> Option<i64> + '_ {
    let mut input_ptr = 0;
    move || {
        input_ptr += 1;
        if input_ptr <= input.len() {
            Some(cast(input[input_ptr - 1]).unwrap())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_intcode_one() {
        let mut intcode = Intcode::new("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(intcode.run_simple(&[1]), None);
        assert_eq!(intcode.code, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_run_intcode_two() {
        let mut intcode = Intcode::new("1,0,0,0,99");
        assert_eq!(intcode.run_simple(&[1]), None);
        assert_eq!(intcode.code, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_run_intcode_three() {
        let mut intcode = Intcode::new("2,3,0,3,99");
        assert_eq!(intcode.run_simple(&[1]), None);
        assert_eq!(intcode.code, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_run_intcode_four() {
        let mut intcode = Intcode::new("2,4,4,5,99,0");
        assert_eq!(intcode.run_simple(&[1]), None);
        assert_eq!(intcode.code, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_run_intcode_five() {
        let mut intcode = Intcode::new("1,1,1,4,99,5,6,0,99");
        assert_eq!(intcode.run_simple(&[1]), None);
        assert_eq!(intcode.code, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_run_intcode_six() {
        let mut intcode = Intcode::new("3,0,4,0,99");
        assert_eq!(intcode.run_simple(&[19132]), Some(19132));
        assert_eq!(intcode.code, [19132, 0, 4, 0, 99]);
    }

    #[test]
    fn test_run_intcode_seven() {
        let mut intcode = Intcode::new("1002,4,3,4,33");
        assert_eq!(intcode.run_simple(&[19132]), None);
        assert_eq!(intcode.code, [1002, 4, 3, 4, 99]);
    }
}
