//#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    let mut al = VModule::new("axi_interface");
    let clk = al.input("clk", 0);
    let rst = al.input("rst", 0);

    let mut axi = AxiLite::new(clk, rst);
    axi.named_reg_set("calc_a");
    axi.named_reg_set("calc_b");
    axi.named_reg_set("output_calc");

    let a = al.output("o_A", 32);
    let b = al.output("o_B", 32);
    al.assign(a._e(axi.named_reg("Calc_A")));
    al.assign(b._e(axi.named_reg("Calc_B")));

    let w = al.wire("write_en_cdata", 0);
    al.assign(w._e(_Num(1)));

    let calc = al.input("i_Calc", 32);
    axi.reg_write(w, calc);

    al.axi(axi);

    println!("{}", al.gen());
}
