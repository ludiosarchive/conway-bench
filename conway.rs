extern crate rand;
use std::io::stdio;
use std::str;

static COLS: uint = 177;
static ROWS: uint = 60;

type Grid = [[u8, ..COLS+1], ..ROWS+1];
type GhostGrid = [[u8, ..COLS+1+2], ..ROWS+1+2];

/**
 * A virtual grid that includes wrapped edges, so that we don't have to
 * do funky modulo arithmetic.
 */
fn update_ghost(grid: &Grid, ghost_grid: &mut GhostGrid) -> () {
	/* Copy bottom of grid to top of ghost_grid */
	for n in range(0u, COLS) {
		ghost_grid[0][n+1] = grid[ROWS-1][n];
	}

	/* Copy top of grid to bottom of ghost_grid */
	for n in range(0u, COLS) {
		ghost_grid[ROWS+2-1][n+1] = grid[0][n];
	}

	/* Copy the rest of grid to ghost_grid */
	for y in range(0u, ROWS) {
		for x in range(0u, COLS) {
			ghost_grid[y+1][x+1] = grid[y][x];
		}
	}

	/* Wrap ghost_grid left and right columns */
	for y in range(0u, ROWS+2) {
		ghost_grid[y][0] = ghost_grid[y][COLS+2-2];
		ghost_grid[y][COLS+2-2] = ghost_grid[y][1];
	}
}

fn count_neighbors(x: uint, y: uint, ghost_grid: &GhostGrid) -> u8 {
	ghost_grid[(y-1)+1][(x-1)+1] + ghost_grid[(y-1)+1][(x)+1] + ghost_grid[(y-1)+1][(x+1)+1] +
	ghost_grid[(y)+1  ][(x-1)+1]                              + ghost_grid[(y)+1  ][(x+1)+1] +
	ghost_grid[(y+1)+1][(x-1)+1] + ghost_grid[(y+1)+1][(x)+1] + ghost_grid[(y+1)+1][(x+1)+1]
}

fn pretty_print(grid: &Grid) -> () {
	let mut out: [u8, ..((ROWS+1)*COLS)] = [0, ..((ROWS+1)*COLS)];
	let mut out_i: uint = 0;
	for y in range(0u, ROWS) {
		for x in range(0u, COLS) {
			if grid[y][x] == 0 {
				out[out_i] = ' ' as u8;
				out_i += 1;
			} else {
				out[out_i] = '#' as u8;
				out_i += 1;
			}
		}
		out[out_i] = '\n' as u8;
		out_i += 1;
	}
	unsafe {
		let s = str::raw::from_utf8(out);
		stdio::print(s);
	}
}

fn next_gen(grid: &mut Grid, ghost_grid: &mut GhostGrid) -> () {
	for y in range(0u, ROWS) {
		for x in range(0u, COLS) {
			let neighbors = count_neighbors(x, y, &*ghost_grid);
			//print!("N %d, ", neighbors);
			if neighbors < 2 || neighbors > 3 {
				grid[y][x] = 0;
			} else if neighbors == 3 {
				grid[y][x] = 1;
			}
		}
	}
	update_ghost(&*grid, ghost_grid);
}

fn main() {
	let mut grid: Grid = [[0, ..COLS+1], ..ROWS+1];
	let mut ghost_grid: GhostGrid = [[0, ..COLS+1+2], ..ROWS+1+2];

	/* Generate a random grid */
	for y in range(0u, ROWS) {
		for x in range(0u, COLS) {
			grid[y][x] = (rand::random::<uint>() % 2) as u8;
		}
	}

	update_ghost(&grid, &mut ghost_grid);

	for _ in range(0, 50000) {
		stdio::print("\n\n\n");
		pretty_print(&grid);
		//std::io::timer::sleep(160);
		next_gen(&mut grid, &mut ghost_grid);
	}
}
