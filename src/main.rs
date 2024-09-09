use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

//Individual cell holding all aoe information.
#[derive(Clone)]
pub struct Cell {
	digit: u16, //Digit of cell
	row: Vec<[usize; 2]>, //Coordinates of cell's row
	col: Vec<[usize; 2]>, //Coordinates of cell's col
	house: Vec<[usize; 2]>, //Coordinates of cell's house
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
			p: vec![],
			p_limit: vec![],
		}
	}

	//Set current digit to first possibility
	fn set_p(&mut self) {
		self.digit = self.p[0];
	}
}

//Entire board containing size information and 2d vector of cells.
#[derive(Clone)]
pub struct Board {
	bsize: usize, //Board side-length
	hsize: usize, //House side-length
	last_modified: [usize; 3], //Information about the last-modified cell.
	cell: Vec<Vec<Cell>>, //2D vector containing all cells
}
impl Board {

	//Constructor
	pub fn new(bsize: usize) -> Self {
		Self {
			bsize,
			hsize: (bsize as f64).sqrt() as usize,
			last_modified: [0,0,0],
			cell: vec![],
		}
	}

	//Initialize values of board from given input
	fn init(&mut self, init: &Vec<Vec<u16>>) {
		let mut i: usize = 0;
		let mut j: usize = 0;
		let mut k: usize = 0;
		let mut l: usize;

		let mut hx: usize;
		let mut hy: usize;

		//Initialize cells with their digits
		while i < self.bsize {
			self.cell.push(Vec::new());
			while j < self.bsize {
				self.cell[i].push(Cell::new());
				self.cell[i][j].digit = init[i][j];
				j = j + 1;
			}
			j = 0;
			i = i + 1;
		}
		i=0;
		j=0;

		//Initialize row and column coordinates
		while i < self.bsize {
			while j < self.bsize {
				while k < self.bsize {
					if k != j {
						self.cell[i][j].row.push([i,k]);
					}
					if k != i {
						self.cell[i][j].col.push([k,j]);
					}
					k = k + 1;
				}
				k = 0;
				j = j + 1;
			}
			j = 0;
			i = i + 1;
		}

		i = 0;
		j = 0;

		//Initialize house coordinates
		while i < self.bsize {
			while j < self.bsize {

				//Calculate top-left coordinate of cell's house.
				hy = (((i/self.hsize) as f64).floor() as usize)*(self.hsize);
				hx = (((j/self.hsize) as f64).floor() as usize)*(self.hsize);

				k = 0;
				l = 0;

				//Iterate from top-left coordinate of house and add to cell's house.
				while k < self.hsize {
					while l < self.hsize {
						if i != (k+hy) || j != (l+hx) {
							self.cell[i][j].house.push([(k+hy),(l+hx)]);
						}
						
						l = l + 1;
					}
					l = 0;
					k = k + 1;
				}
				j = j + 1;
			}
			j = 0;
			i = i + 1;
		}
	}

	//Show current state of board
	fn show(&self) {
		let mut i = 0;
		let mut j = 0;
		let mut k = 0;
		let mut l = 0;

        let mut output = String::from("");

		let mut space_per_digit = 2;

		if self.hsize > 3 {
			space_per_digit = 3;
		}
        
		print!("\x1B[2J\x1B[1;1H");
        output.push_str("\n");

		while i < self.bsize {
			while j < self.bsize {
				if self.cell[i][j].digit < 10 && self.hsize > 3 {
					output.push_str(" ");
				}
				if self.cell[i][j].digit != 0 {
                    output.push_str(&self.cell[i][j].digit.to_string());
				} else {
                    output.push_str(" ");
				}
				if (j+1) % self.hsize == 0 && (j+1) != (self.hsize*self.hsize) {
                    output.push_str("|");
				} else {
                    output.push_str(" ");
				}
				j = j + 1;
			}
            output.push_str("\n");
			if (i+1) % self.hsize == 0 && (i+1) != (self.hsize*self.hsize) {
				while k < self.hsize {
					while l < (self.hsize*space_per_digit)-1 {
                        output.push_str("-");
						l = l + 1;
					}
					if k != self.hsize-1 {
                        output.push_str("+");
					}
					l=0;
					k = k + 1;
				}
				k = 0;
                output.push_str("\n");
			}
			j = 0;
			i = i + 1;
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
				for each in &self.cell[each[0]][each[1]].p {
					output.push(*each);
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
	fn update_all_p(&mut self) {
		let mut i: usize;
		let mut j: usize;
		let mut k: usize;

		let mut reset: bool = true;

		let mut limit: Vec<u16> = vec![];

		while reset == true {
			reset = false;

			i = 0;
			j = 0;
			k = 1;

			'outer: while i < self.bsize {
				while j < self.bsize {
					if self.cell[i][j].digit == 0 {

						limit.clear();
						self.cell[i][j].p.clear();

						//Update limits of current cell
						limit.append(&mut self.coords_to_digits(&self.cell[i][j].row, false));
						limit.append(&mut self.coords_to_digits(&self.cell[i][j].col, false));
						limit.append(&mut self.coords_to_digits(&self.cell[i][j].house, false));

						//Assign all possibilities, restricted by limit and p_limit.
						while k < (self.bsize+1) {
							if !limit.contains(&(k as u16)) && !self.cell[i][j].p_limit.contains(&(k as u16)) {
								self.cell[i][j].p.push(k as u16);
							}
							k = k + 1;
						}

						//If there is only 1 possibility, set it as the digit and restart.
						if self.cell[i][j].p.len() == 1 {
							self.cell[i][j].digit = self.cell[i][j].p[0];
							reset = true;
							break 'outer;
						}
						k=1;
					}
					j = j + 1;
				}
				j = 0;
				i = i + 1;
			}
		}
	}


	//Checks for lone-possibility's and updates all possibilities.
	fn process_of_elimination(&mut self) {
		let mut i: usize;
		let mut j: usize;
		let mut k: usize;

		let mut p: u16; //Current cell's possibility
		let mut row: Vec<u16>; //Current cell's row
		let mut col: Vec<u16>; //Current cell's col
		let mut house: Vec<u16>; //Current cell's house

		let mut reset: bool = true; //Whether or not to end search
		
		//Main loop
		while reset == true {
			reset = false;

			//Update all possibilities and fill-in lone-possibilities
			self.update_all_p();

			i = 0;
			j = 0;
			k = 0;

			//Show board during calculation. (SUPER SLOWDOWN)
			self.show();

			//Iterate through cels
			'outer: while i < self.bsize {
				while j < self.bsize {

					//Ensure cell is a 0
					if self.cell[i][j].digit == 0 {

						//Save all possibility's in area of cell
						row = self.coords_to_digits(&self.cell[i][j].row, true);
						col = self.coords_to_digits(&self.cell[i][j].col, true);
						house = self.coords_to_digits(&self.cell[i][j].house, true);

						//If area's do not contain a possibility, then set digit to possibility.
						while k < self.cell[i][j].p.len() {
							p = self.cell[i][j].p[k];
							if !row.contains(&p) || !col.contains(&p) || !house.contains(&p) {
								self.cell[i][j].digit = p;
                                reset = true;
								break 'outer;
							}
							k = k + 1;
						}
						k = 0;
					}
					j = j + 1;
				}
				j = 0;
				i = i + 1;
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
	let mut i: usize;
	let mut j: usize;

	println!("Press enter to begin.");
	pause();

	//Current epoch time before solve
	let mut since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
	let start_ms = since_the_epoch.as_secs() * 1000 +
		since_the_epoch.subsec_nanos() as u64 / 1_000_000;

	/*
		//Easy difficulty
		let init = vec![
			vec![8,0,1,9,0,0,0,4,0],
			vec![0,4,0,8,5,1,0,2,0],
			vec![0,5,6,0,7,0,0,9,1],
			vec![0,3,0,0,0,5,0,7,0],
			vec![0,0,0,0,3,0,1,0,0],
			vec![7,6,0,2,0,0,5,0,8],
			vec![4,2,0,0,6,8,9,1,0],
			vec![0,0,0,1,0,0,6,8,7]];
			vec![0,0,3,1,0,0,2,5,0],

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

		Rated 11.9 difficulty
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
	*/

	//The sudoku board to solve.
	//Beyond-hell difficulty
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


	let mut b = Board::new(init.len()); //The main board
	let mut b_stack: Vec<Board> = vec![]; //The stack of boards

	b.init(&init); //Initialize cells and area coordinates
	b.process_of_elimination(); //Possibilities initialization
	b_stack.push(b.clone()); //Push first unsolved board to stack.

	let mut t: usize = &b_stack.len()-1; //Top of the stack of boards
	let mut reset: bool = true; //Whether or not to reset if the board isn't solved yet.

	//Main back-tracking loop
	while reset == true {
		reset = false;

		//Update temporary board
		b = b_stack[t].clone();

		i = 0;
		j = 0;

		//Iterate through cells
		'outer: while i < b.bsize {
			while j < b.bsize {

				//Ensure cell is a 0
				if b.cell[i][j].digit == 0 {

					//Ensure cell has possibilities
					if b.cell[i][j].p.len() > 0 {

						//Set cell to first possibility and update the last-modified cell data.
						b.cell[i][j].set_p();
						b.last_modified = [i, j, b.cell[i][j].p[0] as usize];
						
						//Update all possibilities and check for lone-possibilities in rows/cols/houses.
						b.process_of_elimination();
						
						//Push board to stack
						b_stack.push(b.clone());
						t = b_stack.len()-1;
						
						reset = true;
						break 'outer;

					//No possibilities mean the current board state is impossible to solve.
					} else {

                        //Only encountered if the board is unsolvable, which means it was entered incorrectly.
                        if b_stack.len() == 1 {
							panic!("ERROR - Sudoku board not entered correctly.");
						}

						//Pop top of stack.
						b_stack.remove(t);
                        t = b_stack.len()-1;

						//Revert the last-modified cell to a 0 and update its p_limit list.
						b_stack[t].cell[b.last_modified[0]][b.last_modified[1]].p_limit.push(b.last_modified[2] as u16);
						b_stack[t].cell[b.last_modified[0]][b.last_modified[1]].digit = 0;

						//Update all possibilities and check for lone-possibilities in rows/cols/houses.
						b_stack[t].process_of_elimination();
						
						reset = true;
						break 'outer;
					}
				}
				j = j + 1;
			}
			j = 0;
			i = i + 1;
		}
	}

	//Current epoch time after solve
	since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
	let final_ms = (since_the_epoch.as_secs() * 1000 +
		since_the_epoch.subsec_nanos() as u64 / 1_000_000) - start_ms;

	//Show the solved board
	b_stack[t].show();

	println!("\nSolved in {} milliseconds.", final_ms);
	pause();



}