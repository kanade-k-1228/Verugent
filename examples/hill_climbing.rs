#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    let mut m = VModule::new("hill_climbing");

    let clk = m.input("CLK", 1);
    let rst = m.input("RST", 1);

    let start = m.input("Start", 1);
    let data = m.input("Data", 32);
    let ready = m.output("Ready", 1);
    let busy = m.output("Busy", 1);
    let done = m.output("Done", 1);

    let evalfuncdata = m.reg_out("Eval", 32);
    let evalstart = m.reg_out("EvalS", 1);
    let evalfuncout = m.input("EvalOut", 32);
    let evalvalid = m.input("Evalvalid", 1);

    let result = m.output("Result", 32);

    let bestdata = m.reg("Bestdata", 32);
    let besteval = m.reg("Besteval", 32);

    let nextdata = m.reg("Nextdata", 32);
    let nexteval = m.reg("Nexteval", 32);

    let neighborcount = m.reg("Neighbor_Count", 32);
    let tmpeval = m.reg("Tmpeval", 32);

    let currentnode = m.reg("CurrentNode", 32);

    let neighborset = m.wire("NBWire", 32);

    m.assign(neighborset._e(&neighborcount + &currentnode));
    m.assign(result._e(&bestdata));

    let mut fsm = FsmModule::new(&clk, &rst, "state")
        .AddState("IDLE")
        .goto("INIT", F!(start == 1))
        .AddState("INIT")
        .goto("NINIT", F!(evalvalid == 1))
        .AddState("NINIT")
        .goto("UPDATE_NEXT1", Blank!())
        .AddState("UPDATE_NEXT1")
        .goto("UPDATE_NEXT2", F!(evalvalid == 1))
        .AddState("UPDATE_NEXT2")
        .goto("UPDATE_NEXT1", Blank!())
        .goto("POSTPROCESS", F!(neighborcount > 1))
        .AddState("POSTPROCESS")
        .goto("NINIT", Blank!())
        .goto("END", F!(nexteval <= besteval))
        .AddState("END")
        .goto("IDLE", Blank!());

    let s_idle = fsm.Param("IDLE");
    let s_init = fsm.Param("INIT");
    let s_ninit = fsm.Param("NINIT");
    let s_upd_n1 = fsm.Param("UPDATE_NEXT1");
    let s_upd_n2 = fsm.Param("UPDATE_NEXT2");
    let s_pproc = fsm.Param("POSTPROCESS");
    let s_end = fsm.Param("END");

    let fstate = m.fsm(fsm);

    m.always(
        posedge(clk)
            .posedge(&rst)
            .block()
            .if_(
                rst,
                Form(F!(evalfuncdata = 0))
                    .Form(F!(bestdata = 0))
                    .Form(F!(besteval = 0))
                    .Form(F!(evalstart = 0))
                    .Form(F!(nextdata = 0))
                    .Form(F!(nexteval = 0))
                    .Form(F!(neighborcount = 0))
                    .Form(F!(tmpeval = 0))
                    .Form(F!(currentnode = 0)),
            )
            .else_(Form(
                If(
                    F!(fstate == s_init),
                    Form(F!(bestdata = data))
                        .Form(F!(evalfuncdata = data))
                        .Form(F!(evalstart = 1))
                        .Form(F!(besteval = evalfuncout))
                        .Form(F!(currentnode = data)),
                )
                .Else_If(
                    F!(fstate == s_ninit),
                    Form(F!(nextdata = 0))
                        .Form(F!(nexteval = 0))
                        .Form(F!(neighborcount = 0))
                        .Form(F!(evalstart = 0)),
                )
                .Else_If(
                    F!(fstate == s_upd_n1),
                    Form(F!(evalfuncdata = neighborset))
                        .Form(F!(evalstart = 1))
                        .Form(F!(tmpeval = evalfuncout))
                        .Form(F!(currentnode = nextdata)),
                )
                .Else_If(
                    F!(fstate == s_upd_n2),
                    Form(If(
                        F!(nexteval < tmpeval),
                        Form(F!(nexteval = tmpeval))
                            .Form(F!(nextdata = evalfuncdata))
                            .Form(neighborcount.sst(&neighborcount + 1)),
                    )),
                )
                .Else_If(
                    F!(fstate == s_pproc),
                    Form(If(F!(nextdata > bestdata), Form(F!(bestdata = nextdata)))),
                ),
            )),
    );

    m.assign(ready._e(F!(fstate == s_idle)));
    m.assign(done._e(F!(fstate == s_end)));
    m.assign(busy._e(F!(fstate != s_idle).land(F!(fstate != s_end))));

    println!("{}", m.gen());
}
