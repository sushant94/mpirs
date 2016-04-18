from mpi4py import MPI
from numpy import *

comm = MPI.COMM_WORLD
size = comm.Get_size()
rank = comm.Get_rank()

x = random.uniform(0, 4096, 1024)
y = random.uniform(0, 4096, 1024)
a = 313.37


comm.Barrier()
start = MPI.Wtime()

val = 0
for i in range(len(x)):
	val += (x[i] * a) + y[i]

if rank == 0:
    for i in range(size - 1):
        recv_val = comm.recv(source=MPI.ANY_SOURCE,tag=42)
else:
	comm.send(val,dest=0,tag=42)

comm.Barrier()
stop = MPI.Wtime()

if rank == 0 :
	print stop-start
