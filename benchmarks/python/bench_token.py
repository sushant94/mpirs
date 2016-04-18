from mpi4py import MPI

comm = MPI.COMM_WORLD
size = comm.Get_size()
rank = comm.Get_rank()

comm.Barrier()
start = MPI.Wtime()

if rank == 0:
	token = 65
	comm.send(token+1,dest=(rank+1)%size,tag=42)
	token = comm.recv(source=size-1,tag=42)
	# print "Process %d received token %c from process %d"%(rank,chr(token),size-1)
else:
	token = comm.recv(source=rank-1,tag=42)
	# print "Process %d received token %c from process %d"%(rank,chr(token),rank-1)
	comm.send(token+1,dest=(rank+1)%size,tag=42)

comm.Barrier()
stop = MPI.Wtime()

if rank == 0 :
	print stop-start
