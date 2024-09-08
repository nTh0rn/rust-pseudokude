use std::process::Command;
use std::io::{stdin, stdout, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Cell {
	y: u16,
	x: u16,
	digit: u16,
	row: Vec<[usize; 2]>,
	col: Vec<[usize; 2]>,
	house: Vec<[usize; 2]>,
	limit: Vec<u16>,
	p: Vec<u16>,
	p_limit: Vec<u16>,
}

impl Cell {
	pub fn new(y: u16, x:u16) -> Self {
		Self {
			y,
			x,
			digit: 0,
			row: vec![],
			col: vec![],
			house: vec![],
			limit: vec![],
			p: vec![],
			p_limit: vec![],
		}
	}

	fn set_p(&mut self) {
		self.digit = self.p[0];
	}
}

#[derive(Clone)]
pub struct Board {
	bsize: usize, //Board side-length
	hsize: usize, //House side-length
	last_modified: [usize; 3],
	cell: Vec<Vec<Cell>>,
}
impl Board {
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
		let mut l: usize = 0;

		let mut hx: usize = 0;
		let mut hy: usize = 0;

		//Initialize cells with their digits
		while i < self.bsize {
			self.cell.push(Vec::new());
			while j < self.bsize {
				self.cell[i].push(Cell::new(i as u16, j as u16));
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
				hy = ((((i/self.hsize) as f64).floor() as usize)*(self.hsize));
				hx = ((((j/self.hsize) as f64).floor() as usize)*(self.hsize));
				k = 0;
				l = 0;
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
		println!();
		while i < self.bsize {
			while j < self.bsize {
				if self.cell[i][j].digit != 0 {
					print!("{} ", self.cell[i][j].digit);
				} else {
					print!(". ");
				}
				j = j + 1;
			}
			println!("");
			j = 0;
			i = i + 1;
		}
	}

	//Returns a vector of digits OR possibilities from a vector of coordinates
	fn coords_to_digits(&self, list: &Vec<[usize; 2]>, return_p: bool) -> Vec<u16> {
		let mut output: Vec<u16> = vec![];
		for each in list {
			if(return_p == false) {
				if self.cell[each[0]][each[1]].digit != 0 {
					output.push(self.cell[each[0]][each[1]].digit);
				}
			} else {
				for each in &self.cell[each[0]][each[1]].p {
					output.push(*each);
				}
			}
		}
		return output;
	}

	//Updates the possibilities of all cells, restricted by p_limit.
	fn update_all_p(&mut self) {
		let mut i: usize = 0;
		let mut j: usize = 0;
		let mut k: usize = 1;

		let mut limit: Vec<u16> = vec![];

		while i < self.bsize {
			while j < self.bsize {
				if self.cell[i][j].digit == 0 {
					limit.clear();
					limit.append(&mut self.coords_to_digits(&self.cell[i][j].row, false));
					limit.append(&mut self.coords_to_digits(&self.cell[i][j].col, false));
					limit.append(&mut self.coords_to_digits(&self.cell[i][j].house, false));
					self.cell[i][j].p.clear();
					while k < (self.bsize+1) {
						if !limit.contains(&(k as u16)) && !self.cell[i][j].p_limit.contains(&(k as u16)) {
							self.cell[i][j].p.push(k as u16);
						}
						k = k + 1;
					}
					k=1;
				}
				j = j + 1;
			}
			j = 0;
			i = i + 1;
		}
	}


	//
	fn process_of_elimination(&mut self) {
		let mut i: usize = 0;
		let mut j: usize = 0;
		let mut k: usize = 0;
		let mut p: u16 = 0;
		let mut row: Vec<u16> = vec![];
		let mut col: Vec<u16> = vec![];
		let mut house: Vec<u16> = vec![];
		let mut reset: bool = true;
		
		while reset == true {
			reset = false;
			self.update_all_p();
			i = 0;
			j = 0;
			k = 0;
			//print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
			//self.show();
			while i < self.bsize {
				while j < self.bsize {
					if self.cell[i][j].digit == 0 {
						row = self.coords_to_digits(&self.cell[i][j].row, true);
						col = self.coords_to_digits(&self.cell[i][j].col, true);
						house = self.coords_to_digits(&self.cell[i][j].house, true);
						while k < self.cell[i][j].p.len() {
							p = self.cell[i][j].p[k];
							if !row.contains(&p) || !col.contains(&p) || !house.contains(&p) {
								self.cell[i][j].digit = p;
								reset = true;
							}
							if reset == true {
								break;
							}
							k = k + 1;
						}
						if reset == true {
							break;
						}
						k = 0;
					}
					j = j + 1;
				}
				if reset == true {
					break;
				}
				j = 0;
				i = i + 1;
			}
		}
	}
}

fn main() {
	let mut i: usize = 0;
	let mut j: usize = 0;
	println!("Press enter to begin.");
	let _ = Command::new("cmd.exe").arg("/c").arg("pause>nul").status();
	let mut start = SystemTime::now();
    let mut since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
	let start_ms = since_the_epoch.as_secs() * 1000 +
		since_the_epoch.subsec_nanos() as u64 / 1_000_000;

	/*
	let mut init = vec![
		vec![0,0,7,6,0,5,9,4,0],
        vec![0,0,0,0,0,0,0,0,6],
        vec![8,0,0,1,0,0,0,0,0],
        vec![0,0,0,0,0,0,2,0,0],
        vec![0,7,0,0,9,0,0,0,0],
        vec![0,0,9,0,0,4,5,3,0],
        vec![0,1,0,5,0,0,3,6,0],
        vec![0,0,0,0,0,6,0,0,7],
        vec![0,0,3,0,0,0,0,0,2]];
	*/

	let mut init = vec![
		vec![9,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,1,0,0,6,0],
        vec![0,0,7,3,0,0,8,0,9],
        vec![0,1,0,4,2,0,0,0,0],
        vec![0,0,0,0,0,0,0,5,0],
        vec![6,5,3,0,0,0,0,0,0],
        vec![8,0,0,0,6,0,0,0,0],
        vec![0,0,0,0,0,9,0,4,0],
        vec![0,2,9,0,0,7,1,0,0]];

	let mut board = Board::new(init.len());
	let mut board_stack: Vec<Board> = vec![];

	board.init(&mut init);
	board.update_all_p();

	board_stack.push(board.clone());
	let mut t: usize = &board_stack.len()-1;
	let mut b: Board = board_stack[t].clone();
	let mut reset: bool = true;
	let mut lm = [0,0,0];

	while reset == true {
		reset = false;
		b = board_stack[t].clone();
		i = 0;
		j = 0;
		'outer: while i < b.bsize {
			while j < b.bsize {
				if b.cell[i][j].digit == 0 {
					if b.cell[i][j].p.len() > 0 {
						b.cell[i][j].set_p();
						b.last_modified = [i, j, b.cell[i][j].p[0] as usize];
						b.process_of_elimination();
						board_stack.push(b.clone());
						t = board_stack.len()-1;
						reset = true;
						break 'outer;
					} else {
						lm = board_stack[t].last_modified.clone();
						board_stack.remove(t);
						t = board_stack.len()-1;
						board_stack[t].cell[lm[0]][lm[1]].p_limit.push(lm[2] as u16);
						board_stack[t].cell[lm[0]][lm[1]].digit = 0;
						board_stack[t].process_of_elimination();
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
	start = SystemTime::now();
	since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
	let end_ms = (since_the_epoch.as_secs() * 1000 +
		since_the_epoch.subsec_nanos() as u64 / 1_000_000) - start_ms;
	board_stack[t].show();
	println!("Completed in {} milliseconds.", end_ms);
	let _ = Command::new("cmd.exe").arg("/c").arg("pause>nul").status();



}