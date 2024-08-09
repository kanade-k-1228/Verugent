#[macro_use]
extern crate verugent;

use verugent::vcore::*;

fn main() {
    let mut m = VModule::new("uart");

    let rst = m.input("rst", 1);

    let txclk = m.input("txclk", 1);
    let ldtxdata = m.input("ld_tx_data", 1);
    let txdata = m.input("tx_data", 8);
    let txen = m.input("tx_enable", 1);
    let txo = m.reg_out("tx_out", 1);
    let txemp = m.reg_out("tx_empty", 1);

    let rxclk = m.input("rxclk", 1);
    let ulrxdata = m.input("uld_rx_data", 1);
    let rxdata = m.reg_out("rx_data", 8);
    let rxen = m.input("rx_enable", 1);
    let rxin = m.input("rx_in", 1);
    let rxemp = m.reg_out("rx_empty", 0);

    let txreg = m.reg("tx_reg", 8);
    let txovrn = m.reg("rx_over_run", 0);
    let txcnt = m.reg("tx_cnt", 4);

    let rxreg = m.reg("rx_reg", 8);
    let rxsmpl = m.reg("rx_sample_cnt", 4);
    let rxcnt = m.reg("rx_cnt", 4);
    let rxfrerr = m.reg("rx_frame_err", 0);
    let rxovrn = m.reg("rx_over_run", 0);

    let rxd1 = m.reg("rx_d1", 1);
    let rxd2 = m.reg("rx_d2", 1);

    let rxbsy = m.reg("rx_busy", 1);

    m.always(
        posedge(rxclk)
            .posedge(&rst)
            .non()
            .if_(
                &rst,
                Form(F!(rxreg = 0))
                    .Form(F!(rxdata = 0))
                    .Form(F!(rxsmpl = 0))
                    .Form(F!(rxcnt = 0))
                    .Form(F!(rxfrerr = 0))
                    .Form(F!(rxovrn = 0))
                    .Form(F!(rxemp = 1))
                    .Form(F!(rxd1 = 1))
                    .Form(F!(rxd2 = 1))
                    .Form(F!(rxbsy = 0)),
            )
            .else_(
                Form(F!(rxd1 = rxin))
                    .Form(F!(rxd2 = rxd1))
                    .Form(If(ulrxdata, Form(F!(rxdata = rxreg)).Form(F!(rxemp = 1))))
                    .Form(If(
                        &rxen,
                        Form(If(
                            !rxbsy.land(rxd2.not()),
                            Form(F!(rxbsy = 1)).Form(F!(rxsmpl = 1)).Form(F!(rxcnt = 0)),
                        ))
                        .Form(If(
                            &rxbsy,
                            Form(rxsmpl.sst(&rxsmpl + 1)).Form(If(
                                F!(rxsmpl == 7),
                                Form(
                                    If(F!(rxd2 == 1).land(F!(rxcnt == 0)), Form(F!(rxbsy = 0)))
                                        .Else(
                                            Form(rxcnt.sst(&rxcnt + 1))
                                                .Form(If(
                                                    F!(rxcnt > 0).land(F!(rxcnt < 9)),
                                                    Form(rxreg.addr(&rxcnt - 1).sst(&rxd2)),
                                                ))
                                                .Form(If(
                                                    F!(rxcnt == 9),
                                                    Form(F!(rxbsy = 0)).Form(
                                                        If(F!(rxd2 == 0), Form(F!(rxfrerr = 1)))
                                                            .Else(
                                                                Form(F!(rxemp = 0))
                                                                    .Form(F!(rxfrerr = 0))
                                                                    .Form(F!(rxovrn = !rxemp)),
                                                            ),
                                                    ),
                                                )),
                                        ),
                                ),
                            )),
                        )),
                    )),
            )
            .if_(rxen.not(), Form(F!(rxbsy = 0))),
    );

    m.always(
        posedge(txclk)
            .posedge(&rst)
            .non()
            .if_(
                rst,
                Form(F!(txreg = 0))
                    .Form(F!(txemp = 1))
                    .Form(F!(txovrn = 0))
                    .Form(F!(txo = 1))
                    .Form(F!(txcnt = 0)),
            )
            .else_(
                Form(If(
                    ldtxdata,
                    Form(
                        If(txemp.not(), Form(F!(txovrn = 0)))
                            .Else(Form(F!(txreg = txdata)).Form(F!(txemp = 0))),
                    ),
                ))
                .Form(If(
                    txen.land(txemp.not()),
                    Form(txcnt.sst(&txcnt + 1))
                        .Form(If(F!(txcnt == 0), Form(F!(txo = 0))))
                        .Form(If(
                            F!(txcnt > 0).land(F!(txcnt < 9)),
                            Form(txo.sst(txreg.addr(&txcnt + 1))),
                        ))
                        .Form(If(
                            F!(txcnt == 9),
                            Form(F!(txo = 1)).Form(F!(txcnt = 0)).Form(F!(txemp = 1)),
                        )),
                ))
                .Form(If(txen.not(), Form(F!(txcnt = 0)))),
            ),
    );
    println!("{}", m.gen());
}
