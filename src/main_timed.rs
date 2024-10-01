/*
This file is intended for timing how fast Pseudokude is able to solve the same puzzle
a certain number of times in a row. Certain functionality is stripped from this version,
including printing the board while solving and remembering details about cells which would
be used later for coloring.

This file also contains multiple example boards to stress-test Pseudokude with.
*/

use std::process::Command;
use std::time::{Instant};
use colored::Colorize;

use std::io::{self, Write};


//Individual cell holding all aoe information.
#[derive(Clone)]
pub struct Cell {
	digit: u16, //Digit of cell
	row: Vec<[usize; 2]>, //Coordinates of cell's row
	col: Vec<[usize; 2]>, //Coordinates of cell's col
	house: Vec<[usize; 2]>, //Coordinates of cell's house
	aoe: Vec<[usize; 2]>, //Coordinates of cell's aoe
	p: Vec<u16>, //Possibilities of current cell
	p_limit: Vec<u16>, //Restrictions on possibilities
}
impl Cell {

	//Constructor
	pub fn new() -> Self {
		Self {
			digit: 0,
			row: vec![],
			col: vec![],
			house: vec![],
			aoe: vec![],
			p: vec![],
			p_limit: vec![],
		}
	}
}

//Entire board containing size information and 2d vector of cells.
#[derive(Clone)]
pub struct Board {
	bsize: usize, //Board side-length
	hsize: usize, //House side-length
	last_modified: [usize; 3], //Information about the last-modified cell.
	cell: Vec<Vec<Cell>>, //2D vector containing all cells
	solved: bool,
}
impl Board {

	//Constructor
	pub fn new(bsize: usize) -> Self {
		Self {
			bsize,
			hsize: (bsize as f64).sqrt() as usize,
			last_modified: [0,0,0],
			cell: vec![],
			solved: true,
		}
	}

	//Initialize values of board from given input
	fn init(&mut self, init: &Vec<Vec<u16>>) {

		let mut hx: usize;
		let mut hy: usize;

		//Iterate through row
		for i in 0..self.bsize {

			//Initialize row
			self.cell.push(Vec::new());

			//Iterate through column
			for j in 0.. self.bsize {

				//Initialize cell
				self.cell[i].push(Cell::new());

				//Assign digit to cell
				self.cell[i][j].digit = init[i][j];

				//Initialize row and column coordinates
				for k in 0..self.bsize {
					if k != j {
						self.cell[i][j].row.push([i,k]);
					}
					if k != i {
						self.cell[i][j].col.push([k,j]);
					}
				}

				//The top-left coordinate for the cell's house
				hy = (((i/self.hsize) as f64).floor() as usize)*(self.hsize);
				hx = (((j/self.hsize) as f64).floor() as usize)*(self.hsize);

				//Iterate from top-left of house and add to cell's house and aoe coordinates.
				for k in 0..self.hsize {
					for l in 0..self.hsize {
						if i != (k+hy) || j != (l+hx) {
							self.cell[i][j].house.push([(k+hy),(l+hx)]);
							self.cell[i][j].aoe.push([(k+hy),(l+hx)]);
						}
					}
				}


				//Initialize AOE coordinates
				for k in (self.hsize-(j%self.hsize)+j)..self.bsize {
					self.cell[i][j].aoe.push([i, k]); //Row after house
				}

				for k in 0..(j+(self.hsize-(j%self.hsize))-self.hsize) {
					self.cell[i][j].aoe.push([i, k]); //Row before house
				}

				for k in (self.hsize-(i%self.hsize)+i)..self.bsize {
					self.cell[i][j].aoe.push([k, j]); //Column after house
				}

				for k in 0..(i+(self.hsize-(i%self.hsize))-self.hsize) {
					self.cell[i][j].aoe.push([k, j]); //Column before house
				}
			}
		}
	}

	//Show current state of board
	fn show(&self) {

		let mut output = String::from("");

		//How much space, including whitespace, each digit needs.
		let space_per_digit = ((self.bsize as f64).log10()+2.0).floor() as usize;
		
		print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		output.push_str("\n");

		//Main loop
		for i in 0..self.bsize {
			for j in 0..self.bsize {

				//Ensure enough white-space before digit.
				if self.cell[i][j].digit != 0 {
					for _ in 0..space_per_digit-(((self.cell[i][j].digit).checked_ilog10().unwrap_or(0)+2) as usize) {
						output.push_str(" ");
					}
					output.push_str(&format!("{}", self.cell[i][j].digit.to_string()));
				} else {
					for _ in 0..space_per_digit-1 {
						output.push_str(" ");
					}
				}

				//Add vertical line when end of house is reached.
				if (j+1) % self.hsize == 0 && (j+1) != (self.bsize) {
					output.push_str("|");
				} else {
					output.push_str(" ");
				}
			}
			output.push_str("\n");

			//Add horizontal line when end of house is reached.
			if (i+1) % self.hsize == 0 && (i+1) != (self.bsize) {
				for k in 0..self.hsize { 
					for _ in 0..(self.hsize*space_per_digit)-1 {
						output.push_str("―");
					}
					if k != self.hsize-1 {
						output.push_str("+");
					}
				}
				output.push_str("\n");
			}
		}

		print!("{}", output);
	}

	//Returns a vector of digits OR possibilities from a vector of coordinates
	fn coords_to_digits(&self, area: &Vec<[usize; 2]>, return_p: bool) -> Vec<u16> {
		let mut output: Vec<u16> = vec![];

		//Iterate through area
		for each in area {

			//Whether to return area's digits or all of area's possibilities
			if return_p {
				if self.cell[each[0]][each[1]].digit == 0 {
					for each in &self.cell[each[0]][each[1]].p {
						output.push(*each);
					}
				}
			} else {
				if self.cell[each[0]][each[1]].digit != 0 {
					output.push(self.cell[each[0]][each[1]].digit);
				}
			}
		}
		return output;
	}

	//Updates the possibilities of all cells, restricted by p_limit.
	fn update_p(&mut self, c: [usize; 2]) {
		let mut row: Vec<u16>; //Current cell's row
		let mut col: Vec<u16>; //Current cell's col
		let mut house: Vec<u16>; //Current cell's house
		let mut p_len: u16;
		
		for each in &self.cell[c[0]][c[1]].aoe.clone() {
			if self.cell[each[0]][each[1]].digit == 0 {

				p_len = 0;
				self.cell[each[0]][each[1]].p.clear();

				//Assign all possibilities, restricted by limit and p_limit.
				row = self.coords_to_digits(&self.cell[each[0]][each[1]].row, false);
				col = self.coords_to_digits(&self.cell[each[0]][each[1]].col, false);
				house = self.coords_to_digits(&self.cell[each[0]][each[1]].house, false);

				for k in 1..(self.bsize+1) {
					if !row.contains(&(k as u16)) && !col.contains(&(k as u16)) && !house.contains(&(k as u16)) && !self.cell[each[0]][each[1]].p_limit.contains(&(k as u16)) {
						self.cell[each[0]][each[1]].p.push(k as u16);
						p_len += 1;
					}
				}

				//If there is only 1 possibility, set it as the digit and restart.
				if p_len == 1 {
					self.cell[each[0]][each[1]].digit = self.cell[each[0]][each[1]].p[0];
					self.update_p([each[0], each[1]]);
				}
			}
		}
	}

	//Updates the possibilities of all cells, restricted by p_limit.
	fn update_all_p(&mut self) {
		let mut row: Vec<u16>; //Current cell's row
		let mut col: Vec<u16>; //Current cell's col
		let mut house: Vec<u16>; //Current cell's house
		
		//Iterate through cells
		for i in 0..self.bsize {
			for j in 0..self.bsize {
				//Ensure cell is a 0
				if self.cell[i][j].digit == 0 {

					self.cell[i][j].p.clear();

					row = self.coords_to_digits(&self.cell[i][j].row, false);
					col = self.coords_to_digits(&self.cell[i][j].col, false);
					house = self.coords_to_digits(&self.cell[i][j].house, false);

					//Assign all possibilities, restricted by limit and p_limit.
					for k in 1..(self.bsize+1) {
						if !row.contains(&(k as u16)) && !col.contains(&(k as u16)) && !house.contains(&(k as u16)) && !self.cell[i][j].p_limit.contains(&(k as u16)) {
							self.cell[i][j].p.push(k as u16);
						}
					}
				}
			}
		}
		
	}

	
	//Checks for lone-possibility's and updates all possibilities.
	fn process_of_elimination(&mut self) {
		let mut p: u16; //Current cell's possibility
		let mut row: Vec<u16>; //Current cell's row
		let mut col: Vec<u16>; //Current cell's col
		let mut house: Vec<u16>; //Current cell's house
		let mut reset: bool = true;

		while reset {
			self.solved = true;
			reset = false;
			for i in 0..self.bsize {
				for j in 0..self.bsize {
					//Ensure cell is a 0
					if self.cell[i][j].digit == 0 {
						self.solved = false;
						row = self.coords_to_digits(&self.cell[i][j].row, true);
						col = self.coords_to_digits(&self.cell[i][j].col, true);
						house = self.coords_to_digits(&self.cell[i][j].house, true);

						//If area's do not contain a possibility, then set digit to possibility.
						for k in 0..self.cell[i][j].p.len() {
							p = self.cell[i][j].p[k];
							if !row.contains(&p) || !col.contains(&p) || !house.contains(&p) {
								self.cell[i][j].digit = p;
								self.update_p([i, j]);
								reset = true;
							}
						}
					}
				}
			}
		}
	}
}


//Wait for user input, just invokes Batch pause>nul.
fn pause() {
	let _ = Command::new("cmd.exe").arg("/c").arg("pause>nul").status();
}

//Main code containing backtracking logic.
fn main() {
	println!("How many times to run?: ");
	let mut final_avg: f64 = 0.0;

	let mut init = vec![vec![0]];

	let mut b = Board::new(init.len()); //The main board
	let mut b_stack: Vec<Board> = vec![]; //The stack of boards

	b.init(&init); //Initialize cells and area coordinates
	b.process_of_elimination(); //Possibilities initialization
	b_stack.push(b.clone()); //Push first unsolved board to stack.

	let mut reset: bool; //Whether or not to reset if the board isn't solved yet.

	b_stack.clear();
	
	let mut start;

	let mut duration;
	
	let mut input_line = String::new();
	io::stdin()
		.read_line(&mut input_line)
		.expect("Failed to read line");
	let num_of_loop: i32 = input_line.trim().parse().expect("Input not an integer");

	for l in 0..num_of_loop {

		b_stack.clear();
		println!("{}/{}", (l+1), num_of_loop);

		start = Instant::now();

		/* Example 9x9 and 16x16 boards to solve.
			//Easy difficulty
			let init = vec![
				vec![0,4,5,8,7,0,9,0,0],
				vec![0,0,0,9,0,0,0,0,0],
				vec![2,0,8,0,6,0,0,0,4],
				vec![0,1,0,2,0,0,4,0,0],
				vec![9,3,0,5,4,7,2,0,0],
				vec![0,0,4,6,9,0,7,0,3],
				vec![0,6,0,4,8,0,0,3,1],
				vec![3,8,0,7,0,2,6,0,9],
				vec![0,0,0,0,0,6,0,2,7]];

			//Master difficulty
			let init = vec![
				vec![0,0,0,0,0,0,0,0,0],
				vec![0,0,4,1,6,2,9,0,0],
				vec![2,0,0,0,3,0,0,7,0],
				vec![0,9,0,0,0,0,0,6,3],
				vec![0,0,0,0,0,0,0,0,0],
				vec![0,6,0,0,1,3,0,0,7],
				vec![9,0,6,0,0,5,0,0,0],
				vec![8,5,0,7,0,6,4,0,0],
				vec![0,7,0,0,0,0,0,2,0]];

			//Extreme difficulty
			let init = vec![
				vec![0,0,7,6,0,5,9,4,0],
				vec![0,0,0,0,0,0,0,0,6],
				vec![8,0,0,1,0,0,0,0,0],
				vec![0,0,0,0,0,0,2,0,0],
				vec![0,7,0,0,9,0,0,0,0],
				vec![0,0,9,0,0,4,5,3,0],
				vec![0,1,0,5,0,0,3,6,0],
				vec![0,0,0,0,0,6,0,0,7],
				vec![0,0,3,0,0,0,0,0,2]];
			
			//Beyond-hell difficulty
			let init = vec![
				vec![9,0,0,0,0,0,0,0,0],
				vec![0,0,0,0,1,0,0,6,0],
				vec![0,0,7,3,0,0,8,0,9],
				vec![0,1,0,4,2,0,0,0,0],
				vec![0,0,0,0,0,0,0,5,0],
				vec![6,5,3,0,0,0,0,0,0],
				vec![8,0,0,0,6,0,0,0,0],
				vec![0,0,0,0,0,9,0,4,0],
				vec![0,2,9,0,0,7,1,0,0]];

			//Rated 11.9 difficulty
			let init = vec![
				vec![1,2,0,3,0,0,0,0,0],
				vec![4,0,0,0,0,0,3,0,0],
				vec![0,0,3,0,5,0,0,0,0],
				vec![0,0,4,2,0,0,5,0,0],
				vec![0,0,0,0,8,0,0,0,9],
				vec![0,6,0,0,0,5,0,7,0],
				vec![0,0,1,5,0,0,2,0,0],
				vec![0,0,0,0,9,0,0,6,0],
				vec![0,0,0,0,0,7,0,0,8]];
			
			//Easy 16x16
			let init = vec![
				vec![0,4,0,16,2,0,10,14,0,6,0,0,5,15,3,8],
				vec![2,0,0,8,11,5,6,4,9,15,13,14,7,0,12,0],
				vec![0,7,0,12,3,0,1,16,10,4,0,0,0,0,0,0],
				vec![10,11,5,0,8,0,13,15,0,0,0,2,0,1,0,9],
				vec![14,16,0,10,0,0,9,1,0,12,2,0,8,13,0,0],
				vec![0,0,6,0,5,2,7,8,0,0,0,0,16,0,0,0],
				vec![0,13,8,4,15,14,0,0,0,0,3,0,9,0,0,1],
				vec![0,0,2,15,16,3,11,0,0,0,10,0,0,0,6,12],
				vec![4,6,3,0,0,0,0,0,7,5,11,0,13,0,0,10],
				vec![7,5,0,0,0,6,0,0,0,1,0,0,15,0,16,0],
				vec![12,1,0,11,0,13,0,7,2,14,15,10,0,3,0,4],
				vec![0,8,15,14,0,10,0,11,4,0,0,0,12,15,0,2],
				vec![8,0,11,13,0,0,4,2,0,10,5,1,3,0,9,0],
				vec![6,2,0,7,0,16,3,5,0,0,0,0,0,12,0,0],
				vec![3,0,12,0,13,1,14,0,0,0,0,6,0,0,0,0],
				vec![16,10,9,1,12,11,0,6,13,0,7,0,2,14,0,0]];

			//Extreme difficulty 16x16
			let init = vec![
				vec![13,0,0,0,0,10,0,0,6,0,0,0,0,11,0,0],
				vec![14,3,0,0,0,12,0,9,10,0,0,0,16,0,0,0],
				vec![0,9,0,0,0,0,1,0,0,0,15,13,8,0,0,12],
				vec![0,0,0,15,16,0,14,8,4,0,0,0,10,3,2,0],
				vec![0,0,13,8,15,0,3,0,1,2,6,0,0,16,0,0],
				vec![0,0,0,0,5,1,6,0,7,0,3,4,0,12,0,0],
				vec![6,0,0,11,0,2,0,0,0,13,0,15,0,0,0,0],
				vec![3,0,12,0,0,0,13,0,0,0,5,11,1,0,6,15],
				vec![0,1,0,13,6,3,0,0,0,0,0,0,2,0,16,0],
				vec![10,0,0,0,0,0,9,0,8,0,4,16,3,13,0,0],
				vec![0,11,2,0,7,8,0,16,0,10,13,0,0,0,15,4],
				vec![12,0,0,14,11,15,0,13,0,0,2,7,5,0,0,0],
				vec![7,8,0,0,9,0,0,2,0,11,0,10,12,0,0,0],
				vec![9,0,3,0,0,13,0,0,15,0,0,14,0,0,0,0],
				vec![0,10,0,1,0,11,0,3,0,0,0,0,0,8,7,0],
				vec![0,0,15,12,10,0,5,0,2,7,0,0,0,0,9,16]];

			//Hard 25x25
			let init = vec![
				vec![0,25,0,0,0,0,0,0,1,0,0,6,11,0,0,0,0,0,23,14,0,0,7,0,5],
				vec![0,22,19,23,12,16,17,20,21,0,0,10,0,1,0,0,8,0,0,0,9,13,24,11,0],
				vec![7,15,0,0,0,0,2,0,0,6,21,0,12,0,20,0,0,0,24,16,0,0,0,0,0],
				vec![6,0,10,9,5,0,0,15,4,0,22,0,0,0,8,0,11,0,0,0,2,0,16,0,0],
				vec![0,20,17,0,21,0,10,0,14,0,24,0,5,0,19,0,13,0,0,25,1,0,0,6,0],
				vec![1,0,21,0,0,0,0,0,7,23,0,20,0,8,0,0,17,0,6,0,0,5,9,0,13],
				vec![0,0,7,0,0,0,0,16,13,2,0,14,17,4,5,23,0,0,0,24,0,0,0,0,19],
				vec![0,5,0,15,17,0,0,25,0,12,0,0,16,0,21,0,18,4,11,8,0,0,0,7,0],
				vec![0,0,20,0,16,8,0,0,24,0,0,0,0,0,2,12,0,5,0,0,0,1,14,10,11],
				vec![24,0,0,6,4,18,20,0,0,5,0,0,7,0,3,0,14,16,0,0,0,0,17,23,0],
				vec![17,0,0,0,0,19,8,1,0,0,9,0,10,21,0,5,24,22,13,0,0,16,6,0,4],
				vec![0,0,9,12,0,15,0,11,0,0,0,13,18,25,22,0,0,10,0,21,0,0,0,8,0],
				vec![0,10,1,0,0,0,0,0,0,0,5,4,0,17,6,0,0,0,0,0,0,0,3,9,0],
				vec![0,4,0,0,0,3,0,14,0,0,23,8,2,20,0,0,0,12,0,6,0,25,5,0,0],
				vec![11,0,5,8,0,0,23,7,18,21,0,3,1,0,12,0,0,14,15,9,0,0,0,0,2],
				vec![0,12,14,0,0,0,0,5,6,0,15,0,19,0,0,17,0,0,22,11,24,20,0,0,21],
				vec![18,2,4,5,0,0,0,13,0,8,14,0,0,0,0,0,9,0,0,19,3,0,10,0,0],
				vec![0,19,0,0,0,4,14,24,25,0,20,0,3,0,0,10,0,23,0,0,6,7,0,16,0],
				vec![23,0,0,0,0,21,0,0,0,22,17,5,9,2,0,1,4,18,0,0,0,0,18,0,0],
				vec![25,0,13,17,0,0,16,0,20,0,0,24,0,22,0,2,21,0,0,0,0,0,23,0,14],
				vec![0,13,0,0,22,24,0,0,11,0,4,0,21,0,14,0,23,0,9,0,7,0,8,17,0],
				vec![0,0,25,0,24,0,0,0,16,0,6,0,0,0,11,0,5,21,0,0,20,14,15,0,12],
				vec![0,0,0,0,0,23,4,0,0,0,2,0,22,0,25,14,0,0,10,0,0,0,0,18,16],
				vec![0,17,15,10,18,0,0,0,5,0,0,12,0,7,0,0,19,24,3,1,13,6,11,2,0],
				vec![5,0,23,0,0,10,12,0,0,0,0,0,24,19,0,0,1,0,0,0,0,0,0,21,0]];

			//Easy 25x25
			let init = vec![
				vec![18,0,8,0,10,15,9,0,0,12,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0],
				vec![0,25,23,6,5,3,0,0,20,0,0,0,15,0,0,21,0,0,0,0,17,0,0,16,0],
				vec![0,0,0,0,0,8,0,0,6,11,5,19,4,0,0,0,1,3,0,0,0,20,21,18,0],
				vec![0,9,0,15,0,13,16,0,0,0,0,21,0,6,22,0,2,0,0,0,24,0,19,0,0],
				vec![13,0,0,0,1,17,18,4,5,0,2,11,0,24,10,0,0,0,7,6,22,0,0,23,0],
				vec![0,0,0,24,0,0,0,16,0,0,17,0,0,0,8,1,7,0,21,0,6,0,13,4,5],
				vec![11,5,0,0,0,0,0,23,13,0,0,0,0,0,9,0,8,19,10,0,0,0,0,22,15],
				vec![0,7,25,3,21,0,8,11,0,5,0,0,23,0,0,0,14,0,0,18,10,0,9,19,16],
				vec![6,0,0,0,4,14,21,0,22,0,7,25,0,0,0,0,0,2,0,17,0,23,0,0,24],
				vec![20,0,0,10,15,0,0,0,0,19,0,24,0,3,11,13,25,0,0,4,1,0,14,21,0],
				vec![4,0,15,19,0,7,0,0,25,22,0,0,0,0,0,0,0,0,17,0,12,24,20,0,0],
				vec![1,20,12,0,22,0,24,0,14,0,0,0,0,17,0,0,0,0,0,2,21,0,0,5,7],
				vec![25,0,0,0,7,0,1,0,11,21,0,9,0,0,0,0,0,0,5,0,23,3,2,0,0],
				vec![23,0,0,17,11,0,3,19,0,0,8,0,0,0,20,0,0,0,0,0,0,0,0,0,9],
				vec![0,0,16,0,2,23,0,0,0,0,14,0,13,0,0,11,22,20,15,0,0,8,0,10,1],
				vec![10,0,0,8,0,0,4,13,0,0,21,7,0,14,15,0,11,0,1,0,0,0,0,9,0],
				vec![0,6,24,20,0,16,0,10,0,25,22,0,0,0,0,0,15,0,23,7,0,0,0,0,0],
				vec![0,2,0,12,0,0,0,5,0,0,0,0,0,0,17,0,13,9,0,0,19,0,0,25,0],
				vec![15,19,17,21,0,0,12,0,9,0,0,1,25,13,4,0,0,24,0,0,16,6,0,11,22],
				vec![0,11,0,23,25,2,0,6,0,1,20,5,0,0,19,18,16,0,3,0,7,12,0,13,0],
				vec![0,0,22,0,20,9,0,18,17,23,0,0,0,0,7,0,0,0,0,0,0,0,6,3,0],
				vec![0,0,0,18,0,0,22,0,0,0,0,0,0,11,5,0,0,6,0,1,15,13,25,14,20],
				vec![24,0,14,5,0,0,0,1,16,0,0,15,0,0,0,0,0,0,13,0,9,19,0,7,11],
				vec![12,0,3,0,0,4,5,14,7,0,0,0,0,0,2,25,17,0,0,0,0,0,0,0,0],
				vec![0,0,0,0,6,0,10,0,0,0,0,12,0,0,3,16,0,0,0,21,0,22,0,0,4]];

			//Medium 25x25
			let init = vec![
				vec![23,2,0,0,0,5,14,0,0,11,0,18,0,20,13,0,0,0,0,0,0,24,19,4,0],
				vec![17,18,0,25,0,0,21,0,0,24,0,0,0,0,0,0,12,0,4,14,20,9,8,0,0],
				vec![7,0,0,4,0,0,18,0,1,23,0,0,19,0,0,0,9,2,0,5,0,0,0,0,0],
				vec![0,14,22,0,0,0,0,0,0,0,0,0,0,0,3,24,0,15,0,8,0,0,0,0,0],
				vec![24,0,8,6,0,4,20,25,3,0,16,12,15,9,0,1,10,0,0,0,0,0,0,0,0],
				vec![0,0,0,0,22,25,8,2,24,0,21,0,9,6,0,0,0,0,0,10,23,0,0,20,0],
				vec![0,24,13,1,0,0,17,0,14,0,3,0,20,0,0,0,7,0,23,0,10,0,16,0,0],
				vec![0,19,0,20,4,22,0,0,0,0,11,0,0,0,0,0,0,12,13,9,7,6,15,0,0],
				vec![9,0,0,0,0,13,16,0,0,12,23,0,0,0,10,11,0,0,0,1,0,18,4,21,3],
				vec![6,0,0,0,3,20,0,0,15,0,14,0,5,0,17,16,0,8,19,25,0,0,0,9,0],
				vec![0,0,19,0,0,21,0,0,17,20,0,0,0,23,0,0,0,1,0,4,0,10,0,2,15],
				vec![0,0,0,0,0,0,0,0,0,2,5,0,14,15,0,0,18,0,17,0,8,21,0,3,0],
				vec![0,0,0,12,7,0,15,8,0,9,0,1,24,0,0,6,0,22,0,0,0,20,0,0,0],
				vec![1,0,20,10,0,0,0,0,0,6,0,7,12,0,0,0,14,0,3,0,0,0,0,0,0],
				vec![4,0,0,0,14,0,0,11,18,0,0,0,0,0,0,0,0,16,0,0,19,7,25,1,0],
				vec![0,0,11,15,18,16,0,0,0,0,2,0,0,0,20,23,0,7,12,0,0,25,13,0,22],
				vec![0,8,16,0,0,0,0,0,5,22,0,0,3,13,0,0,0,0,10,0,0,17,0,18,0],
				vec![0,0,10,0,0,12,13,0,0,1,24,0,0,22,0,25,0,9,0,21,0,0,0,19,0],
				vec![3,0,0,0,12,0,0,23,0,0,6,19,10,0,15,2,5,13,0,0,0,0,0,0,8],
				vec![20,0,23,21,0,0,0,0,6,3,4,8,1,0,5,17,0,0,16,0,0,15,0,10,12],
				vec![0,17,21,0,0,0,0,0,0,0,12,0,0,1,11,0,16,0,0,0,4,0,7,8,0],
				vec![10,5,0,0,6,7,2,4,22,0,15,0,0,0,19,0,0,0,0,0,12,0,21,0,1],
				vec![16,0,0,18,0,0,3,0,11,14,0,25,23,0,24,4,17,0,20,0,9,2,10,6,0],
				vec![14,22,0,0,0,23,25,0,21,0,7,3,8,5,0,0,0,0,9,0,17,13,0,0,0],
				vec![12,0,0,7,1,8,0,0,16,0,0,17,0,14,2,5,0,0,0,0,24,0,0,25,0]];
		*/

		//The sudoku board to solve.
		let init = vec![
				vec![13,0,0,0,0,10,0,0,6,0,0,0,0,11,0,0],
				vec![14,3,0,0,0,12,0,9,10,0,0,0,16,0,0,0],
				vec![0,9,0,0,0,0,1,0,0,0,15,13,8,0,0,12],
				vec![0,0,0,15,16,0,14,8,4,0,0,0,10,3,2,0],
				vec![0,0,13,8,15,0,3,0,1,2,6,0,0,16,0,0],
				vec![0,0,0,0,5,1,6,0,7,0,3,4,0,12,0,0],
				vec![6,0,0,11,0,2,0,0,0,13,0,15,0,0,0,0],
				vec![3,0,12,0,0,0,13,0,0,0,5,11,1,0,6,15],
				vec![0,1,0,13,6,3,0,0,0,0,0,0,2,0,16,0],
				vec![10,0,0,0,0,0,9,0,8,0,4,16,3,13,0,0],
				vec![0,11,2,0,7,8,0,16,0,10,13,0,0,0,15,4],
				vec![12,0,0,14,11,15,0,13,0,0,2,7,5,0,0,0],
				vec![7,8,0,0,9,0,0,2,0,11,0,10,12,0,0,0],
				vec![9,0,3,0,0,13,0,0,15,0,0,14,0,0,0,0],
				vec![0,10,0,1,0,11,0,3,0,0,0,0,0,8,7,0],
				vec![0,0,15,12,10,0,5,0,2,7,0,0,0,0,9,16]];

		b = Board::new(init.len()); //The main board
		b_stack = vec![]; //The stack of boards

		b.init(&init); //Initialize cells and area coordinates
		b.update_all_p();
		b.process_of_elimination(); //Possibilities initialization
		
		b_stack.push(b.clone()); //Push first unsolved board to stack.

		reset = true; //Whether or not to reset if the board isn't solved yet.

		//Main back-tracking loop
		while reset == true && b.solved == false {
			reset = false;

			//Update temporary board
			b = b_stack.last_mut().unwrap().clone();

			//Iterate through cells
			'outer: for i in 0..b.bsize {
				for j in 0..b.bsize {

					//Ensure cell is a 0
					if b.cell[i][j].digit == 0 {

						//Ensure cell has possibilities
						if b.cell[i][j].p.len() > 0 {

							//Set cell to first possibility and update the last-modified cell data.
							b.cell[i][j].digit = b.cell[i][j].p[0];
							b.last_modified = [i, j, b.cell[i][j].p[0] as usize];
							b.update_p([i, j]);
							//Update all possibilities and check for lone-possibilities in rows/cols/houses.
							b.process_of_elimination();
							
							//Push board to stack
							b_stack.push(b.clone());
							b = b_stack.last_mut().unwrap().clone();

						//No possibilities mean the current board state is impossible to solve.
						} else {

							//Pop top of stack.
							b_stack.pop();

							//Revert the last-modified cell to a 0 and update its p_limit list.
							b_stack.last_mut().unwrap().cell[b.last_modified[0]][b.last_modified[1]].p_limit.push(b.last_modified[2] as u16);
							b_stack.last_mut().unwrap().cell[b.last_modified[0]][b.last_modified[1]].digit = 0;
							
							b_stack.last_mut().unwrap().update_all_p();
							//b_stack.last_mut().unwrap().update_p([b.last_modified[0], b.last_modified[1]]);
							
							//Update all possibilities and check for lone-possibilities in rows/cols/houses.
							b_stack.last_mut().unwrap().process_of_elimination();
							
							reset = true;
							break 'outer;
						}
					}
				}
			}
		}
		
		duration = start.elapsed();

		final_avg += duration.as_secs_f64() * 1000.0;
	}
	//Show the solved board
	b_stack.last_mut().unwrap().show();
	//b.show();
	final_avg = final_avg/(num_of_loop as f64);
	println!("Average time to solve: {}ms", final_avg);
	pause();



}