#include <stdio.h>
#include <mpi.h>
#include <time.h>
#include <stdlib.h>

#define MSG_DATA 100

int main (int argc, char *argv[])
{
	int rank, size;
	int token;
	MPI_Status status;

	MPI_Init(&argc, &argv);

	MPI_Comm_size(MPI_COMM_WORLD, &size);
	MPI_Comm_rank(MPI_COMM_WORLD, &rank);
	MPI_Barrier(MPI_COMM_WORLD);

	double start = MPI_Wtime();

	// The master is supposed to generate a random number and pass it onto the
	// process with the next rank.
	if (rank == 0) {
		token = 65;
		MPI_Send (&token, 1, MPI_INT, 1, MSG_DATA, MPI_COMM_WORLD);
	} else {
		MPI_Recv (&token, 1, MPI_INT, rank - 1, MSG_DATA, MPI_COMM_WORLD, &status);
		// Unless it is the last process.
		if (rank + 1 < size) {
			// Need to initialize with token as time(0) will likely result in
			// giving us the same time() value and hence the same seed.
			token = token + 1;
			MPI_Send (&token, 1, MPI_INT, rank + 1, MSG_DATA, MPI_COMM_WORLD);
		}
	}

	MPI_Barrier(MPI_COMM_WORLD);

	if (rank == 0) {
		printf("%lf", MPI_Wtime() - start);
	}

	MPI_Finalize();
	return 0;
}
