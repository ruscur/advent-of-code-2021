use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Inp,
    Mul,
    Add,
    Mod,
    Div,
    Eql,
}

impl Operation {
    fn from(text: &str) -> Operation {
        match text {
            "inp" => Operation::Inp,
            "mul" => Operation::Mul,
            "add" => Operation::Add,
            "mod" => Operation::Mod,
            "div" => Operation::Div,
            "eql" => Operation::Eql,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn from(text: &str) -> Register {
        match text {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Source {
    Immediate(isize),
    Register(Register),
}

impl Source {
    fn from(text: &str) -> Source {
        if ["w", "x", "y", "z"].contains(&text) {
            Source::Register(Register::from(text))
        } else {
            Source::Immediate(text.parse::<isize>().unwrap())
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Insn {
    op: Operation,
    dst: Register,
    src: Option<Source>,
}

fn parse(lines: Vec<String>) -> Result<Vec<Insn>, Box<dyn std::error::Error>> {
    Ok(lines
        .iter()
        .filter_map(|l| {
            if let Some((op, rest)) = l.split_once(" ") {
                let op = Operation::from(op);
                let (dst, src) = if let Some((dst, src)) = rest.split_once(" ") {
                    (Register::from(dst), Some(Source::from(src)))
                } else {
                    (Register::from(rest), None)
                };

                Some(Insn { op, dst, src })
            } else {
                None
            }
        })
        .collect::<Vec<Insn>>())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ALU {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl ALU {
    #[cfg(test)]
    fn new() -> ALU {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn get(&self, reg: Register) -> isize {
        match reg {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
        }
    }

    fn set(&mut self, reg: Register, val: isize) {
        match reg {
            Register::W => self.w = val,
            Register::X => self.x = val,
            Register::Y => self.y = val,
            Register::Z => self.z = val,
        }
    }

    fn get_source_val(&self, src: Source) -> isize {
        match src {
            Source::Immediate(val) => val,
            Source::Register(reg) => self.get(reg),
        }
    }

    fn run(&mut self, insn: Insn) {
        let src_val = self.get_source_val(insn.src.unwrap());
        let dst_val = self.get(insn.dst);
        match insn.op {
            Operation::Inp => self.set(insn.dst, src_val),
            Operation::Mul => self.set(insn.dst, dst_val * src_val),
            Operation::Add => self.set(insn.dst, dst_val + src_val),
            Operation::Mod => self.set(insn.dst, dst_val % src_val),
            Operation::Div => {
                if src_val == 0 {
                    unreachable!()
                } else {
                    self.set(insn.dst, dst_val / src_val)
                }
            }
            Operation::Eql => self.set(insn.dst, (dst_val == src_val) as isize),
        }
    }
}

fn number_to_digits(number: isize) -> Vec<isize> {
    let as_string = number.to_string();
    as_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect::<Vec<isize>>()
}

fn digits_to_number(digits: &Vec<isize>) -> isize {
    digits.iter().enumerate().fold(0, |acc, (n, d)| {
        acc + d * (10_isize.pow((digits.len() - n - 1) as u32) as isize)
    })
}

fn run_program(alu: &mut ALU, insns: &Vec<Insn>, inputs: Vec<isize>) {
    let mut inp_count = 0;
    let insns = insns
        .iter()
        .map(|insn| {
            if insn.op == Operation::Inp {
                let src = Some(Source::Immediate(inputs[inp_count]));
                inp_count += 1;
                Insn {
                    op: insn.op,
                    dst: insn.dst,
                    src,
                }
            } else {
                *insn
            }
        })
        .collect::<Vec<Insn>>();

    for insn in insns {
        alu.run(insn);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR")?);
    path.push("input");

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    let insns = parse(lines)?;

    let mut chunks = insns.split(|insn| insn.op == Operation::Inp);

    // Maps state (value of z) to the lowest value it has been reached from.
    let mut states: BTreeMap<isize, isize> = BTreeMap::new();
    states.insert(0, 0);

    let mut valid_models = vec![];

    while let Some(chunk) = chunks.next() {
        if chunk.is_empty() {
            continue;
        }

        let mut new_lows: BTreeMap<isize, isize> = BTreeMap::new();

        states.iter().for_each(|(state, input)| {
            for digit in 1..10 {
                let mut alu = ALU {
                    z: state.clone(),
                    x: 0,
                    y: 0,
                    w: digit,
                };

                run_program(&mut alu, &chunk.to_vec(), vec![]);

                let mut new_input = number_to_digits(*input);
                new_input.push(digit);
                let score = digits_to_number(&new_input);

                if alu.z == 0 && new_input.len() == 14 {
                    valid_models.push(score);
                } else {
                    if let Some(prev_score) = new_lows.get(&alu.z) {
                        if &score > prev_score {
                            continue;
                        }
                    }
                    new_lows.insert(alu.z, score);
                }
            }
        });
        states = new_lows;
    }

    println!("Part 2: {}", valid_models.iter().min().unwrap());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1_test() {
        let example1 = "inp x
mul x -1"
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let insns = parse(example1).unwrap();

        let mut alu = ALU::new();
        run_program(&mut alu, &insns, vec![5]);
        assert!(alu.x == -5);

        alu = ALU::new();
        run_program(&mut alu, &insns, vec![-5]);
        assert!(alu.x == 5);
    }

    #[test]
    fn example2_test() {
        let example2 = "inp z
inp x
mul z 3
eql z x"
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let insns = parse(example2).unwrap();

        let mut alu = ALU::new();
        run_program(&mut alu, &insns, vec![9, 27]);
        assert!(alu.z == Some(1));
    }

    #[test]
    fn example3_test() {
        let example3 = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<String>>();

        let insns = parse(example3).unwrap();

        let mut alu = ALU::new();
        run_program(&mut alu, &insns, vec![15]);
        assert!(alu.w == 1 && alu.x == 1 && alu.y == 1 && alu.z == 1);

        alu = ALU::new();
        run_program(&mut alu, &insns, vec![85]);
        assert!(alu.w == 0 && alu.x == 1 && alu.y == 0 && alu.z == 1);

        alu = ALU::new();
        run_program(&mut alu, &insns, vec![682]);
        assert!(alu.w == 1 && alu.x == 0 && alu.y == 1 && alu.z == 0);
    }

    #[test]
    fn digits_to_number_test() {
        assert!(
            digits_to_number(&vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9]) == 13579246899999
        );
        assert!(number_to_digits(13579246899999) == vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9]);
    }
}
