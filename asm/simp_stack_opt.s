$INIT # Initialize stack
@STACKOPT {
    $PCONST 0b1101 # Push 0b1101 onto stack
    $PCONST 0b1011 # Push 0b1011 onto stack

    $PCONST 0 # Push 0 onto stack
    $JEQ DO_AND # If top value on stack is zero jump to DO_AND

    $ADD # Add two values from stack and push result onto it
    $JMP FIN # Jump to FIN

    DO_AND:
    $AND # Logic-And two values from stack and push result onto it 

    FIN:
}
HALT
0