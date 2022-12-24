$INIT
JMP MAIN

# =============================================
PRINT_CHAR:
# ARGS
# 0 char
$FUNC 0 1 0 # Func initialize, 0 locals 2 args 1 return
$LARG 0
LOAD.S r2 sp -1 # Load argument swapped to store correctly

LCONST r3 0xF104
STORE r2 r3 +2

LOAD r2 r3 +1
ACONST r2 8
STORE r2 r3 +1

$RET # Return

# =============================================

WAIT_READ:
# ARGS
# 0 count
$FUNC 0 1 0
$LARG 0

LCONST r3 0xF100
LCONST r4 0x7f
WAIT_READ__LOOP:
LOAD r2 r3 0
LOAD r5 r3 1
SUB r2 r2 r5
AND r2 r2 r4
LOAD r5 sp -1
SUB r2 r2 r5
JLT r2 WAIT_READ__LOOP # If < 0 then not enough characters in buffer

$RET # Return

# =============================================

READ_CHAR:
# ARGS
# 0 char
$FUNC 0 1 1 # Func initialize, 0 locals 1 args 1 return
$PCONST 8
$CALL WAIT_READ
LCONST r3 0xF100

MOV r2 ZX
LOAD.LS r2 r3 +2

LOAD r4 r3 +1
ACONST r4 8
STORE r4 r3 +1

$PUSH r2
$SARG 0
$RET # Return


# =============================================

PRINT_STRING:
# ARGS
# 0 string_pointer
$FUNC 0 1 0

PRINT_STRING__WRITE_LOOP:

$LARG 0
$LOAD
$DUP
$JEQ PRINT_STRING__END

$CALL PRINT_CHAR

# Increment current pointer
$LARG 0
$INC
$SARG 0

JMP PRINT_STRING__WRITE_LOOP

PRINT_STRING__END:
$RET

# =============================================
READ_STRING:
# ARGS
# 1 string_pointer
# 0 size
$FUNC 0 2 0
$PCONST 0
$SVAR 0

READ_STRING_LOOP:
$LARG 0
$JEQ READ_STRING__LINEEND 

$LARG 0
$DEC
$SARG 0

$PUSH ZX
$CALL READ_CHAR
$DUP

$LARG 1
$DUP
$INC
$SARG 1
$STORE

$PCONST 10
$SUB
$JEQ READ_STRING__END

READ_STRING__LINEEND:
$PUSH ZX
$CALL READ_CHAR
$PCONST 10
$SUB
$JNE READ_STRING__LINEEND


READ_STRING__END:
$RET

# =============================================
READ_OP:
# RET
# 0 op
$FUNC 0 1 1

READ_OP_LOOP:

$PLABEL READ_OP_BUF
$PCONST 1
$CALL READ_STRING
$PLABEL READ_OP_BUF
$LOAD

$DUP
$PCONST 43
$SUB
$JEQ READ_OP_PLUS


$DUP
$PCONST 45
$SUB
$JEQ READ_OP_MINUS

$PCONST 94
$SUB
$JEQ READ_OP_XOR

$PLABEL UNKNOWN_OPERATION
$CALL PRINT_STRING
$JMP READ_OP_LOOP

READ_OP_PLUS:
$POP zx
$PCONST 1
$SARG 0
$RET

READ_OP_MINUS:
$POP zx 
$PCONST 2
$SARG 0
$RET

READ_OP_XOR:
$POP zx
$PCONST 3
$SARG 0
$RET

# =============================================
READ_NUM:
# RET
# 0 num
$FUNC 0 1 1

READ_NUM_LOOP:


$PCONST 1
$CALL READ_STRING
$PLABEL READ_OP_BUF
$LOAD

$DUP
$PCONST 43
$SUB
$JEQ READ_OP_PLUS


$DUP
$PCONST 45
$SUB
$JEQ READ_OP_MINUS

$PCONST 94
$SUB
$JEQ READ_OP_XOR

$PLABEL UNKNOWN_OPERATION
$CALL PRINT_STRING
$JMP READ_OP_LOOP

READ_OP_PLUS:
$POP zx
$PCONST 1
$SARG 0
$RET

READ_OP_MINUS:
$POP zx 
$PCONST 2
$SARG 0
$RET

READ_OP_XOR:
$POP zx
$PCONST 3
$SARG 0
$RET

# =============================================

MAIN:
$FUNC 2 0 0

$PLABEL GREET 
$CALL PRINT_STRING

MAIN__ASK_OPERATION:
$PCONST 0xDEAD
$PCONST 0
$CALL READ_OP

HALT

# $PLABEL ASK_OPERATION 
# $CALL PRINT_STRING


# $PUSH ZX
# $CALL READ_CHAR
# $SVAR 0

# $PUSH ZX
# $CALL READ_CHAR
# $POP ZX

# # IS: if(ch == '+')
# $LVAR 0
# $ACONST -43 
# $JNE MAIN__PLUS_ENDIF 

# $PCONST 1 # 1  corresponds to '+'
# $SVAR 0

# $PLABEL PLUS_OPERATION
# $CALL PRINT_STRING

# JMP MAIN__OPFOUND

# MAIN__PLUS_ENDIF:
# # HALT

# $PLABEL UNKNOWN_OPERATION
# $CALL PRINT_STRING
# JMP MAIN__ASK_OPERATION


# MAIN__OPFOUND:
# $PCONST 1
# $CALL WAIT_READ
# $PCONST 0xDEAD
# HALT

GREET:
"Hello, this is simple calculator!"
0xa 
0

ASK_OPERATION:
"Enter required operation: "
0


PLUS_OPERATION:
"Operation plus"
0xa
0

SUB_OPERATION:
"Operation sub"
0xa
0

XOR_OPERATION:
"Operation xor"
0xa
0


UNKNOWN_OPERATION:
"Unknown operation"
0xa
0

INCORRECT_NUMBER:
"Incorrect number"
0xa
0

READ_OP_BUF:
0
0
0
0

READ_NUM_BUF:
0
0
0
0