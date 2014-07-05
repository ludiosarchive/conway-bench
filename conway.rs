use std::rand::random;

static ROWS: uint = 78;
static COLS: uint = 20;

type Grid = [[u8, ..ROWS], ..COLS];
type GhostGrid = [[u8, ..ROWS+2], ..COLS+2];

/**
 * A virtual grid that includes wrapped edges, so that we don't have to
 * do funky modulo arithmetic.
 */
fn update_ghost(grid: &Grid, ghost_grid: &GhostGrid) -> () {
	/* Copy bottom of grid to top of ghost_grid */
	for n in range(0u, ROWS) {
		ghost_grid[n+1][0] = grid[n+1][COLS-1];
	}

	/* Copy top of grid to bottom of ghost_grid */
	for n in range(0u, ROWS) {
		ghost_grid[n+1][COLS+2-1] = grid[n+1][0];
	}

	/* Copy the rest of grid to ghost_grid */
	for y in range(0u, COLS) {
		for x in range(0u, ROWS) {
			ghost_grid[x+1][y+1] = grid[x][y];
		}
	}

	/* Wrap ghost_grid left and right columns */
	for n in range(0u, COLS+2) {
		ghost_grid[0][y] = ghost_grid[ROWS+2-2][y];
		ghost_grid[ROWS+2-2][y] = ghost_grid[1][y];
	}
}

fn count_neighbors(x: uint, y: uint, ghost_grid: &GhostGrid) -> uint {
	ghost_grid[(x-1)+1][(y-1)+1] + ghost_grid[(x)+1][(y-1)+1] + ghost_grid[(x+1)+1][(y-1)+1] +
	ghost_grid[(x-1)+1][(y)+1  ]                              + ghost_grid[(x+1)+1][(y)+1  ] +
	ghost_grid[(x-1)+1][(y+1)+1] + ghost_grid[(x)+1][(y+1)+1] + ghost_grid[(x+1)+1][(y+1)+1]
}

fn pretty_print(grid: &Grid) -> () {
	for y in range(0u, COLS) {
		for x in range(0u, ROWS) {
			if(grid[x][y] == 0) {
				print!(" ");
			} else {
				print!("#");
			}
		}
		print!("\n");
	}
}

fn next_gen(grid: &Grid, ghost_grid: &GhostGrid) -> () {
	for y in range(0u, COLS) {
		for x in range(0u, ROWS) {
			let neighbors = count_neighbors(x, y, ghost_grid);
			//print!("N %d, ", neighbors);
			if(neighbors < 2 || neighbors > 3) {
				grid[x][y] = 0;
			} else if(neighbors == 3) {
				grid[x][y] = 1;
			}
		}
	}
	update_ghost(grid, ghost_grid);
}

fn main() {
	let grid: Grid;
	let ghost_grid: GhostGrid;

	/* Generate a random grid */
	for y in range(0u, COLS) {
		for x in range(0u, ROWS) {
			grid[x][y] = random() % 2;
		}
	}

	pretty_print(grid);
	update_ghost(grid, ghost_grid);

	let iterations: uint = 1000000;

	while iterations-- {
		next_gen(grid, ghost_grid);
	}
}
