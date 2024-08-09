#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    let mut m = VModule::new("functest");

    let ina = m.input("ina", 32);
    let inb = m.input("inb", 32);

    let out = m.output("outs", 0);

    let mut f = Func::new("testf", 32);

    let ia = f.Input("ia", 32);
    let ib = f.Input("ib", 32);

    f.If(F!(ia == ib), Form(f.clone().own().sst(1)));
    f.Else(Form(f.clone().own().sst(0)));
    m.func(f.clone());

    m.assign(out._e(f.using(func_args!(ina, inb))));

    m.gen();
    println!("{}", m.gen());
}
