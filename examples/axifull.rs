extern crate verugent;

use verugent::vcore::*;

fn main() {
    let mut al = VModule::new("axi_full_interface");
    let clk = al.input("clk", 0);
    let rst = al.input("rst", 0);

    let mut axi = Axi4Slave::new(clk, rst);
    axi.order_reg_set(64);
    al.axi(axi);

    println!("{}", al.gen());
}
