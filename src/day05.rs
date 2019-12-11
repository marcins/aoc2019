// FIXME clean up this thing
fn execute(program: &Vec<i32>, input: i32) -> Result<i32, String> {
    let mut ptr: usize = 0;
    let mut program = program.clone();
    let mut output = 0;

    loop {
        let opcode = program[ptr] % 100;
        let modes = (program[ptr] - opcode) / 100;
        let mode0 = modes % 10;
        let mode1 = ((modes - mode0) / 10) % 10;
        let mode2 = (modes - mode0 - mode1) / 100;
        let modes = vec![mode0, mode1, mode2];

        let get_input = |i: usize| {
            if modes[i - 1] == 0 {
                program[program[ptr + i] as usize]
            } else {
                program[ptr + i]
            }
        };

        match opcode {
            1 => {
                let input1 = get_input(1);
                let input2 = get_input(2);
                let output = program[ptr + 3] as usize;
                program[output] = input1 + input2;
                ptr += 4;
            }
            2 => {
                let input1 = get_input(1);
                let input2 = get_input(2);
                let output = program[ptr + 3] as usize;
                program[output] = input1 * input2;
                ptr += 4;
            }
            3 => {
                let loc = program[ptr + 1] as usize;
                program[loc] = input;
                ptr += 2;
            }
            4 => {
                output = program[program[ptr + 1] as usize];
                ptr += 2;
            }
            5 => {
                let input1 = get_input(1);
                let input2 = get_input(2);
                if input1 != 0 {
                    ptr = input2 as usize;
                } else {
                    ptr += 3
                }
            }
            6 => {
                let input1 = get_input(1);
                let input2 = get_input(2);
                if input1 == 0 {
                    ptr = input2 as usize;
                } else {
                    ptr += 3
                }
            }
            7 => {
                let input1 = get_input(1);
                let input2 = get_input(2);
                let output = program[ptr + 3] as usize;
                program[output] = (input1 < input2) as i32;
                ptr += 4;
            }
            8 => {
                let input1 = get_input(1);
                let input2 = get_input(2);
                let output = program[ptr + 3] as usize;
                program[output] = (input1 == input2) as i32;
                ptr += 4
            }
            99 => return Ok(output),
            _ => return Err(format!("Invalid opcode: {}", opcode)),
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    return input
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
}

#[aoc(day5, part1)]
pub fn day5_part1(program: &Vec<i32>) -> i32 {
    execute(program, 1).expect("Execution error")
}

#[aoc(day5, part2)]
pub fn day5_part2(program: &Vec<i32>) -> i32 {
    execute(program, 5).expect("Execution error")
}
