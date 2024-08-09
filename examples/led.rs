#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    let mut module = VModule::new("LED");
    let clk = module.input("clk", 1);
    let rst = module.input("rst", 1);
    let btn1 = module.input("i_btn1", 1);
    let btn2 = module.input("i_btn2", 1);
    let led = module.output("o_led", 8);
    let mut fsm = FsmModule::new(clk, &rst, "state")
        .AddState("IDLE")
        .goto("RUN", F!(btn1 == 1).land(F!(rst != 1)))
        .AddState("RUN")
        .goto("END", F!(btn2 == 1))
        .AddState("END")
        .goto("IDLE", Blank!());
    let run = fsm.Param("RUN");
    let fstate = module.fsm(fsm);
    module.assign(led._e(_Branch(F!(fstate == run), _Num(255), _Num(0))));
    println!("{}", module.gen());
}
