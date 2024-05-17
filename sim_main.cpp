#include <arpa/inet.h>
#include <sys/ioctl.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <csignal>

#include "Vour.h"
#include "verilated.h"
#include "instruction.h"


#define RAM_SIZE 65535

unsigned short RAM[RAM_SIZE + 1] = {0};
bool should_stop = false;

void print_ram(int rows, int start)
{
  int cur_idx = 0;

  for (int i = 0; i < rows; i++)
  {
    fprintf(stderr, "%04x %04x %04x %04x\n", RAM[start + cur_idx], RAM[start + cur_idx + 1], RAM[start + cur_idx + 2], RAM[start + cur_idx + 3]);
    cur_idx += 4;
  }
}

void signal_handler(int signal_num)
{
  fprintf(stderr, "Recieved int, halting\n");
      should_stop = true;
}

int main(int argc, char **argv, char **env)
{
  fprintf(stderr, "Loding initial RAM from ram.bin:\n");
  FILE *f = fopen("./ram.bin", "rb");
  fread(RAM, sizeof(unsigned short), RAM_SIZE, f);
  for (int i = 0; i < RAM_SIZE; i++)
    RAM[i] = ntohs(RAM[i]);
  fclose(f);

  signal(SIGINT, signal_handler);

  print_ram(4, 0);

  VerilatedContext *contextp = new VerilatedContext;
  contextp->commandArgs(argc, argv);
  Vour *top = new Vour{contextp};

  bool prev_par_signal = false;
  fprintf(stderr,"\nStarted cpu\n\n");

  unsigned short to_send = 0;
  unsigned short left_to_send = 0;
  unsigned short left_to_read = 0;
  unsigned short serial_state = 0;

  unsigned short inp_clock = 0;
  bool prev_output = false;
  unsigned short read_start = 0;
  unsigned short cur_read = 0;
  unsigned short read_bits_left = 8;

  while (!contextp->gotFinish())
  {
    top->eval();
    top->clk = 1;
    top->eval();
    top->clk = 0;

    // printf("RAM: 0x%04x %s 0x%04x\n", top->ram_addr, top->ram_op ? "WRITE" : "READ ", top->ram_op ? top->ram_write : 0 );
    auto ram_addr = top->phy_ram_addr;
    if (ram_addr >= (sizeof(RAM) / sizeof(RAM[0])))
    {
      fprintf(stderr, "Invalid RAM address\n");
      break;
    }

    if (top->phy_ram_op)
    { // WRITE
      RAM[ram_addr] = top->phy_ram_write;
      // printf("RAM: 0x%04x WRITE 0x%04x\n", ram_addr, top->ram_write);
    }
    else
    { // READ
      top->phy_ram_read = RAM[ram_addr];
      // printf("RAM: 0x%04x READ  0x%04x\n", ram_addr, RAM[ram_addr]);
    }

    if (top->halt_out)
    {
      fprintf(stderr,"\nCpu HALTed\n\n");
      break;
    }
    if (should_stop)
    {
      break;
    }


    // SERIAL INPUT
    if (left_to_read == 0)
    {
      int n;
      if (ioctl(0, FIONREAD, &n) == 0 && n > 0)
      {
        left_to_read += n;
      }
    }

    if (serial_state == 12)
    {
      top->serial_input_port = 1;
    }
    if (serial_state == 7)
    {
      top->serial_input_port = (to_send & 1 << 7) != 0;
    }
    if (serial_state == 0)
    {
      if (left_to_send == 0)
      {
        if (left_to_read > 0)
        {
          left_to_read--;
          to_send = getchar();
          left_to_send = 7;
          serial_state = 16;
        }
        else
        {
          serial_state = 0;
        }
      }
      else
      {
        to_send <<= 1;
        left_to_send--;
        serial_state = 16;
      }
      top->serial_input_port = 0;
    }
    if (serial_state > 0)
      serial_state--;

    // SERIAL OUTPUT
    inp_clock = (inp_clock + 1) & 15;
    if(prev_output != top->serial_output_port) {
      if(top->serial_output_port == 1) { // Rising edge
        read_start = inp_clock;
      }else{
        unsigned short passed = (inp_clock - read_start) & 0xf;
        cur_read <<= 1;
        read_bits_left--;

        if(passed > 8) {
          cur_read |= 1;
        }

        if(read_bits_left == 0) {
          // printf("C: %d\n", cur_read);
          printf("%c", cur_read);
          fflush(stdout);
          cur_read= 0;
          read_bits_left = 8;

        }
        // printf("Change in heart %d %d\n", top->serial_output_port, );
      }
    }
    prev_output = top->serial_output_port;

    // printf("%d %d\n", top->par_output_port, top->par_output_signal);
    // std::cout << "RAM: " << (int)top->ram_op << std::endl;
    // usleep(100000);
  }

  fprintf(stderr, "Program ram after HALT:\n");
  print_ram(4, 0);

  fprintf(stderr, "\nStack after HALT:\n");
  print_ram(8, 0x4000);

  delete top;
  delete contextp;
  return 0;
}
