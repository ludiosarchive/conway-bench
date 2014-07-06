public class Conway {

static int ROWS = 177;
static int COLS = 60;

static void update_ghost(short grid[][], short ghost_grid[][]) {
	/*
	A virtual grid that includes wrapped edges, so that we don't have to
	do funky modulo arithmetic.
	*/

	int n, x, y;

	/* Copy bottom of grid to top of ghost_grid */
	for(n=0; n <= ROWS-1; n++) {
		ghost_grid[n+1][0] = grid[n+1][COLS-1];
	}

	/* Copy top of grid to bottom of ghost_grid */
	for(n=0; n <= ROWS-1; n++) {
		ghost_grid[n+1][COLS+2-1] = grid[n+1][0];
	}

	/* Copy the rest of grid to ghost_grid */
	for(y=0; y <= COLS-1; y++) {
		for(x=0; x <= ROWS-1; x++) {
			ghost_grid[x+1][y+1] = grid[x][y];
		}
	}

	/* Wrap ghost_grid left and right columns */
	for(y=0; y <= COLS+2-1; y++) {
		ghost_grid[0][y] = ghost_grid[ROWS+2-2][y];
		ghost_grid[ROWS+2-2][y] = ghost_grid[1][y];
	}
}

static int count_neighbors(int x, int y, short ghost_grid[][]) {
	return
		ghost_grid[(x-1)+1][(y-1)+1] + ghost_grid[(x)+1][(y-1)+1] + ghost_grid[(x+1)+1][(y-1)+1] +
		ghost_grid[(x-1)+1][(y)+1  ]                              + ghost_grid[(x+1)+1][(y)+1  ] +
		ghost_grid[(x-1)+1][(y+1)+1] + ghost_grid[(x)+1][(y+1)+1] + ghost_grid[(x+1)+1][(y+1)+1];
}

static void pretty_print(short grid[][]) {
	int x, y;

	char out[] = new char[(ROWS+1)*COLS];
	int out_i = 0;

	for(y=0; y <= COLS-1; y++) {
		for(x=0; x <= ROWS-1; x++) {
			if(grid[x][y] == 0) {
				out[out_i] = ' ';
				out_i += 1;
			} else {
				out[out_i] = '#';
				out_i += 1;
			}
		}
		out[out_i] = '\n';
		out_i += 1;
	}
	System.out.print(out);
}

static void next_gen(short grid[][], short ghost_grid[][]) {
	int neighbors, x, y;
	for(y=0; y <= COLS-1; y++) {
		for(x=0; x <= ROWS-1; x++) {
			neighbors = count_neighbors(x, y, ghost_grid);
			//printf("N %d, ", neighbors);
			if(neighbors < 2 || neighbors > 3) {
				grid[x][y] = 0;
			} else if(neighbors == 3) {
				grid[x][y] = 1;
			}
		}
	}
	update_ghost(grid, ghost_grid);
}


public static void main(String[] args) {
	short grid[][] = new short[ROWS][COLS];
	short ghost_grid[][] = new short[ROWS][COLS];
	int character;
	int x;
	int y;

	/* Generate a random grid */
	for(y=0; y <= COLS-1; y++) {
		for(x=0; x <= ROWS-1; x++) {
			grid[x][y] = (short)Math.round(Math.random() * 2);
		}
	}

	pretty_print(grid);
	update_ghost(grid, ghost_grid);

	int iterations = 50000;

	while(iterations-- > 0) {
		System.out.print("\n\n\n");
		pretty_print(grid);
		//nanosleep(&wait, NULL);

		next_gen(grid, ghost_grid);
	}
}

}