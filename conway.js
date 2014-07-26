var COLS = 177;
var ROWS = 60;

var update_ghost = function(grid, ghost_grid) {
	/*
	A virtual grid that includes wrapped edges, so that we don't have to
	do funky modulo arithmetic.
	*/

	var n, x, y;

	/* Copy bottom of grid to top of ghost_grid */
	for(n=0; n < COLS; n++) {
		ghost_grid[0][n+1] = grid[ROWS-1][n];
	}

	/* Copy top of grid to bottom of ghost_grid */
	for(n=0; n < COLS; n++) {
		ghost_grid[ROWS+2-1][n+1] = grid[0][n];
	}

	/* Copy the rest of grid to ghost_grid */
	for(y=0; y < ROWS; y++) {
		for(x=0; x < COLS; x++) {
			ghost_grid[y+1][x+1] = grid[y][x];
		}
	}

	/* Wrap ghost_grid left and right columns */
	for(y=0; y < ROWS+2; y++) {
		ghost_grid[y][0] = ghost_grid[y][COLS+2-2];
		ghost_grid[y][COLS+2-1] = ghost_grid[y][1];
	}
}

var count_neighbors = function(x, y, ghost_grid) {
	return (
		ghost_grid[(y-1)+1][(x-1)+1] + ghost_grid[(y-1)+1][(x)+1] + ghost_grid[(y-1)+1][(x+1)+1] +
		ghost_grid[(y)+1  ][(x-1)+1]                              + ghost_grid[(y)+1  ][(x+1)+1] +
		ghost_grid[(y+1)+1][(x-1)+1] + ghost_grid[(y+1)+1][(x)+1] + ghost_grid[(y+1)+1][(x+1)+1]);
}

var out = Array((COLS+1)*ROWS);
var pretty_print = function(grid) {
	var x, y;
	var out_i = 0;
	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			if(grid[y][x] == 0) {
				out[out_i] = " ";
				out_i += 1;
			} else {
				out[out_i] = "#";
				out_i += 1;
			}
		}
		out[out_i] = "\n";
		out_i += 1;
	}
	process.stdout.write(out.join(''));
}

var next_gen = function(grid, ghost_grid) {
	var neighbors, x, y;
	for(y=0; y < ROWS; y++) {
		for(x=0; x < COLS; x++) {
			neighbors = count_neighbors(x, y, ghost_grid);
			//printf("N %d, ", neighbors);
			if(neighbors < 2 || neighbors > 3) {
				grid[y][x] = 0;
			} else if(neighbors == 3) {
				grid[y][x] = 1;
			}
		}
	}
	update_ghost(grid, ghost_grid);
}

var make_grid = function(width, height) {
	var rows = [];
	for(var i=0; i < height; i++) {
		rows.push(Array(width));
	}
	return rows;
}

var run_once = function() {
	var grid = make_grid(COLS, ROWS);
	var ghost_grid = make_grid(COLS + 2, ROWS + 2);
	var x;
	var y;

	/* Generate a random grid */
	for(y=0; y < ROWS; y++) {
		for(x=0; x < COLS; x++) {
			grid[y][x] = Math.floor(Math.random() * 2);
		}
	}

	pretty_print(grid);
	update_ghost(grid, ghost_grid);

	var iterations = 10000;

	while(iterations--) {
		process.stdout.write("\n\n\n");
		pretty_print(grid);
		//nanosleep(&wait, NULL);

		next_gen(grid, ghost_grid);
	}
}

var main = function() {
	run_once();
	process.stderr.write("Warmed up.\n");
	var start = Date.now();
	run_once()
	var end = Date.now();
	process.stderr.write("Ran in " + (end - start) + "ms");
}

main();
