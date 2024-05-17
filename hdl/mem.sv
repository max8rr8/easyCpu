
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

   output reg serial_output_port,
   input reg serial_input_port,

   output reg keep_running
);
  // reg ram_mode;
  wire _unused_ok_ = &{1'b0, clk, 1'b1};
  
  reg[15:0] mmio_read;

  reg[127:0] serial_inp_buf, serial_out_buf;
  reg[6:0] cur_inp_pos, read_inp_pos;
  reg[6:0] write_out_pos, cur_out_pos;

  reg[3:0] serial_clock, inp_posclock, clock_diff;
  reg should_send;

  assign should_send = cur_out_pos != write_out_pos; 
  assign clock_diff = serial_clock - inp_posclock;

  assign phy_ram_addr = ram_addr[15] ? 0 : ram_addr;
  assign phy_ram_op = ram_addr[15] ? 0 : ram_op;
  assign phy_ram_write = ram_write;
  assign ram_read = ram_addr[15] ? mmio_read : phy_ram_read;


  initial begin
    serial_clock = 4'b0000;
    cur_inp_pos = 0;
    read_inp_pos = 0;
    
    cur_out_pos = 0;
    write_out_pos = 0;
    
    serial_inp_buf = 0;
    serial_out_buf = 0;
  end


  always @(posedge serial_input_port) begin
    inp_posclock <= serial_clock;
  end


  always @(negedge serial_input_port) begin
    if(clock_diff < 8) begin
      serial_inp_buf[cur_inp_pos] <= 0;
    end else begin
      serial_inp_buf[cur_inp_pos] <= 1;
    end
    cur_inp_pos <= cur_inp_pos + 1;


  end

  always @(posedge clk) begin
    serial_clock <= serial_clock + 1;

    if(serial_clock == 0) begin
      // if(should_send)  
      //   $display("Start_Send", cur_out_pos, " ", write_out_pos);
      if(should_send) begin
        serial_output_port <= 1;
        cur_out_pos <= cur_out_pos + 1;
      end
    end

    if(serial_clock == 4) begin
      // If we are started to send something, send value
      serial_output_port <= serial_output_port ? serial_out_buf[cur_out_pos - 1] : 0;
    end

    if(serial_clock == 12) begin
      serial_output_port <= 0;

    end
  end

  always @* begin
    mmio_read = 0;
    // $display("MMIO READ:  %d %d", ram_addr, test_arg);

    case(ram_addr[11:8]) 
        4'b0001: begin // 1111 0001 00mm XXXX
          case(ram_addr[3:0])
            4'b0000: mmio_read[6:0] = cur_inp_pos;
            4'b0001: mmio_read[6:0] = read_inp_pos;
            4'b0010: begin
              for(reg[6:0] i = 0; i <= 7'd15; i++ )
                mmio_read[(4'd15 - i[3:0])] = serial_inp_buf[read_inp_pos + i];
            end

            4'b0100: mmio_read[6:0] = cur_out_pos;
            4'b0101: mmio_read[6:0] = write_out_pos;
            4'b0110: begin
              // $display("WRITE");
              for(reg[6:0] i = 0; i <= 7'd15; i++ )
                mmio_read[(4'd15 - i[3:0])] = serial_out_buf[write_out_pos + i];
            end
            4'b0111: begin
              // $display("WRITE");
              for(reg[6:0] i = 0; i <= 7'd7; i++ )
                mmio_read[(4'd7 - i[3:0])] = serial_out_buf[write_out_pos + i];
            end
            default;
          endcase
        end
        
      // 12'b0001_0000_0000: mmio_read[3:0] = send_inp_pos;
      // 12'b0001_0000_0001: mmio_read[3:0] = cur_inp_pos;

      // 12'b0001_0000_0001: mmio_read = {16{serial_output_signal}};
      default: ;
    endcase
  end

  always @(posedge clk) begin
    if(ram_op == 1 && ram_addr[15] == 1) begin
      // $display("MMIO WRITE: %d %d", ram_addr, ram_write);
      case(ram_addr[11:8]) 
        4'b0001: begin
          case(ram_addr[3:0])
            4'b0001: read_inp_pos <= ram_write[6:0];
            4'b0101: write_out_pos <= ram_write[6:0];
            4'b0110: begin
              for(reg[6:0] i = 0; i <= 7'd15; i++ )
                serial_out_buf[write_out_pos + i] <= ram_write[(4'd15 - i[3:0])]; 
            end

            4'b0111: begin
              // $display("WRITE");
              for(reg[6:0] i = 0; i <= 7'd7; i++ )
                serial_out_buf[write_out_pos + i] <= ram_write[(4'd7 - i[3:0])];
            end
            
            default;
          endcase
        end
          // case(ram_addr[7:0])
           //   8'b0000_0000: begin
          //     serial_output_port <= ram_write[7:0];
          //   end

          //   8'b0000_0001: begin
          //     serial_output_signal <= ram_write[0];
          //   end

          //   default;
          // endcase

        4'b1111: begin
           if(ram_addr[7:0] == 8'b1111_1111) begin
              keep_running <= ram_write[0];
            end
        end

        default: ;
      endcase
    end
  end

endmodule
