use crate::runner::{ExecCond, StackOptExec, Test, TestGroup};

pub fn simple() -> Test {
    let mut g = TestGroup::new("simple");

    g.add(
        "puzx",
        StackOptExec::new("$PUZX", vec![ExecCond::CheckStack(vec![0])]),
    );

    g.add(
        "supdup",
        StackOptExec::new(
            "$DUP; $DUP; $DUP; $DUP; $DUP; $DUP; $DUP",
            vec![
                ExecCond::SetStack(vec![0x1234]),
                ExecCond::CheckStack(vec![
                    0x1234, 0x1234, 0x1234, 0x1234, 0x1234, 0x1234, 0x1234, 0x1234,
                ]),
            ],
        ),
    );

    g.add(
        "simpexpr",
        StackOptExec::new(
            "$AND; $PCONST 14; $PCONST 3; $SUB; $ADD",
            vec![
                ExecCond::SetStack(vec![0x10, 0x12]),
                ExecCond::CheckStack(vec![0x1b]),
            ],
        ),
    );

    g.add(
        "condcalc",
        StackOptExec::new(
            "$JEQ DO_ADD 
            $AND; $JMP END
            DO_ADD: $ADD
            END:
            ",
            vec![
                ExecCond::SetStack(vec![0x14, 0x13, 0x1]),
                ExecCond::CheckStack(vec![0x10]),
            ],
        )
        .add_case(vec![
            ExecCond::SetStack(vec![0x14, 0x13, 0x0]),
            ExecCond::CheckStack(vec![0x27]),
        ]),
    );



    g.add(
        "trickyjump",
        StackOptExec::new(
            "$PCONST 0x20
            $PCONST 0
            
            $DUP
            $JNE LLLL
            $DROP
            $ADD
            LLLL:
            ",
            vec![
                ExecCond::SetStack(vec![0x10]),
                ExecCond::CheckStack(vec![0x30]),
            ],
        )
    );

    g.add(
        "formnum",
        StackOptExec::new(
            "$DUP

            $AND ($PCONST 0xf)
            $ADD ($PLABEL RESOLVEBUF)
            $LOAD
            $SWP 
            
            $SHR; $SHR; $SHR; $SHR
            
            $AND ($PCONST 0xf)
            $ADD ($PLABEL RESOLVEBUF)
            $LOAD
            $SWP 
            } # END STACKOPT
            HALT
            RESOLVEBUF: \"0123456789abcdef\"
            {",
            vec![
                ExecCond::SetStack(vec![0x5a]),
                ExecCond::CheckStack(vec![0x35, 0x61]),
            ],
        )
        .add_case(vec![
            ExecCond::SetStack(vec![0xff]),
            ExecCond::CheckStack(vec![0x66, 0x66]),
        ])
        .add_case(vec![
            ExecCond::SetStack(vec![0]),
            ExecCond::CheckStack(vec![0x30, 0x30]),
        ])
        .add_case(vec![
            ExecCond::SetStack(vec![0xc4]),
            ExecCond::CheckStack(vec![0x63, 0x34]),
        ]),
    );


    g.add(
        "parsedigit",
        StackOptExec::new(
            "
            { # Check 0-9 
              $DUP; $ACONST -48
              $JLT IFNOT ($DUP)                # >= '0'
              $JGE IFNOT ($DUP; $ACONST -10)   # <= '9'
          
              $SWP; $DROP
              $JMP END
              
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

              $SWP; $DROP
              $JMP END
              
              IFNOT: $DROP 
            }

            $DROP; $PCONST 0xdead
            END:
            ",
            vec![
                ExecCond::SetStack(vec![0x34]),
                ExecCond::CheckStack(vec![4]),
            ],
        )
        .add_case(vec![
            ExecCond::SetStack(vec![0xff]),
            ExecCond::CheckStack(vec![0xdead]),
        ])
        .add_case(vec![
            ExecCond::SetStack(vec![0x61]),
            ExecCond::CheckStack(vec![0xa]),
        ])
        .add_case(vec![
            ExecCond::SetStack(vec![0x44]),
            ExecCond::CheckStack(vec![0xd]),
        ]),
    );

    g.into()
}
