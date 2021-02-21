// Copyright 2018 - 2021, Dan Glastonbury <dan.glastonbury@gmail.com> and the
// verilated-rs contributors.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

module counter(
    clk_i,
    rst_i,
    count_o
);

    input clk_i;
    input rst_i;

    output [3:0] count_o;
    reg [3:0]    count_o;
   
    always @(posedge clk_i)
      begin
        if (rst_i == 1'b1) begin
            count_o <= 4'b0000;
        end
        else begin
            count_o <= count_o + 1;
        end
    end
   
endmodule
