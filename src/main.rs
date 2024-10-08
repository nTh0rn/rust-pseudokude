use std::process::Command;
use std::time::{Instant};

use colored::Colorize;
extern crate winapi;

use std::ptr;
use std::io::{self, Write};
use winapi::um::consoleapi::GetConsoleMode;
use winapi::um::consoleapi::SetConsoleMode;
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

//Used to allow color printing
fn enable_virtual_terminal_processing() -> io::Result<()> {
    unsafe {
        // Get the handle to the standard output (console)
        let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if stdout_handle == ptr::null_mut() {
            return Err(io::Error::last_os_error());
        }

        // Get the current console mode
        let mut mode: u32 = 0;
        if GetConsoleMode(stdout_handle, &mut mode) == 0 {
            return Err(io::Error::last_os_error());
        }

        // Enable ENABLE_VIRTUAL_TERMINAL_PROCESSING flag
        mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        if SetConsoleMode(stdout_handle, mode) == 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

//Individual cell holding all aoe information.
#[derive(Clone)]
pub struct Cell {
	digit: u16, //Digit of cell
	row: Vec<[usize; 2]>, //Coordinates of cell's row
	col: Vec<[usize; 2]>, //Coordinates of cell's col
	house: Vec<[usize; 2]>, //Coordinates of cell's house
	aoe: Vec<[usize; 2]>, //Coordinates of cell's aoe
	cand: Vec<u16>, //candidates of current cell
	cand_limit: Vec<u16>, //Restrictions on candidates
	was_empty: bool,
	known: bool,
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
			cand: vec![],
			cand_limit: vec![],
			was_empty: false,
			known: false,
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

	//Initialize values of board from given input, where init is the sudoku board.
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

				if self.cell[i][j].digit == 0 {
					self.cell[i][j].was_empty = true;
				}

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

		output.push_str(&format!("{} - Original puzzle\n", "White\t"));
		output.push_str(&format!("{} - Solved via candidate analysis\n", "Red\t".red()));
		output.push_str(&format!("{} - Current backtracking cell\n", "Blue\t".cyan()));
		output.push_str(&format!("{} - Solved via backtracking, other candidates still exist\n", "Yellow\t".yellow()));
		output.push_str(&format!("{} - Solved via backtracking, no more candidates exist\n\n", "Green\t".green()));

		//Main loop
		for i in 0..self.bsize {
			for j in 0..self.bsize {

				//Ensure enough white-space before digit.
				if self.cell[i][j].digit != 0 {
					for _ in 0..space_per_digit-(((self.cell[i][j].digit).checked_ilog10().unwrap_or(0)+2) as usize) {
						output.push_str(" ");
					}

					//Color cell depending on if it was solved via backtracking or candidate elimination.
					if self.cell[i][j].known == true {
						output.push_str(&format!("{}", self.cell[i][j].digit.to_string().green()));
					} else if self.cell[i][j].was_empty == true && (i*9+j) < (self.last_modified[0]*9+self.last_modified[1]) {
						output.push_str(&format!("{}", self.cell[i][j].digit.to_string().yellow()));
					} else if i == self.last_modified[0] && j == self.last_modified[1] {
						output.push_str(&format!("{}", self.cell[i][j].digit.to_string().cyan()));
					} else if self.cell[i][j].was_empty == true {
						output.push_str(&format!("{}", self.cell[i][j].digit.to_string().red()));
					} else {
						output.push_str(&format!("{}", self.cell[i][j].digit.to_string()));
					}
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

	//Returns a vector of digits OR candidates from a vector of coordinates
	fn coords_to_digits(&self, area: &Vec<[usize; 2]>, return_cand: bool) -> Vec<u16> {
		let mut output: Vec<u16> = vec![];

		//Iterate through area
		for each in area {

			//Whether to return area's digits or all of area's candidates
			if return_cand {
				if self.cell[each[0]][each[1]].digit == 0 {
					for each in &self.cell[each[0]][each[1]].cand {
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

	//Updates the candidates of all cells, restricted by cand_limit.
	fn update_cand(&mut self, coord: [usize; 2]) {
		let mut aoe: Vec<u16>; //Current cell's house
		let mut cand_len: u16;
		
		for each in &self.cell[coord[0]][coord[1]].aoe.clone() {
			if self.cell[each[0]][each[1]].digit == 0 {

				cand_len = 0;
				self.cell[each[0]][each[1]].cand.clear();

				//Assign all candidates, restricted by limit and cand_limit.
				aoe = self.coords_to_digits(&self.cell[each[0]][each[1]].aoe, false);

				for k in 1..(self.bsize+1) {
					if !aoe.contains(&(k as u16)) && !self.cell[each[0]][each[1]].cand_limit.contains(&(k as u16)) {
						self.cell[each[0]][each[1]].cand.push(k as u16);
						cand_len += 1;
					}
				}

				//If there is only 1 candidate, set it as the digit and restart.
				if cand_len == 1 {
					self.cell[each[0]][each[1]].digit = self.cell[each[0]][each[1]].cand[0];
					self.update_cand([each[0], each[1]]);
				}
			}
		}
	}

	//Updates the candidates of all cells, restricted by cand_limit.
	fn update_all_cand(&mut self) {
		let mut aoe: Vec<u16>; //Current cell's house
		
		//Iterate through cells
		for i in 0..self.bsize {
			for j in 0..self.bsize {
				//Ensure cell is a 0
				if self.cell[i][j].digit == 0 {

					self.cell[i][j].cand.clear();

					aoe = self.coords_to_digits(&self.cell[i][j].aoe, false);

					//Assign all candidates, restricted by limit and cand_limit.
					for k in 1..(self.bsize+1) {
						if !aoe.contains(&(k as u16)) && !self.cell[i][j].cand_limit.contains(&(k as u16)) {
							self.cell[i][j].cand.push(k as u16);
						}
					}
				}
			}
		}
		
	}

	
	//Checks for cells that have candidates that are unique to one of its areas
	fn process_of_elimination(&mut self) {
		let mut c: u16; //Current cell's candidate
		let mut row: Vec<u16>; //Current cell's row
		let mut col: Vec<u16>; //Current cell's col
		let mut house: Vec<u16>; //Current cell's house
		let mut reset: bool = true; //Whether or not to keep searching

		//Show board during calculation. (SUPER SLOWDOWN)
		//self.show();
		//Start search
		while reset {
			self.solved = true;
			reset = false;

			//Iterate through all cells
			for i in 0..self.bsize {
				for j in 0..self.bsize {

					//Ensure cell is a 0
					if self.cell[i][j].digit == 0 {
						self.solved = false;

						//Establish candidates in each area
						row = self.coords_to_digits(&self.cell[i][j].row, true);
						col = self.coords_to_digits(&self.cell[i][j].col, true);
						house = self.coords_to_digits(&self.cell[i][j].house, true);

						//If areas do not contain candidate, then set cell to candidate.
						for k in 0..self.cell[i][j].cand.len() {
							c = self.cell[i][j].cand[k];
							if !row.contains(&c) || !col.contains(&c) || !house.contains(&c) {
								self.cell[i][j].digit = c;
								self.update_cand([i, j]);
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
	if let Err(e) = enable_virtual_terminal_processing() {
        writeln!(io::stderr(), "Error enabling virtual terminal processing: {}", e).unwrap();
    }

	let init = vec![
				vec![0,0,0,0,0,0,0,0,0],
				vec![4,0,0,0,0,0,3,0,0],
				vec![0,0,0,0,5,0,0,0,0],
				vec![0,0,4,2,0,0,5,0,0],
				vec![0,0,0,0,8,0,0,0,9],
				vec![0,6,0,0,0,5,0,7,0],
				vec![0,0,1,5,0,0,2,0,0],
				vec![0,0,0,0,9,0,0,6,0],
				vec![0,0,0,0,0,7,0,0,8]];

	let mut b = Board::new(init.len()); //The main board
	let mut b_stack: Vec<Board> = vec![]; //The stack of boards

	b.init(&init); //Initialize cells and area coordinates
	b.update_all_cand(); //Update the candidates for all cells
	b.process_of_elimination(); //candidates initialization
	b_stack.push(b.clone()); //Push first unsolved board to stack.
	b_stack.push(b.clone());
	//Main back-tracking loop
	while b.solved == false {

		//Update temporary board
		b = b_stack.last_mut().unwrap().clone();

		//Iterate through cells
		'outer: for i in 0..b.bsize {
			for j in 0..b.bsize {

				//Ensure cell is a 0
				if b.cell[i][j].digit == 0 {
					
					//Ensure cell has candidates
					if b.cell[i][j].cand.len() > 0 {
						
						//Set cell to first candidate and update the last-modified cell data.
						b.cell[i][j].digit = b.cell[i][j].cand[0];
						if b.cell[i][j].cand.len() == 1 {
							b.cell[i][j].known = true;
						}
						b.last_modified = [i, j, b.cell[i][j].cand[0] as usize];

						//Update candidates and check for area candidate eliminations.
						b.update_cand([i, j]);
						b.process_of_elimination();
						
						//Push board to stack
						b_stack.push(b.clone());

						b.show();
						
					//No candidates mean the current board state is impossible to solve.
					} else {

						//Only encountered if the board is unsolvable, which means it was entered incorrectly.
						if b_stack.len() == 1 {
							panic!("ERROR - Sudoku board not entered correctly.");
						}

						//Pop top of stack.
						b_stack.pop();

						//Revert the last-modified cell to a 0 and update its cand_limit list.
						b_stack.last_mut().unwrap().cell[b.last_modified[0]][b.last_modified[1]].cand_limit.push(b.last_modified[2] as u16);
						b_stack.last_mut().unwrap().cell[b.last_modified[0]][b.last_modified[1]].digit = 0;
						
						//Update candidates and check for area candidate eliminations.
						b_stack.last_mut().unwrap().update_all_cand();
						b_stack.last_mut().unwrap().process_of_elimination();
						//Restart search
						break 'outer;
					}
				}
			}
		}
	}
	
	//Show the solved board
	b_stack.last_mut().unwrap().show();

	pause();

}
