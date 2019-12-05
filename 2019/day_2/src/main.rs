fn main()
{
	// test();
	// part1();
	part2();
}

fn part1()
{
	run(&mut parse(INPUT1), true);
}

fn part2()
{

	let mut result:usize = 0;
	let mut run_count = 0;
	for noun in (0..100)
	{
		for verb in (0..100)
		{
			run_count += 1;
			let mut program = parse(INPUT1);			
			program[1] = noun;
			program[2] = verb;
			run(&mut program, false);
			if program[0] == 19690720
			{
				let answer = 100 * noun + verb;
				println!("run:{},noun:{}, verb:{}, answer:{}",run_count,noun, verb,answer);
				return;
			}
		}
	}
}

fn test()
{
		run(&mut parse(TEST1), false);
		run(&mut parse(TEST2), false);
		run(&mut parse(TEST3), false);
		run(&mut parse(TEST4), false);
		run(&mut parse(TEST5), false);}

fn parse(input:&str) -> Vec<usize>
{
	let program:Vec<usize>  = input.split(",").map(|i|{i.trim().parse().unwrap()}).collect();
	return program;
}

fn run(program:&mut Vec<usize>, restore_state:bool)
{
	
	if restore_state
	{
		program[1] = 12;
		program[2] = 2;
	}
//	println!("instruction size = {}, proper instruction count?: {}", program.len(), if program.len() % 4 == 0  {true} else {false});
	for p in (0..program.len()).step_by(4)
	{
		let p = p as usize;
		let opcode = program[p];
		let opcode = match opcode
		{
			1 => OpCode::Add,
			2 => OpCode::Multiply,
			_ => OpCode::Exit,			
		};								
		
		if let OpCode::Exit = opcode
		{
			break;
		}
		// print!("{:?}: ", opcode);
		
		
		let lvalue_index = program[p+1];
		if lvalue_index >= program.len()
		{
			break;
		}
		let lvalue = program[lvalue_index];
		
				
		let rvalue_index = program[p+2];
		if rvalue_index >= program.len()
		{
			break;
		}
		let rvalue = program[rvalue_index];
		
		
		let result_index = program[p+3];		
		if result_index >= program.len()
		{
			break;
		}
		let result = match opcode
		{
			OpCode::Add => lvalue + rvalue,
			OpCode::Multiply => lvalue * rvalue,
			_ => 0
		};
		// print!("{} {} => {}\n", lvalue, rvalue, result);
		
		program[result_index] = result;
		
		
	}
	// println!("Result after running: {:?}", program);
	// println!("Position 0value left: {}", program[0]);

}

#[derive(Debug)]
enum OpCode
{
	Add = 1, 
	Multiply = 2,
	Exit = 99,
}
const INPUT1:&str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,5,23,2,9,23,27,1,5,27,31,1,5,31,35,1,35,13,39,1,39,9,43,1,5,43,47,1,47,6,51,1,51,13,55,1,55,9,59,1,59,13,63,2,63,13,67,1,67,10,71,1,71,6,75,2,10,75,79,2,10,79,83,1,5,83,87,2,6,87,91,1,91,6,95,1,95,13,99,2,99,13,103,1,103,9,107,1,10,107,111,2,111,13,115,1,10,115,119,1,10,119,123,2,13,123,127,2,6,127,131,1,13,131,135,1,135,2,139,1,139,6,0,99,2,0,14,0";
const TEST1:&str = "1,9,10,3,2,3,11,0,99,30,40,50";
const TEST2:&str = "1,0,0,0,99";
const TEST3:&str = "2,3,0,3,99";
const TEST4:&str = "2,4,4,5,99,0";
const TEST5:&str = "1,1,1,4,99,5,6,0,99";