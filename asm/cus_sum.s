LCONST r2 0b1101	#	Operand	1
LCONST r3 0b1011	#	Operand	2

LCONST r4 0 # Operation 0 and 1 add

JEQ r4 DO_AND

ADD r2 r2 r3
JMP FIN

DO_AND:
AND r2 r2 r3

FIN:
STORE r2 pc 2
HALT
0