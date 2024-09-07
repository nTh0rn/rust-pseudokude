use std::process::Command;
use std::io::{stdin, stdout, Read, Write};

fn pause() {
	let mut stdout = stdout();
	stdout.write(b"Press Enter to continue...").unwrap();
	stdout.flush().unwrap();
	stdin().read(&mut [0]).unwrap();
}


pub struct Cell {
	y: u16,
	x: u16,
	digit: u16,
	aoe: Vec<[u16; 2]>,
	row: Vec<[u16; 2]>,
	col: Vec<[u16; 2]>,
	house: Vec<[u16; 2]>,
	p: Vec<u16>,
	p_limit: Vec<u16>,
}

impl Cell {
	pub fn new(y: u16, x:u16) -> Self {
		Self {
			y,
			x,
			digit: 0,
			aoe: vec![],
			row: vec![],
			col: vec![],
			house: vec![],
			p: vec![],
			p_limit: vec![],
		}
	}
}


pub struct Board {
	bsize: usize, //Board side-length
	hsize: usize, //House side-length
	cell: Vec<Vec<Cell>>, //The cell within the board
}
impl Board {
	pub fn new(bsize: usize) -> Self {
		Self {
			bsize,
			hsize: (bsize as f64).sqrt() as usize,
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
						self.cell[i][j].row.push([i as u16,k as u16]);
					}
					if k != i {
						self.cell[i][j].col.push([k as u16,j as u16]);
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
							self.cell[i][j].house.push([(k+hy) as u16,(l+hx) as u16]);
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


}




fn main() {
	let mut init = vec![
		vec![0,0,0,5,0,0,0,3,1],
        vec![0,0,0,0,4,2,0,7,8],
        vec![6,0,0,1,0,7,2,0,5],
        vec![1,0,0,6,8,5,4,9,0],
        vec![7,4,0,2,0,0,0,0,0],
        vec![0,8,0,7,0,0,1,2,0],
        vec![0,1,0,9,0,6,0,5,0],
        vec![3,0,0,0,5,0,7,0,2],
        vec![2,0,5,0,7,0,8,0,9]];

	let mut board = Board::new(init.len());
	board.init(&mut init);
	board.show();

	for item in &board.cell[3][3].house {
		print!("{} - ", board.cell[item[0] as usize][item[1] as usize].digit);
	}



	
	//board.cell.push(vec![Cell{y: 3, x: 3}]);

	//println!("{}", board.cell[1][0].x);
}