extern crate rand;
use std::io::{self, Write};
use std::str;
use rand::Rng;

const COLS: usize = 177;
const ROWS: usize = 60;

type Grid = [[u8; COLS]; ROWS];
type GhostGrid = [[u8; COLS+2]; ROWS+2];

fn print(s: &[u8]) {
	std::io::stdout().write(s);
}

/**
 * A virtual grid that includes wrapped edges, so that we don't have to
 * do funky modulo arithmetic.
 */
fn update_ghost(grid: &Grid, ghost_grid: &mut GhostGrid) -> () {
	/* Copy bottom of grid to top of ghost_grid */
	for n in 0..COLS {
		ghost_grid[0][n+1] = grid[ROWS-1][n];
	}

	/* Copy top of grid to bottom of ghost_grid */
	for n in 0..COLS {
		ghost_grid[ROWS+2-1][n+1] = grid[0][n];
	}

	/* Copy the rest of grid to ghost_grid */
	for y in 0..ROWS {
		for x in 0..COLS {
			ghost_grid[y+1][x+1] = grid[y][x];
		}
	}

	/* Wrap ghost_grid left and right columns */
	for y in 0..ROWS+2 {
		ghost_grid[y][0] = ghost_grid[y][COLS+2-2];
		ghost_grid[y][COLS+2-1] = ghost_grid[y][1];
	}
}

fn count_neighbors(x: usize, y: usize, ghost_grid: &GhostGrid) -> u8 {
	ghost_grid[y-1][x-1] + ghost_grid[y-1][x] + ghost_grid[y-1][x+1] +
	ghost_grid[  y][x-1]                      + ghost_grid[y  ][x+1] +
	ghost_grid[y+1][x-1] + ghost_grid[y+1][x] + ghost_grid[y+1][x+1]
}

fn pretty_print(grid: &Grid) -> () {
	// COLS+1 for the newline
	let mut out: [u8; (ROWS*(COLS+1))] = [0; (ROWS*(COLS+1))];
	let mut out_i: usize = 0;
	for y in 0..ROWS {
		for x in 0..COLS {
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
	print(&out);
}

fn next_gen(grid: &mut Grid, ghost_grid: &mut GhostGrid) -> () {
	for y in 0..ROWS {
		for x in 0..COLS {
			let neighbors = count_neighbors(x + 1, y + 1, ghost_grid);
			//print!("N %d, ", neighbors);
			if neighbors < 2 || neighbors > 3 {
				grid[y][x] = 0;
			} else if neighbors == 3 {
				grid[y][x] = 1;
			}
		}
	}
	update_ghost(grid, ghost_grid);
}

fn main() {
	let mut grid: Grid = [[0; COLS]; ROWS];
	let mut ghost_grid: GhostGrid = [[0; COLS+2]; ROWS+2];

	/* Generate a random grid */
	for y in 0..ROWS {
		for x in 0..COLS {
			grid[y][x] = (rand::random::<usize>() % 2) as u8;
		}
	}

	update_ghost(&grid, &mut ghost_grid);

	for _ in 0..10000 {
		print!("\n\n\n");
		pretty_print(&grid);
		//std::io::timer::sleep(160);
		next_gen(&mut grid, &mut ghost_grid);
	}
}
