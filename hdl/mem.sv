
module mem(
   input clk,

   input reg[15:0] ram_addr,
   input reg[15:0] ram_write,
   input reg ram_op,
   output reg[15:0] ram_read,


   output reg[15:0] phy_ram_addr,
   output reg[15:0] phy_ram_write,
   output reg phy_ram_op,
   input reg[15:0] phy_ram_read,

   output reg[7:0] par_output_port,
   output reg par_output_signal  
);
  // reg ram_mode;
  wire _unused_ok_ = &{1'b0, clk, 1'b1};
  
  reg[15:0] mmio_read;

  // reg[15:0] actual_address;
  // assign actual_address = ram_addr;

  assign phy_ram_addr = ram_addr[15] ? 0 : ram_addr;
  assign phy_ram_op = ram_addr[15] ? 0 : ram_op;
  assign phy_ram_write = ram_write;
  assign ram_read = ram_addr[15] ? mmio_read : phy_ram_read;

  always @* begin
    mmio_read = 0;
    // $display("MMIO READ:  %d %d", ram_addr, test_arg);
    case(ram_addr[11:0]) 
      12'b0001_0000_0000: mmio_read[7:0] = par_output_port;
      12'b0001_0000_0001: mmio_read = {16{par_output_signal}};
      default: ;
      
    endcase
  end

  always @(posedge clk) begin
    if(ram_op == 1 && ram_addr[15] == 1) begin
      // $display("MMIO WRITE: %d %d", ram_addr, ram_write);
      case(ram_addr[11:8]) 
        4'b0001: begin
          case(ram_addr[7:0])
            8'b0000_0000: begin
              par_output_port <= ram_write[7:0];
            end

            8'b0000_0001: begin
              par_output_signal <= ram_write[0];
            end

            default;
          endcase

        end
        default: ;
        
      endcase
    end
  end

endmodule
