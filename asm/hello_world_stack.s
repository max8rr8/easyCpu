$INIT
JMP MAIN

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

MAIN:
$FUNC 2 0 0

$PLABEL GREET 
$CALL PRINT_STRING

HALT

GREET:
"Hello WORLD!"
0xa 
0xa 
0