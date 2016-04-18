#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <mpi.h>


int main(int argc, char **argv)
{
	int i, j, rank, size;
	float val = 0;

	float x[1024];
	float y[1024];
	float a = 313.37;

	srand(time(0));

	
	for (i = 0; i < 1024; ++i)
	{
		x[i] = rand() / (float)RAND_MAX*4096.f + 0.f;
		y[i] = rand() / (float)RAND_MAX*4096.f + 0.f; 
	}

	MPI_Init(&argc, &argv);
	MPI_Comm_size(MPI_COMM_WORLD,&size);
	MPI_Comm_rank(MPI_COMM_WORLD,&rank);
	MPI_Status status;

	MPI_Barrier(MPI_COMM_WORLD);
	double start = MPI_Wtime();

	for (i = 0; i < 1024; ++i)
	{
		val += x[i]*a + y[i];
	}

	if (rank == 0)
	{
		for (i = 0; i < size-1; i++) {
			float recv_val = 0;
			MPI_Recv(&recv_val, 1, MPI_FLOAT, MPI_ANY_SOURCE, 42, MPI_COMM_WORLD, &status);
		}
	}
	else
	{
		MPI_Send(&val, 1, MPI_FLOAT, 0, 42, MPI_COMM_WORLD);
	}

	MPI_Barrier(MPI_COMM_WORLD);

	if (rank == 0)
	{
		printf("%lf", (MPI_Wtime() - start));
	}

	MPI_Finalize();

}
