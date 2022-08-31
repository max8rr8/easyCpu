verilator --top-module our -Wall --cc --exe --build sim_main.cpp hdl/our.sv -I hdl/mem.sv -I hdl/inst.sv -I hdl/reg_read.sv &&

echo "+++++++++++++++++++++" && obj_dir/Vour $@ 