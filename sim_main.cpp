#include "Vour.h"
#include "verilated.h"
#include <iostream>   
#include "instruction.h"

// unsigned short RAM[1024] = {
//   I_NOP  | 13, // Used for r0 load
//   I_NOP  | 11, // Used for r1 load

//   I_LOAD | ARG_REG1(REG_R0) | ARG_REG2(REG_PC) | 0b100 | 0x2,         // LOAD r0, pc + 0x1    
//   I_LOAD | ARG_REG1(REG_R1) | ARG_REG2(REG_PC) | 0b100 | 0x2,         // LOAD r0, pc + 0x1    
//   I_LOAD | ARG_REG1(REG_R2) | ARG_REG2(REG_PC) | 0x1,                 // LOAD r0, pc + 0x1  

//   I_NOP | 0,

//   I_BRANCH | I_BRANCH_GT | I_BRANCH_LT | ARG_REG1(REG_R2) | 0x3,      // JNE AND_OP
//   I_ALU_ADD | ARG_REG1(REG_R0) | ARG_REG2(REG_R0) | ARG_REG3(REG_R1), // ADD r0, r0, r1
//   I_BRANCH | I_BRANCH_ANY | 0x2,                                      // JMP END_IF
  
//   // AND_OP:-
//   I_ALU_AND | ARG_REG1(REG_R0) | ARG_REG2(REG_R0) | ARG_REG3(REG_R1), // ADD r0, r0, r1
  
//   // END_IF:
//   I_STORE | ARG_REG1(REG_R0) | ARG_REG2(REG_PC) | 0x2,                // STORE r0, pc + 0x2
  
//   I_CTR_HALT
// };


// unsigned short RAM[1024] = {
//   I_NOP  | 13, // Used for r0 load
//   I_NOP  | 11, // Used for r1 load

//   I_LOAD | ARG_REG1(REG_R0) | ARG_REG2(REG_PC) | 0b100 | 0x2,         // LOAD r0, pc + 0x1    
//   I_LOAD | ARG_REG1(REG_R1) | ARG_REG2(REG_PC) | 0b100 | 0x2,         // LOAD r0, pc + 0x1    
//   I_ALU_ADD | ARG_REG1(REG_R0) | ARG_REG2(REG_R0) | ARG_REG3(REG_R1), // ADD r0, r0, r1
//   // END_IF:
//   I_STORE | ARG_REG1(REG_R0) | ARG_REG2(REG_PC) | 0x2,                // STORE r0, pc + 0x2
  
//   I_CTR_HALT
// };


unsigned short RAM[1024] = {
  
  0xeff, 0x208d, 0x4690, 0x11, 0x20cd, 0x54d8, 0x18cb, 0x1b, 0x21cd, 0x5139, 0x5b23, 0x2120, 0x3110, 0x2111, 0x4720, 0x3111, 0x1e2b, 0x0, 0xfe00, 0x0, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x21, 0x21, 0xa, 0x0

};

int main(int argc, char** argv, char** env) {


    // for(int i = 0; i < 24; i++) {

    //   printf("%d\n", RAM[i]);
    //   // cur_idx += 4;
    // }


    VerilatedContext* contextp = new VerilatedContext;
    contextp->commandArgs(argc, argv);
    Vour* top = new Vour{contextp};
    
    bool prev_par_signal = false;
    printf("STARTED \n\n");

    while (!contextp->gotFinish()) { 
      top->eval();
      top->clk = 1;
      top->eval();
      top->clk = 0;

      // printf("RAM: 0x%04x %s 0x%04x\n", top->ram_addr, top->ram_op ? "WRITE" : "READ ", top->ram_op ? top->ram_write : 0 );
      auto ram_addr = top->phy_ram_addr;
      if( ram_addr >= (sizeof(RAM) / sizeof(RAM[0]))) {
        printf("Invalid RAM address\n");
        break;
      }

      if(top->phy_ram_op) { // WRITE
        RAM[ram_addr] = top->phy_ram_write;
        //printf("RAM: 0x%04x WRITE 0x%04x\n", ram_addr, top->ram_write);
      } else { // READ
        top->phy_ram_read = RAM[ram_addr];
        //printf("RAM: 0x%04x READ  0x%04x\n", ram_addr, RAM[ram_addr]);
      }
      
      
      if(top->halt_out) {
        printf("\n\nHALT\n");
        break;
      }
      if(top->par_output_signal != prev_par_signal) {
        prev_par_signal = top->par_output_signal;
        printf("%c", top->par_output_port);
      }
      // printf("%d %d\n", top->par_output_port, top->par_output_signal);
      // std::cout << "RAM: " << (int)top->ram_op << std::endl;
    }

    int cur_idx = 0;
    for(int i = 0; i < 5; i++) {

      printf("%d %d %d %d\n", RAM[cur_idx], RAM[cur_idx + 1], RAM[cur_idx + 2], (short)RAM[cur_idx + 3]);
      cur_idx += 4;
    }



    delete top;
    delete contextp;
    return 0;
}
