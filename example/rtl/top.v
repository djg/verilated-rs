//  Copyright (C) 2018 Dan Glastonbury <dan.glastonbury@gmail.com>

module top(
    clk_i,
    rst_i,
    count_o
);

    input clk_i;
    input rst_i;
    output [3:0] count_o;
   
    // 4-bit counter
    counter uut(
        .clk_i(clk_i),
        .rst_i(rst_i),
        .count_o(count_o)
    );
   
endmodule
