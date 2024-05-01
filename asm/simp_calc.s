$INIT
JMP MAIN

PRINT_CHAR: {
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
}

# =============================================

WAIT_READ: {
  # ARGS
  # 0 count
  $FUNC 0 1 0
  $LARG 0

  LCONST r3 0xF100
  LCONST r4 0x7f
  
  LOOP:
  LOAD r2 r3 0
  LOAD r5 r3 1
  SUB r2 r2 r5
  AND r2 r2 r4
  LOAD r5 sp -1
  SUB r2 r2 r5
  JLT r2 LOOP # If < 0 then not enough characters in buffer

  $RET # Return
}

# =============================================

READ_CHAR: {
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
}


# =============================================

PRINT_STRING: {
  # ARGS
  # 0 string_pointer
  $FUNC 0 1 0

  LOOP:

  $LARG 0
  $LOAD
  $DUP
  $JEQ END

  $CALL PRINT_CHAR

  # Increment current pointer
  $LARG 0
  $INC
  $SARG 0

  JMP LOOP

  END:

  $RET
}

# =============================================
READ_STRING: {
  #     1              0
  # ARG string_pointer size
  # RET readed
  # VAR inpsize

  $FUNC 1 2 1
  
  $SVAR 0 ($LARG 0)

  LOOP:
  $JLE LINEEND ($LARG 0)

  $SARG 0 ($DEC ($LARG 0))
  
  $CALL READ_CHAR ($PUSH ZX)
  $DUP

  $LARG 1
  $DUP
  $SARG 1 ($INC)
  $STORE

  $JEQ END ($ACONST -10) # == '\n'
  $JMP LOOP

  LINEEND: # Wait for new line
  $PUSH ZX
  $CALL READ_CHAR
  $JNE LINEEND ($ACONST -10) # != '\n'


  END:
  $SARG 1 ($SUB ($LVAR 0; $LARG 0))
  $RET
}


# =============================================
READ_OP: {
  #     0
  # RET op
  # ARG prompt
  $FUNC 1 1 1

  $SVAR 0 ($LARG 0)
  LOOP:
  
  $JEQ NO_PROMPT ($LVAR 0)
  $CALL PRINT_STRING ($LVAR 0)
  NO_PROMPT:

  $CALL READ_STRING ($PLABEL BUF; $PCONST 1)
  $DROP

  $LOAD ($PLABEL BUF) # Load char from buf onto stack

  { $JNE IFNOT ($DUP; $ACONST -43)   # char == +
    $SARG 0 ($PCONST 1); $RET
  IFNOT: }

  { $JNE IFNOT ($DUP; $ACONST -45)   # char == +
    $SARG 0 ($PCONST 2); $RET
  IFNOT: }

  { $JNE IFNOT ($DUP; $ACONST -94)   # char == +
    $SARG 0 ($PCONST 3); $RET
  IFNOT: }

  $DROP

  $PLABEL UNKNOWN_OPERATION
  $CALL PRINT_STRING

  $JMP LOOP

  BUF: 0 0
  UNKNOWN_OPERATION: "Unknown operation\n" 0
}

# ===========================================
MATCH_CHAR_TO_NUM: {
  #      1    0
  # RET  num  is_ok
  # ARG  char _
  $FUNC 0 2 2

  $LARG 1
  
  $SARG 0 ($PCONST 1)
  $SARG 1 ($PCONST 0xdead)

  { # Check 0-9 
    $DUP; $ACONST -48
    $JLT IFNOT ($DUP)                # >= '0'
    $JGE IFNOT ($DUP; $ACONST -10)   # <= '9'

    $SARG 1; $RET
    
    IFNOT: $DROP 
  }

  { # Check a-f & A-F 
    $DUP; $ACONST -97
    
    # If input is uppercase letter than after -97 result will be negative 0
    # so we add 32 to match it with a-f
    $JGE SKIP_UPPER_FIX ($DUP);
    $ACONST 32
    SKIP_UPPER_FIX:

    $JLT IFNOT ($DUP)                # >= '0'
    $JGE IFNOT ($DUP; $ACONST -6)   # <= '9'

    $ACONST 10
    $SARG 1; $RET
    
    IFNOT: $DROP 
  }

  $SARG 0 ($PCONST 0)
  $RET
}


# ===========================================
MATCH_NUM_TO_CHAR: {
  #      1   
  # RET  char 
  # ARG  num _
  $FUNC 0 1 1

  $LARG 0
  $JGE LETTER ($DUP; $ACONST -10)
  
  $ACONST 48
  $SARG 0
  $RET

  LETTER:
  $ACONST 87
  $SARG 0

  $RET
}


# =============================================
READ_NUM_16: {
  #     0        1
  # ARG prompt
  # RET num     
  # VAR read_len cur_read prompt

  $FUNC 3 1 1

  $SVAR 2 ($LARG 0)
  LOOP:
  
  $JEQ NO_PROMPT ($LVAR 2)
  $CALL PRINT_STRING ($LVAR 2)
  NO_PROMPT:

  $CALL READ_STRING ($PLABEL BUF; $PCONST 5) 
  
  $DEC
  $SVAR 0 # Store
  
  $PUSH ZX
  $JLE FAIL_READ ($LVAR 0) # <= back_point - buf_start
  $DROP

  $SVAR 1 ($PLABEL BUF)
  $SARG 0 ($PUSH ZX)

  READ_LOOP:

  $LARG 0
  $SHL; $SHL; $SHL; $SHL
  $SARG 0

  $CALL MATCH_CHAR_TO_NUM ($LOAD ($LVAR 1); $PUSH ZX)
  $JEQ FAIL_READ
  
  $SARG 0 ($ADD ($LARG 0))
  $SVAR 1  ($INC ($LVAR 1)) 

  $SVAR 0 ($DEC ($LVAR 0))

  $JLE DONE ($LVAR 0)
  JMP READ_LOOP

  FAIL_READ:
  $DROP
  $CALL PRINT_STRING ($PLABEL INVALID_NUMBER)
  JMP LOOP
  
  DONE:
  $RET 

  BUF: 0 0 0 0 0 0 0
  INVALID_NUMBER: "Invalid number\n" 0
}

# =============================================

EXECUTE_OPERATION: {
  #     2  1 0
  # ARG op a b
  # RET te
  $FUNC 0 3 1

  $LARG 1
  $LARG 0

  $LARG 2

  { $JNE IFNOT ($DUP; $ACONST -1)   # op plus
  $DROP; $ADD; $SARG 2; $RET
  IFNOT: }

  { $JNE IFNOT ($DUP; $ACONST -2)   # op sub
  $DROP; $SUB; $SARG 2; $RET
  IFNOT: }

  { $JNE IFNOT ($DUP; $ACONST -3)   # op plus
  $DROP; $AND; $SARG 2; $RET
  IFNOT: }

  HALT
}

# =============================================
PRINT_NUM16: {
  #     0
  # ARG num
  # RET _
  # VAR cuw
  $FUNC 1 1 0

  
  $SVAR 0 ($DEC ($PLABEL PRINT_BUF_END))

  # Digit left
  LOOP:
  $CALL MATCH_NUM_TO_CHAR ($AND ($LARG 0; $PCONST 0xf));
  $STORE ($LVAR 0)

  $SVAR 0 ($DEC ($LVAR 0))
  
  $LARG 0
  $DUP
  $SARG 0 ($SHR; $SHR; $SHR; $SHR)

  $JNE LOOP ($LARG 0)
  
  $CALL PRINT_STRING ($INC ($LVAR 0))
  $RET

  PRINT_BUF: "######" PRINT_BUF_END: 0xa 0 0
}

# =============================================

MAIN: {
  $FUNC 2 0 0

  $CALL PRINT_STRING ($PLABEL GREET)

  $CALL READ_OP ($PLABEL ASK_OPERATION)
  $CALL READ_NUM_16 ($PLABEL ASK_OPERAND1)
  $CALL READ_NUM_16 ($PLABEL ASK_OPERAND2)

  $CALL EXECUTE_OPERATION

  $CALL PRINT_STRING ($PLABEL RESULT)
  $CALL PRINT_NUM16
  
  $CALL PRINT_STRING ($PLABEL GOODBYE)

  # Wait a little
  $PUSH ZX; $DROP;
  $PUSH ZX; $DROP;
  $PUSH ZX; $DROP;
  $PUSH ZX; $DROP;

  HALT

  TEST_BUF: 
}

GREET: "Hello, this is simple calculator!\n" 0
ASK_OPERATION: "Enter operation: " 0
ASK_OPERAND1:  "Enter operand 1: 0x" 0
ASK_OPERAND2:  "Enter operand 2: 0x" 0
RESULT:        "Result:          0x" 0
GOODBYE:       "Goodbye!\n" 0

DBG: "DEBUG\n" 0