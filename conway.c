#include <stdio.h>
#include <stdlib.h>

#define ROWS 177
#define COLS 60

#define GRID_TYPE unsigned short int

static void update_ghost(GRID_TYPE grid[ROWS][COLS], GRID_TYPE ghost_grid[ROWS+2][COLS+2]) {
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

static int count_neighbors(int x, int y, GRID_TYPE ghost_grid[ROWS+2][COLS+2]) {
	return \
		ghost_grid[(x-1)+1][(y-1)+1] + ghost_grid[(x)+1][(y-1)+1] + ghost_grid[(x+1)+1][(y-1)+1] + \
		ghost_grid[(x-1)+1][(y)+1  ]                              + ghost_grid[(x+1)+1][(y)+1  ] + \
		ghost_grid[(x-1)+1][(y+1)+1] + ghost_grid[(x)+1][(y+1)+1] + ghost_grid[(x+1)+1][(y+1)+1];
}

static int get_random(N) {
	int val;
	while (N <= (val = rand() / (RAND_MAX/N)));
	return val;
}

static void pretty_print(GRID_TYPE grid[ROWS][COLS]) {
	int x, y;

	char out[(ROWS+1)*COLS];
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
	puts(out);
}

static void next_gen(GRID_TYPE grid[ROWS][COLS], GRID_TYPE ghost_grid[ROWS+2][COLS+2]) {
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


static struct timespec wait = {0, 160 * 1000 * 1000};

int main() {
	GRID_TYPE grid[ROWS][COLS];
	GRID_TYPE ghost_grid[ROWS+2][COLS+2];
	int character;
	int x;
	int y;

	srand(time(0));

	/* Generate a random grid */
	for(y=0; y <= COLS-1; y++) {
		for(x=0; x <= ROWS-1; x++) {
			grid[x][y] = get_random(2);
		}
	}

	pretty_print(grid);
	update_ghost(grid, ghost_grid);

	int iterations = 50000;

	while(iterations--) {
		puts("\n\n\n");
		pretty_print(grid);
		//nanosleep(&wait, NULL);

		next_gen(grid, ghost_grid);
	}
}
