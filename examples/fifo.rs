#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    let mut m = VModule::new("FIFO");
    let width = m.add_io_param("width", 8);
    let widthad = m.add_io_param("widthad", 9);
    let words = m.add_io_param("numwords", 512);

    let clk = m.input("CLK", 1);
    let rst = m.input("RST", 1);
    let d = m.input("D", &width);
    let q = m.output("Q", &width);
    let wr = m.input("WR", 1);
    let rd = m.input("RD", 1);
    let full = m.output("FULL", 1);
    let empty = m.output("EMPTY", 1);

    let cnt = m.wire("CNT", 10);
    let wp = m.wire("WP", &widthad);
    let rp = m.wire("RP", &widthad);

    let wcnt = m.reg("WCNT", 9);
    let rcnt = m.reg("RCNT", 9);
    let data = m.Mem("DATA", (width, words));

    m.assign(q._e(data.addr(&rp)));
    m.assign(cnt._e(&wcnt - &rcnt));
    m.assign(full._e(cnt.addr(&widthad)));
    m.assign(wp._e(wcnt.addr(&widthad - 1)));
    m.assign(rp._e(rcnt.addr(&widthad - 1)));

    m.always(
        posedge(clk)
            .posedge(&rst)
            .non()
            .if_(rst, Form(F!(wcnt = 0)).Form(F!(rcnt = 0)))
            .else_(vec![
                If(
                    wr & !full,
                    Form(data.addr(wp).sst(d)).Form(F!(wcnt = (&wcnt + 1))),
                ),
                If(rd & !empty, Form(F!(rcnt = (&rcnt + 1)))),
            ]),
    );
    println!("{}", m.gen());
}
