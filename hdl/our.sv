typedef enum reg [3:0] {
   READ_INSTRUCTION, STAGE_E, HALT
} state_t;

module our(
   input clk,

   output reg[15:0] phy_ram_addr,
   output reg[15:0] phy_ram_write,
   output reg phy_ram_op,
   input reg[15:0] phy_ram_read,

   output reg halt_out,

   output reg serial_output_port,
   input reg serial_input_port
); 
   state_t state;
   state_t prev_state;


   reg[15:0] pc;
   reg[15:0] next_pc;

   reg[15:0] instruction;
   reg[95:0] reg_file;

   reg keep_running;

   reg inst_res_from_ram;
   reg[2:0] inst_res_target;
   reg[15:0] inst_res;
   
   reg[15:0] res;
   
   reg[15:0] inst_ram_addr,  inst_ram_write, ram_read_sw;
   reg[3:0] inst_ram_mode;
   reg inst_ram_op;


   reg[15:0] ram_addr;
   reg[15:0] ram_write;
   reg ram_op;
   reg[15:0] ram_read;

   wire _unused_ok = &{1'b0,
                        prev_state,
                        1'b0};

   inst INST(
      .instruction((prev_state == READ_INSTRUCTION) ? ram_read : instruction),
      .pc(pc),
      .reg_file(reg_file),
      
      .res_from_ram(inst_res_from_ram),
      .res_target(inst_res_target),
      .res(inst_res),

      .ram_addr(inst_ram_addr),
      .ram_write(inst_ram_write),
      .ram_op(inst_ram_op),
      .ram_mode(inst_ram_mode)
   );

   mem MEM(
      .clk(clk),

      .ram_addr(ram_addr),
      .ram_write(ram_write),
      .ram_op(ram_op),
      .ram_read(ram_read),


      .phy_ram_addr(phy_ram_addr),
      .phy_ram_write(phy_ram_write),
      .phy_ram_op(phy_ram_op),
      .phy_ram_read(phy_ram_read),

      .serial_output_port(serial_output_port),
      .serial_input_port(serial_input_port),

      .keep_running(keep_running)
   );

   initial begin
   ram_addr = 0;
   ram_write = 0; 
   ram_op = 0;

   state = READ_INSTRUCTION;
   prev_state = HALT;
   
   pc = ~0; // Next_pc will be equal 0
   instruction = 0;
   reg_file = 0;

   keep_running = 1;
   end

   assign ram_read_sw = inst_ram_mode[1] ? {ram_read[7:0], ram_read[15:8]} : ram_read;
   always @* begin
      res = inst_res;
      if(inst_res_from_ram) begin
         if(inst_ram_mode[0]) begin
            if(~inst_ram_mode[1]) 
               res = inst_res + ram_read;
            else
               res = inst_res - ram_read;
         end
         else begin
            if(inst_ram_mode[2]) 
               res[15:8] = ram_read_sw[15:8];
            
            if(inst_ram_mode[3]) 
               res[7:0] = ram_read_sw[7:0];
            
         end
      end
   end


   assign next_pc = (inst_res_target == 1) ? res : (pc + 1);

   always @ (posedge clk) begin



      prev_state <= state;

      case(state)
         READ_INSTRUCTION: begin
            // $display("Read", next_pc);
            ram_addr <= next_pc; // Read instruction 
            pc <= next_pc; 
            ram_op <= 0;
            state <= STAGE_E;

            case(inst_res_target)
               2: reg_file[95:80] <= res;
               3: reg_file[79:64] <= res;
               4: reg_file[63:48] <= res;
               5: reg_file[47:32] <= res;
               6: reg_file[31:16] <= res;
               7: reg_file[15:00] <= res;
            endcase

         end
         
         STAGE_E: begin
            instruction <= ram_read;
            // ram_addr <= pcounter + 3;
            // ram_op <= write_res;
            // ram_write <= res;
            // pcounter <= pcounter + 4;
            // // $display(halt ? "HALT" :"ELLO?");
            // // if(halt) {
            //$display("INSTRUCTION %d", pc);
            // // }
            ram_addr <= inst_ram_addr;
            ram_op <= inst_ram_op;
            ram_write <= inst_ram_write;
            state <= keep_running ? READ_INSTRUCTION : HALT;
            
         end
         HALT: halt_out <= 1;
         default: ;
      endcase
      
   end
endmodule
