from mpi4py import MPI
from numpy import *
import sys

comm = MPI.COMM_WORLD
size = comm.Get_size()
rank = comm.Get_rank()

buf = None

if rank == 0:
	# print sys.argv
	buf = random.randint(0,int(sys.argv[1]),int(sys.argv[1]))

comm.Barrier()
start = MPI.Wtime()

if rank == 0:
	comm.send(buf,dest=1,tag=42)
else:
	buf = comm.recv(source=0,tag=42)
	# print buf

comm.Barrier()
stop = MPI.Wtime()

if rank == 0 :
	print stop-start
