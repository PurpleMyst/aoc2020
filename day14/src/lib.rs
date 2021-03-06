use rustc_hash::FxHashMap as HashMap;

const MASK_HEADER: &str = "mask = ";
const SET_HEADER: &str = "mem[";

const INT_SIZE: usize = 36;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    Mask { mask: u64, metamask: u64 },
    Set { address: usize, value: u64 },
}

pub fn solve_part1(instructions: &[Instruction]) -> u64 {
    let mut mem = [0u64; 1 << 16];

    let mut mask = 0;
    let mut metamask = 0;
    for &instruction in instructions {
        match instruction {
            Instruction::Mask {
                mask: next_mask,
                metamask: next_metamask,
            } => {
                mask = next_mask;
                metamask = next_metamask;
            }

            Instruction::Set { address, value } => {
                mem[address] = (value & !metamask) | mask;
            }
        }
    }

    mem.iter().sum()
}

fn set(memory: &mut HashMap<u64, u64>, metamask: u64, address: u64, value: u64) {
    let floaty = !metamask & ((1 << INT_SIZE) - 1);
    let mut xor = 1 + floaty;
    (0..1u64 << floaty.count_ones()).for_each(|_| {
        xor = (xor - 1) & floaty;
        memory.insert(address ^ xor, value);
    });
}

pub fn solve_part2(instructions: &[Instruction]) -> u64 {
    let mut memory = HashMap::with_capacity_and_hasher(1 << 16, Default::default());

    let mut mask = 0;
    let mut metamask = 0;
    for &instruction in instructions {
        match instruction {
            Instruction::Mask {
                mask: next_mask,
                metamask: next_metamask,
            } => {
                mask = next_mask;
                metamask = next_metamask;
            }

            Instruction::Set { address, value } => {
                let address = address as u64 | mask;
                set(&mut memory, metamask, address, value);
            }
        }
    }

    memory.values().sum()
}

pub fn parse_input() -> Vec<Instruction> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            if line.starts_with(MASK_HEADER) {
                let mut mask = 0;
                let mut metamask = 0;

                line.bytes()
                    .skip(MASK_HEADER.len())
                    .enumerate()
                    .filter(|&(_, b)| b != b'X')
                    .for_each(|(i, b)| {
                        let i = INT_SIZE - (i + 1);
                        mask |= u64::from(b - b'0') << i;
                        metamask |= 1 << i;
                    });

                Instruction::Mask { mask, metamask }
            } else {
                let mut parts = line[SET_HEADER.len()..].splitn(2, "] = ");
                let address = parts.next().unwrap().parse::<usize>().unwrap();
                let value = parts.next().unwrap().parse::<u64>().unwrap();
                Instruction::Set { address, value }
            }
        })
        .collect()
}

#[inline]
pub fn solve() -> (u64, u64) {
    let instructions = parse_input();
    (solve_part1(&instructions), solve_part2(&instructions))
}
