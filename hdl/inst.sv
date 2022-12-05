module inst(
  input[15:0] instruction,
  input[15:0] pc,
  input[95:0] reg_file,

  output reg[15:0] res,
  output reg res_from_ram,
  output reg[2:0] res_target,

  output reg[15:0] ram_addr,
  output reg ram_op,
  output reg[15:0] ram_write,
  output reg[3:0] ram_mode
);
  // reg[15:0] x;
  // reg[15:0] y;
  
  reg[15:0] arg_reg1, arg_reg2, arg_reg3;

  reg[15:0] alu_x, alu_y, shift;

  reg[2:0] dst_reg;
  reg low_bits, cond_eq, cond_lt, cond_gt, should_branch;

  assign dst_reg = instruction[8:6];

  reg_read ARG_REG1(
    .targ(dst_reg),
    .pc(pc),
    .reg_file(reg_file),

    .res(arg_reg1)
  ); 

   reg_read ARG_REG2(
    .targ(instruction[5:3]),
    .pc(pc),
    .reg_file(reg_file),

    .res(arg_reg2)
  ); 

   reg_read ARG_REG3(
    .targ(instruction[2:0]),
    .pc(pc),
    .reg_file(reg_file),

    .res(arg_reg3)
  ); 
  
  assign alu_x = instruction[11] ? ~arg_reg2 : arg_reg2;
  assign alu_y = instruction[10] ? ~arg_reg3 : arg_reg3;

  always @* begin
  res_from_ram = 0;
  res_target = 0;

  ram_addr = 0;
  ram_op = 0;
  ram_write = 0;

  //   write_res = 0;
  //   halt = 0;

  case(instruction[15:12]) 
      'b0000:;

      'b0100,          // AND
      'b0101: begin    // ADD
        // if instruction[11] && !instruction[
        if(instruction[11:9] == 3'b100) begin // Override useless instructions
          res = instruction[12] ? (arg_reg2 ^ arg_reg3) : (arg_reg2 >> 1);
          // $display("Override instructions ", instruction[12], " ", arg_reg2,  " " , arg_reg2 >> 1, " ", res);
        end
        else begin 
          res = instruction[12] ? (alu_x + alu_y) : (alu_x & alu_y);
          res = instruction[9] ? ~res : res;
        end
        res_target = dst_reg;
      end

      'b0010,       // LOAD
      'b0011: begin // STORE
        shift = {14'h0000, instruction[1:0]};
        shift = instruction[2] ? (-shift) : shift;
        
        ram_addr = arg_reg2 + shift;
        ram_write = arg_reg1;
        ram_mode = {instruction[11:9], instruction[11:10] == 'b00};

        ram_op = instruction[12];
        res = arg_reg1;
        res_from_ram = 1;
        res_target = instruction[12] ? 0 : dst_reg;
      end

      'b0001: begin // BRANCH

        shift = {11'h0000, instruction[4:0]};
        shift = instruction[5] ? (-shift) : shift;

        low_bits = |arg_reg1[14:0];
        
        cond_eq = ~(low_bits | arg_reg1[15]);
        cond_gt = low_bits & ~arg_reg1[15]; 
        cond_lt = arg_reg1[15];

        should_branch = 0;
        should_branch |= cond_eq & instruction[11];
        should_branch |= cond_gt & instruction[10];
        should_branch |= cond_lt & instruction[9];
        // $display("BRANCH ", pc, " ", instruction[15:12], " ", should_branch);
        if(should_branch) begin
          // $display("Actual branch");
          res_target = 1;
          res = pc + shift;
        end
      end
      default: begin
        // $display("Unknown command pc: ", pc, " ins: ", instruction);
      end

  endcase

  end
endmodule;
