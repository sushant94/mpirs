#include <stdio.h>
#include <mpi.h>
#include <time.h>
#include <stdlib.h>
#include <string.h>

#define MSG_DATA 100

int main (int argc, char *argv[])
{
	int rank, size;

	MPI_Status status;
	MPI_Init(&argc, &argv);

	MPI_Comm_size(MPI_COMM_WORLD, &size);
	MPI_Comm_rank(MPI_COMM_WORLD, &rank);


	int ssize = atoi(argv[1]);
	int *data;

	data = (int *) malloc(ssize * sizeof(int));

	if (rank == 0) {
		for (int i = 0; i < ssize; i++) {
			data[i] = rand() % ssize;
		}
	}

	MPI_Barrier(MPI_COMM_WORLD);
	double start = MPI_Wtime();

	if (rank == 0) {
		MPI_Send (data, ssize, MPI_INT, 1, MSG_DATA, MPI_COMM_WORLD);
	} else {
		MPI_Recv (data, ssize, MPI_INT, 0, MSG_DATA, MPI_COMM_WORLD, &status);
	}

	MPI_Barrier(MPI_COMM_WORLD);

	if (rank == 0) {
		printf("%lf", MPI_Wtime() - start);
	}

	MPI_Finalize();
	return 0;
}
