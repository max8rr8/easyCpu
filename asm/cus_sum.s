LC r0 0b1101	#	Operand	1
LC r1 0b1011	#	Operand	2

LC r2 1 # Operation 0 and 1 add

JEQ r2 DO_AND

ADD r0 r0 r1
JMP FIN

DO_AND:
AND r0 r0 r1

FIN:
STORE r0 pc 2
HALT
0