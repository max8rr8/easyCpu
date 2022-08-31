module reg_read(
  input[2:0] targ,

  input[15:0] pc,
  input[95:0] reg_file,

  output reg[15:0] res
);
 

  always @* begin
    case(targ) 
        0: res = 0;
        1: res = pc;
        2: res = reg_file[95:80];
        3: res = reg_file[79:64];
        4: res = reg_file[63:48];
        5: res = reg_file[47:32];
        6: res = reg_file[31:16];
        7: res = reg_file[15:00];
    endcase
  end
endmodule;
