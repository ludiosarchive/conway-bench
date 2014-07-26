public class Conway {

static int COLS = 177;
static int ROWS = 60;

static void update_ghost(short grid[][], short ghost_grid[][]) {
	/*
	A virtual grid that includes wrapped edges, so that we don't have to
	do funky modulo arithmetic.
	*/

	int n, x, y;

	/* Copy bottom of grid to top of ghost_grid */
	for(n=0; n <= COLS-1; n++) {
		ghost_grid[0][n+1] = grid[ROWS-1][n];
	}

	/* Copy top of grid to bottom of ghost_grid */
	for(n=0; n <= COLS-1; n++) {
		ghost_grid[ROWS+2-1][n+1] = grid[0][n];
	}

	/* Copy the rest of grid to ghost_grid */
	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			ghost_grid[y+1][x+1] = grid[y][x];
		}
	}

	/* Wrap ghost_grid left and right columns */
	for(y=0; y <= ROWS+2-1; y++) {
		ghost_grid[y][0] = ghost_grid[y][COLS+2-2];
		ghost_grid[y][COLS+2-1] = ghost_grid[y][1];
	}
}

static int count_neighbors(int x, int y, short ghost_grid[][]) {
	return
		ghost_grid[(y-1)+1][(x-1)+1] + ghost_grid[(y-1)+1][(x)+1] + ghost_grid[(y-1)+1][(x+1)+1] +
		ghost_grid[(y)+1  ][(x-1)+1]                              + ghost_grid[(y)+1  ][(x+1)+1] +
		ghost_grid[(y+1)+1][(x-1)+1] + ghost_grid[(y+1)+1][(x)+1] + ghost_grid[(y+1)+1][(x+1)+1];
}

static void pretty_print(short grid[][]) {
	int x, y;

	char out[] = new char[(COLS+1)*ROWS];
	int out_i = 0;

	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			if(grid[y][x] == 0) {
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
	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
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

static void run_once() {
	short grid[][] = new short[ROWS][COLS];
	short ghost_grid[][] = new short[ROWS+2][COLS+2];
	int character;
	int x;
	int y;

	/* Generate a random grid */
	for(y=0; y <= ROWS-1; y++) {
		for(x=0; x <= COLS-1; x++) {
			grid[y][x] = (short)Math.floor(Math.random() * 2);
		}
	}

	pretty_print(grid);
	update_ghost(grid, ghost_grid);

	int iterations = 10000;

	while(iterations-- > 0) {
		System.out.print("\n\n\n");
		pretty_print(grid);
		//nanosleep(&wait, NULL);

		next_gen(grid, ghost_grid);
	}
}

public static void main(String[] args) {
	run_once();
	System.err.println("Warmed up.");
	long start = System.currentTimeMillis();
	run_once();
	long end = System.currentTimeMillis();
	System.err.println("Ran in " + (end - start) + "ms");
}

}