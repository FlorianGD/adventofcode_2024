use std::fmt::Display;

type Registers = [usize; 3];
type Program = Vec<usize>;

pub fn parse_input(input: &str) -> (Registers, Program) {
    let mut lines = input.lines();
    let register_a = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let register_b = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let register_c = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    lines.next();
    let program = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    ([register_a, register_b, register_c], program)
}

#[derive(Debug, Clone)]
enum Operand {
    Zero,
    One,
    Two,
    Three,
    RegisterA,
    RegisterB,
    RegisterC,
    Invalid,
}

impl From<usize> for Operand {
    fn from(value: usize) -> Self {
        match value {
            0 => Operand::Zero,
            1 => Operand::One,
            2 => Operand::Two,
            3 => Operand::Three,
            4 => Operand::RegisterA,
            5 => Operand::RegisterB,
            6 => Operand::RegisterC,
            7 => Operand::Invalid,
            c => panic!("Invalid value found {c}"),
        }
    }
}

impl Operand {
    fn literal_value(&self) -> usize {
        match self {
            Operand::Zero => 0,
            Operand::One => 1,
            Operand::Two => 2,
            Operand::Three => 3,
            Operand::RegisterA => 4,
            Operand::RegisterB => 5,
            Operand::RegisterC => 6,
            Operand::Invalid => 7,
        }
    }

    fn combo_value(&self, registers: &Registers) -> usize {
        match self {
            Operand::Zero => 0,
            Operand::One => 1,
            Operand::Two => 2,
            Operand::Three => 3,
            Operand::RegisterA => registers[0],
            Operand::RegisterB => registers[1],
            Operand::RegisterC => registers[2],
            Operand::Invalid => panic!("Invalid operand value"),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum Instruction {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl From<usize> for Instruction {
    fn from(value: usize) -> Self {
        match value {
            0 => Instruction::adv,
            1 => Instruction::bxl,
            2 => Instruction::bst,
            3 => Instruction::jnz,
            4 => Instruction::bxc,
            5 => Instruction::out,
            6 => Instruction::bdv,
            7 => Instruction::cdv,
            c => panic!("Invalid value found {c}"),
        }
    }
}

fn compute(
    instruction: Instruction,
    operand: Operand,
    registers: &mut Registers,
    instruction_pointer: usize,
    outputs: &mut Vec<usize>,
) -> usize {
    match instruction {
        Instruction::adv => {
            let numerator = registers[0];
            let denominator = 2usize.pow(operand.combo_value(registers) as u32);
            registers[0] = numerator / denominator;
            instruction_pointer + 2
        }
        Instruction::bdv => {
            let numerator = registers[0];
            let denominator = 2usize.pow(operand.combo_value(registers) as u32);
            registers[1] = numerator / denominator;
            instruction_pointer + 2
        }
        Instruction::cdv => {
            let numerator = registers[0];
            let denominator = 2usize.pow(operand.combo_value(registers) as u32);
            registers[2] = numerator / denominator;
            instruction_pointer + 2
        }
        Instruction::bst => {
            registers[1] = operand.combo_value(registers) % 8;
            instruction_pointer + 2
        }
        Instruction::jnz => {
            if registers[0] == 0 {
                instruction_pointer + 2
            } else {
                operand.literal_value()
            }
        }
        Instruction::bxl => {
            let new_val = registers[1] ^ operand.literal_value();
            registers[1] = new_val;
            instruction_pointer + 2
        }
        Instruction::bxc => {
            registers[1] ^= registers[2];
            instruction_pointer + 2
        }
        Instruction::out => {
            outputs.push(operand.combo_value(registers) % 8);
            instruction_pointer + 2
        }
    }
}

fn execute_program(program: &Program, registers: &mut Registers) -> Vec<usize> {
    let mut outputs = Vec::new();
    let mut instruction_pointer = 0usize;
    while instruction_pointer < program.len() - 1 {
        let instruction = program[instruction_pointer].into();
        let operand = program[instruction_pointer + 1].into();
        instruction_pointer = compute(
            instruction,
            operand,
            registers,
            instruction_pointer,
            &mut outputs,
        );
    }
    outputs
}

fn _format_for_output<T: Display>(outputs: &[T]) -> String {
    let mut iterator = outputs.iter();

    let head = match iterator.next() {
        None => return String::from(""),
        Some(x) => format!("{}", x),
    };
    iterator.fold(head, |a, v| format!("{},{}", a, v))
}

pub fn part1((mut registers, program): (Registers, Program)) -> String {
    let outputs = execute_program(&program, &mut registers);
    dbg!(&registers);
    _format_for_output(&outputs)
}

fn _test_register_a(val_a: usize, registers: &mut Registers, program: &Program) -> usize {
    registers[0] = val_a;
    let outputs = execute_program(program, registers);
    outputs.iter().fold(0, |acc, v| acc * 8 + v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    const INPUT: &str = indoc! {
        "Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
        "
    };
    const INPUT_P2: &str = indoc! {
        "Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0"
    };

    #[test]
    fn test_parse_input() {
        let (registers, program) = parse_input(INPUT);
        assert_eq!(registers, [729, 0, 0]);
        assert_eq!(program, Vec::from_iter([0, 1, 5, 4, 3, 0]));
    }

    #[test]
    fn test_compute() {
        let mut registers = [0, 0, 9];
        let program = vec![2, 6];
        let outputs = execute_program(&program, &mut registers);
        assert!(outputs.is_empty());
        assert_eq!(registers[1], 1);

        let mut registers = [10, 0, 9];
        let program = vec![5, 0, 5, 1, 5, 4];
        let outputs = execute_program(&program, &mut registers);
        assert_eq!(outputs, vec![0, 1, 2]);

        let mut registers = [2024, 0, 9];
        let program = vec![0, 1, 5, 4, 3, 0];
        let outputs = execute_program(&program, &mut registers);
        assert_eq!(outputs, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(registers[0], 0);

        let mut registers = [2024, 29, 9];
        let program = vec![1, 7];
        let _outputs = execute_program(&program, &mut registers);
        assert_eq!(registers[1], 26);

        let mut registers = [0, 2024, 43690];
        let program = vec![4, 0];
        let _outputs = execute_program(&program, &mut registers);
        assert_eq!(registers[1], 44354);
    }

    #[test]
    fn test_part1() {
        let (registers, program) = parse_input(INPUT);
        let outputs = part1((registers, program));
        assert_eq!(outputs, "4,6,3,5,6,3,5,2,1,0".to_string())
    }
}
