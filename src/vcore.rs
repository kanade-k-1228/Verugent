#![allow(dead_code)]
#![allow(non_snake_case)]
use std::ops::*;
use std::string::String;
use std::*;

#[macro_export]
macro_rules! F {
    ($lhs:ident == $rhs:expr) => {
        ($lhs.clone()).eq($rhs.clone())
    };

    ($lhs:ident != $rhs:expr) => {
        ($lhs.clone()).ne($rhs.clone())
    };

    ($lhs:ident <= $rhs:expr) => {
        ($lhs.clone()).ge($rhs.clone())
    };

    ($lhs:ident < $rhs:expr) => {
        ($lhs.clone()).gt($rhs.clone())
    };

    ($lhs:ident >= $rhs:expr) => {
        ($lhs.clone()).le($rhs.clone())
    };

    ($lhs:ident > $rhs:expr) => {
        ($lhs.clone()).lt($rhs.clone())
    };

    ($lhs:ident = $rhs:expr) => {
        ($lhs.clone()).sst($rhs.clone())
    };

    ( $lhs:ident || $rhs:expr ) => {
        $lhs.clone().lor($rhs.clone())
    };

    ( $lhs:ident && $rhs:expr ) => {
        $lhs.clone().land($rhs.clone())
    };
}

#[derive(Clone, Debug)]
pub struct VModule {
    name: String,
    io_port: Vec<WireVar>,
    io_param: Vec<WireVar>,
    local_param: Vec<WireVar>,
    always: Vec<Always>,
    assign: Vec<Assign>,
    func: Vec<Func>,
    fsm: Vec<FsmModule>,
    axi: Vec<Bus>,
    inline: String,
}

pub trait VSet<T> {
    fn input(&mut self, name: &str, width: T) -> Box<E>;
    fn inout(&mut self, name: &str, width: T) -> Box<E>;
    fn output(&mut self, name: &str, width: T) -> Box<E>;
    fn reg_out(&mut self, name: &str, width: T) -> Box<E>;
    fn wire(&mut self, name: &str, width: T) -> Box<E>;
    fn reg(&mut self, name: &str, width: T) -> Box<E>;
}

impl<T> VSet<T> for VModule
where
    T: Into<Box<E>>,
{
    fn input(&mut self, name: &str, width: T) -> Box<E> {
        let mut tmp = WireVar::new();
        let width = *width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.input(name, len);
        if let E::Ldc(wr) = width {
            tmp.width(&(&wr.width_p));
        };
        self.io_port.push(tmp.clone());
        return _V(tmp);
    }

    fn inout(&mut self, name: &str, width: T) -> Box<E> {
        let mut tmp = WireVar::new();
        let width = *width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.inout(name, len);
        if let E::Ldc(wr) = width {
            tmp.width(&(&wr.width_p));
        };
        self.io_port.push(tmp.clone());
        return _V(tmp);
    }

    fn output(&mut self, name: &str, width: T) -> Box<E> {
        let mut tmp = WireVar::new();
        let width = *width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.output(name, len);
        if let E::Ldc(wr) = width {
            tmp.width(&(&wr.width_p));
        };
        self.io_port.push(tmp.clone());
        return _V(tmp);
    }

    fn reg_out(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = WireVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.output_reg(name, len);
        if let E::Ldc(wr) = width {
            tmp.width(&(&wr.width_p));
        };
        self.io_port.push(tmp.clone());
        return _V(tmp);
    }

    fn wire(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = WireVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.wire(name, len);
        if let E::Ldc(wr) = width {
            tmp.width(&(&wr.width_p));
        };
        self.local_param.push(tmp.clone());
        return _V(tmp);
    }

    fn reg(&mut self, name: &str, width: T) -> Box<E> {
        let mut tmp = WireVar::new();
        let width = *width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.reg(name, len);
        if let E::Ldc(wr) = width {
            tmp.width(&(&wr.width_p));
        };
        self.local_param.push(tmp.clone());
        return _V(tmp);
    }
}

impl VModule {
    /// モジュールの生成
    pub fn new(Name: &str) -> VModule {
        VModule {
            name: Name.to_string(),
            io_port: Vec::new(),
            io_param: Vec::new(),
            local_param: Vec::new(),
            always: Vec::new(),
            assign: Vec::new(),
            func: Vec::new(),
            fsm: Vec::new(),
            axi: Vec::new(),
            inline: String::new(),
        }
    }

    /// パラメータの追加
    pub fn add_io_param(&mut self, name: &str, value: i32) -> Box<E> {
        let mut tmp = WireVar::new();
        tmp.parameter(name, value);
        self.io_param.push(tmp.clone());
        return _V(tmp);
    }

    /// ローカルパラメータの追加
    pub fn add_local_param(&mut self, name: &str, Value: i32) -> Box<E> {
        let mut tmp = WireVar::new();
        tmp.parameter(name, Value);
        self.local_param.push(tmp.clone());
        return _V(tmp);
    }

    /// always 構文ブロックの追加
    pub fn always(&mut self, always: Always) {
        self.always.push(always.clone())
    }

    /// assign 構文 AST の追加
    pub fn assign(&mut self, assign: Assign) {
        self.assign.push(assign.clone())
    }

    /// モジュールの AST 解析と Verilog 構文の出力
    pub fn gen(&self) -> String {
        let mut st = String::new();
        st += &format!("module {} ", self.name);
        st += &WireVar::print_params(&self.io_param);
        st += &WireVar::print_ports(&self.io_port);
        st += &WireVar::print_local_params(&self.local_param);
        st += &Assign::print_list(&self.assign);
        st += &Always::print_list(&self.always);
        st += &Func::print_list(&self.func);

        if self.fsm.len() != 0 || self.axi.len() != 0 || self.inline.len() != 0 {
            st += &self
                .fsm
                .iter()
                .map(|fsm| fsm.print())
                .collect::<Vec<_>>()
                .join("\n");
            st += &self
                .axi
                .iter()
                .enumerate()
                .map(|(i, axi)| print_axi(axi.clone(), i as i32))
                .collect::<Vec<_>>()
                .join("");
            st += &self.inline;
        }

        st += "\nendmodule\n";

        return st;
    }

    /// Inline verilog
    pub fn inline(&mut self, code: &str) {
        self.inline += code;
        self.inline += "\n\n";
    }

    pub fn out_func_name(&mut self) -> Vec<String> {
        let mut st = Vec::new();
        let tmp = self.func.clone();
        for x in tmp {
            let e = x.top;
            if let E::Ldc(wrtop) = (*e).clone() {
                st.push(wrtop.name);
            }
        }
        st
    }

    pub fn out_assign(&mut self) -> Vec<Assign> {
        self.assign.clone()
    }

    pub fn out_always(&mut self) -> Vec<Always> {
        self.always.clone()
    }
}

/// function 構文ブロック追加用トレイト
pub trait FuncTrait<T> {
    fn func(&mut self, AST_of_Function: T);
}

impl FuncTrait<Func> for VModule {
    fn func(&mut self, AST_of_Function: Func) {
        self.func.push(AST_of_Function);
    }
}

impl FuncTrait<&Func> for VModule {
    fn func(&mut self, AST_of_Function: &Func) {
        self.func.push(AST_of_Function.clone());
    }
}

pub trait FSMTrait<T> {
    fn fsm(&mut self, fsm: T) -> Box<E>;
}

impl FSMTrait<FsmModule> for VModule {
    fn fsm(&mut self, fsm: FsmModule) -> Box<E> {
        let self_fsm = self.fsm.clone();
        for n in self_fsm {
            if _StrOut(n.clone().state_reg) == _StrOut(fsm.clone().state_reg) {
                panic!("Some name FSM exist. :{}\n", _StrOut(fsm.clone().state_reg))
            }
        }

        let tmp = fsm.clone();
        let mut stmt = fsm.StateOut();
        let state = *(tmp.clone().StateReg());
        let p;
        let mut np = WireVar::new();
        let mut n = 0;
        for ss in &mut stmt {
            self.local_param.push(WireVar {
                name: ss.getStateName(),
                io_param: IOType::Param,
                width: 0,
                length: 0,
                reg_set: false,
                value: n,
                width_p: "_".to_string(),
                length_p: "_".to_string(),
            });
            n += 1;
        }

        if let E::Ldc(x) = state {
            p = x.clone();
            let nam = p.name.clone() + "_Next";
            if let E::Ldc(wr) = *WireVar::new().reg(&nam, 32) {
                np = wr;
            }
        } else {
            return Box::new(E::Null);
        }
        self.local_param.push(p);
        self.local_param.push(np);
        self.fsm.push(tmp.clone());

        return tmp.StateReg();
    }
}

impl FSMTrait<&FsmModule> for VModule {
    fn fsm(&mut self, fsm: &FsmModule) -> Box<E> {
        let self_fsm = self.fsm.clone();
        for n in self_fsm {
            if _StrOut(n.clone().state_reg) == _StrOut(fsm.clone().state_reg) {
                panic!("Some name FSM exist. :{}\n", _StrOut(fsm.clone().state_reg))
            }
        }

        let tmp = fsm.clone();
        let retE = fsm.clone().StateReg();
        let mut stmt = fsm.clone().StateOut();
        let state = *(tmp.clone().StateReg());
        let p;
        let mut np = WireVar::new();
        let mut n = 0;
        for ss in &mut stmt {
            self.local_param.push(WireVar {
                name: ss.getStateName(),
                io_param: IOType::Param,
                width: 0,
                length: 0,
                reg_set: false,
                value: n,
                width_p: "_".to_string(),
                length_p: "_".to_string(),
            });
            n += 1;
        }

        if let E::Ldc(x) = state {
            p = x.clone();
            let nam = p.name.clone() + "_Next";
            if let E::Ldc(wr) = *WireVar::new().reg(&nam, 32) {
                np = wr;
            }
        } else {
            return Box::new(E::Null);
        }
        self.local_param.push(p);
        self.local_param.push(np);
        self.fsm.push(tmp);

        return retE;
    }
}

pub trait AXITrait<T> {
    fn axi(&mut self, setAXI: T);
}

impl AXITrait<AxiLite> for VModule {
    fn axi(&mut self, setAXI: AxiLite) {
        let length = self.axi.len() as i32;

        let reg_length = setAXI.reg_array.len() as i32;
        let mut reg_addr_width: i32 = 1;

        // address width calc
        loop {
            if 2i32.pow(reg_addr_width as u32) >= (reg_length * 4 - 1) {
                break;
            }
            reg_addr_width += 1;
        }

        // read address channel
        let o_arr = self.output(&(format!("o_s_arready{}", length.clone())), 0);
        let i_arv = self.input(&(format!("i_s_arvalid{}", length.clone())), 0);
        self.input(&(format!("i_s_araddr{}", length.clone())), reg_addr_width);
        self.input(&(format!("i_s_arprot{}", length.clone())), 3);

        // read data channel
        let o_rda = self.output(&(format!("o_s_rdata{}", length.clone())), 32);
        let o_rsp = self.output(&(format!("o_s_rresp{}", length.clone())), 2);
        let o_rva = self.output(&(format!("o_s_rvalid{}", length.clone())), 0);
        let i_rre = self.input(&(format!("i_s_rready{}", length.clone())), 0);

        // write address channel
        let o_awr = self.output(&(format!("o_s_awready{}", length.clone())), 0);
        let i_awv = self.input(&(format!("i_s_awvalid{}", length.clone())), 0);
        self.input(&(format!("i_s_awaddr{}", length.clone())), reg_addr_width);
        self.input(&(format!("i_s_awprot{}", length.clone())), 3);

        // write data channel
        let i_wda = self.input(&(format!("i_s_wdata{}", length.clone())), 32);
        let i_wst = self.input(&(format!("i_s_wstrb{}", length.clone())), 4);
        let i_wva = self.input(&(format!("i_s_wvalid{}", length.clone())), 0);
        let o_wre = self.output(&(format!("o_s_wready{}", length.clone())), 0);

        // write response channel
        let o_bre = self.output(&(format!("o_s_bresp{}", length.clone())), 2);
        let o_bva = self.output(&(format!("o_s_bvalid{}", length.clone())), 0);
        let i_bre = self.input(&(format!("i_s_bready{}", length.clone())), 0);

        // inner wire and register
        let r_arr = self.reg(&(format!("r_arready{}", length.clone())), 0);
        let w_arv = self.wire(&(format!("w_arvalid{}", length.clone())), 0);
        self.reg(&(format!("r_araddr{}", length.clone())), reg_addr_width);

        let r_rda = self.reg(&(format!("r_rdata{}", length.clone())), 32);
        let r_rva = self.reg(&(format!("r_rvalid{}", length.clone())), 0);
        let w_rre = self.wire(&(format!("w_rready{}", length.clone())), 0);

        let r_awr = self.reg(&(format!("r_awready{}", length.clone())), 0);
        let w_awv = self.wire(&(format!("w_awvalid{}", length.clone())), 0);
        self.reg(&(format!("r_awaddr{}", length.clone())), reg_addr_width);

        let w_wda = self.wire(&(format!("w_wdata{}", length.clone())), 32);
        let w_wst = self.wire(&(format!("r_wstrb{}", length.clone())), 4);
        let w_wva = self.wire(&(format!("w_wvalid{}", length.clone())), 0);
        let r_wre = self.reg(&(format!("r_wready{}", length.clone())), 0);

        let r_bva = self.reg(&(format!("r_bvalid{}", length.clone())), 0);
        let w_bre = self.wire(&(format!("w_bready{}", length.clone())), 0);

        // 接続の追加
        self.assign(o_arr._e(r_arr));
        self.assign(w_arv._e(i_arv));

        self.assign(o_rda._e(r_rda));
        self.assign(o_rsp._e(0));
        self.assign(o_rva._e(r_rva));
        self.assign(w_rre._e(i_rre));

        self.assign(o_awr._e(r_awr));
        self.assign(w_awv._e(i_awv));
        //self.Assign(w_awa._e(i_awa));

        self.assign(w_wda._e(i_wda));
        self.assign(w_wst._e(i_wst));
        self.assign(w_wva._e(i_wva));
        self.assign(o_wre._e(r_wre));

        self.assign(o_bre._e(0));
        self.assign(o_bva._e(r_bva));
        self.assign(w_bre._e(i_bre));

        for x in setAXI.reg_array.clone() {
            //println!("{:?}", (*x));
            if let E::Ldc(wr) = *x {
                self.reg(&(wr.name), wr.width);
            };
        }

        self.axi.push(Bus::AxiLite(setAXI));
    }
}

impl AXITrait<Axi4Slave> for VModule {
    fn axi(&mut self, setAXI: Axi4Slave) {
        let length = setAXI.length.clone();

        let mut addr_width: i32 = 1;
        loop {
            if 2i32.pow(addr_width as u32) >= (length * 4 - 1) {
                break;
            }
            addr_width += 1;
        }

        // read address channel
        let i_rid = self.input("i_saxi_arid", 0);
        let o_arr = self.output("o_saxi_arready", 0);
        let i_arv = self.input("i_saxi_arvalid", 0);
        self.input("i_saxi_araddr", addr_width);
        self.input("i_saxi_arlen", 8);
        self.input("i_saxi_arburst", 2);

        // read data channel
        let o_rid = self.output("o_saxi_rid", 0);
        let o_rda = self.output("o_saxi_rdata", 32);
        let o_rsp = self.output("o_saxi_rresp", 2);
        let o_rva = self.output("o_saxi_rvalid", 0);
        let i_rre = self.input("i_saxi_rready", 0);
        let o_rls = self.output("o_saxi_rlast", 0);

        // write address channel
        let i_wid = self.input("i_saxi_awid", 0);
        let o_awr = self.output("o_saxi_awready", 0);
        let i_awv = self.input("i_saxi_awvalid", 0);
        self.input("i_saxi_awaddr", addr_width);
        self.input("i_saxi_awlen", 8);
        self.input("i_saxi_awburst", 2);

        // write data channel
        self.input("i_saxi_wdata", 32);
        self.input("i_saxi_wstrb", 4);
        let i_wls = self.input("i_saxi_wlast", 0);
        let i_wva = self.input("i_saxi_wvalid", 0);
        let o_wre = self.output("o_saxi_wready", 0);

        // write response channel
        let o_bid = self.output("o_saxi_bid", 0);
        let o_bre = self.output("o_saxi_bresp", 2);
        let o_bva = self.output("o_saxi_bvalid", 0);
        let i_bre = self.input("i_saxi_bready", 0);

        // inner wire and register
        let r_awr = self.reg("r_axi_awready", 0);
        let w_awv = self.wire("w_axi_awvalid", 0);
        self.reg("r_axi_awaddr", addr_width);
        self.reg("r_axi_awlen", 8);

        self.wire("w_axi_wdata", 32);
        let w_wls = self.wire("w_axi_wlast", 0);
        let w_wva = self.wire("w_axi_wvalid", 0);
        let r_wre = self.reg("r_axi_wready", 0);

        let r_bva = self.reg("r_axi_bvalid", 0);
        let w_bre = self.wire("w_axi_bready", 0);

        let r_arr = self.reg("r_axi_arready", 0);
        let w_arv = self.wire("w_axi_arvalid", 0);
        self.reg("r_axi_araddr", 32);
        self.reg("r_axi_arlen", 8);

        let r_rda = self.reg("r_axi_rdata", 32);
        let r_rva = self.reg("r_axi_rvalid", 0);
        let w_rre = self.wire("w_axi_rready", 0);
        let r_rls = self.reg("r_axi_rlast", 0);
        if setAXI.clone().mem {
            self.wire("axis_write", 32);
            self.reg("axis_read", 32);
            self.wire("axis_addr", 32);
            self.wire("axis_wen", 0);
        } else {
            if let E::Null = *(setAXI.clone().rdata) {
                self.wire("axis_read", 32);
            }
            self.wire("axis_write", 32);
            self.wire("axis_addr", 32);
            self.wire("axis_wen", 0);
        }

        self.assign(o_rid._e(i_rid));
        self.assign(o_rsp._e(0));
        self.assign(o_rda._e(r_rda));
        self.assign(o_rva._e(r_rva));
        self.assign(w_rre._e(i_rre));
        self.assign(o_rls._e(r_rls));

        self.assign(o_arr._e(r_arr));
        self.assign(w_arv._e(i_arv));

        self.assign(w_wls._e(i_wls));
        self.assign(w_wva._e(i_wva));
        self.assign(o_wre._e(r_wre));

        self.assign(o_awr._e(r_awr));
        self.assign(w_awv._e(i_awv));

        self.assign(o_bva._e(r_bva));
        self.assign(w_bre._e(i_bre));
        self.assign(o_bid._e(i_wid));
        self.assign(o_bre._e(0));

        self.axi.push(Bus::AxiSlave(setAXI));
    }
}

/// メモリレジスタ生成用のトレイト
pub trait Memset<T> {
    fn Mem(&mut self, name: &str, args: T) -> Box<E>;
}

/// 入力(Box<E>:Box<E>)生成するメモリ構文
impl<T, U> Memset<(T, U)> for VModule
where
    T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    /// メモリ構文

    fn Mem(&mut self, name: &str, args: (T, U)) -> Box<E> {
        let mut tmp = WireVar::new();
        tmp.mem(name, 0, 0);
        if let E::Ldc(wr) = *args.0.into() {
            tmp.width(&(wr.name));
        };
        if let E::Ldc(wr) = *args.1.into() {
            tmp.length(&(wr.name));
        };
        self.local_param.push(tmp.clone());
        return _V(tmp);
    }
}

/**
 * 入出力設定パラメータ
 * 特に大きな意味は無い
 **/

#[derive(Clone, Debug)]
pub enum IOType {
    Input,
    Output,
    InOut,
    Param,
    None,
}

/// 入出力ポート、パラメータデータ格納構造体

#[derive(Clone, Debug)]
pub struct WireVar {
    name: String,
    io_param: IOType,
    width: i32,
    length: i32,
    reg_set: bool,
    value: i32,
    width_p: String,
    length_p: String,
}

/**
 * 入出力パラメータクラスメソッド
 * セット・ゲット・コピー関数
 **/

impl WireVar {
    /// コンストラクタ
    pub fn new() -> WireVar {
        WireVar {
            name: "none".to_string(),
            io_param: IOType::None,
            width: 0,
            length: 0,
            reg_set: false,
            value: 0,
            width_p: "_".to_string(),
            length_p: "_".to_string(),
        }
    }

    /// パラメータによる長さ設定メソッド
    pub fn length(&mut self, S: &str) -> WireVar {
        self.length_p = S.to_string();
        self.clone()
    }

    /// パラメータによる幅設定メソッド
    pub fn width(&mut self, S: &str) -> WireVar {
        self.width_p = S.to_string();
        self.clone()
    }

    /// パラメータ設定メソッド:input
    pub fn input(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = IOType::Input;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:output
    pub fn output(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = IOType::Output;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:inout
    pub fn inout(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = IOType::InOut;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:output(register)
    pub fn output_reg(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();

        self.io_param = IOType::Output;
        self.width = Width;
        self.reg_set = true;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:parameter
    pub fn parameter(&mut self, Name: &str, Value: i32) -> Box<E> {
        self.name = Name.to_string();
        self.value = Value;

        self.io_param = IOType::Param;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:wire
    pub fn wire(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        _V(self.clone())
    }

    /// パラメータ設定メソッド:reg
    pub fn reg(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.reg_set = true;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:reg[ length : 0 ]
    pub fn mem(&mut self, Name: &str, Width: i32, Lenght: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;
        self.length = Lenght;

        self.reg_set = true;
        _V(self.clone())
    }

    pub fn print_as_param(&self) -> String {
        format!("parameter {} = {}", self.name, self.value)
    }

    pub fn print_params(params: &[WireVar]) -> String {
        if params.len() == 0 {
            return "".to_string();
        }
        let param_list = params
            .iter()
            .map(|param| format!("    {}", param.print_as_param()))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("#(\n{}\n)", param_list)
    }

    pub fn print_as_port(&self) -> String {
        let range = format!("[{}-1:0]", self.width);
        let array = format!("[{}-1:0]", self.length);
        match self.io_param {
            IOType::Input => format!("input  {} {} {}", range, self.name, array),
            IOType::Output => format!("output {} {} {}", range, self.name, array),
            IOType::InOut => format!("inout {} {} {}", range, self.name, array),
            IOType::Param => panic!(),
            IOType::None => panic!(),
        }
    }

    pub fn print_ports(ports: &[WireVar]) -> String {
        if ports.len() == 0 {
            return "".to_string();
        }
        let param_list = ports
            .iter()
            .map(|port| format!("    {}", port.print_as_port()))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("(\n{}\n);\n", param_list)
    }

    pub fn print_as_local_param(&self) -> String {
        format!("parameter {} = {};", self.name, self.value)
    }

    pub fn print_local_params(local_params: &[WireVar]) -> String {
        if local_params.len() == 0 {
            return String::new();
        }
        local_params
            .iter()
            .map(|param| format!("    {}\n", param.print_as_local_param()))
            .collect::<Vec<_>>()
            .join("")
    }
}

/// Assign 構文代入用トレイト
pub trait SetEqual<T>
where
    T: Into<Box<E>>,
{
    fn _e(&self, RHS: T) -> Assign;

    fn _ve(&self, RHS: T) -> Assign;
}

/// Assign 構文代入用トレイト
impl<T> SetEqual<T> for Box<E>
where
    T: Into<Box<E>>,
{
    /// Box<E>からAssign生成を行うメソッド
    fn _e(&self, RHS: T) -> Assign {
        let mut tmp = Assign::new();
        tmp.lhs(self).rhs(&RHS.into())
    }

    fn _ve(&self, RHS: T) -> Assign {
        let mut tmp = Assign::new();
        tmp.lhs(self).rhs(&RHS.into())
    }
}

#[derive(Clone, Debug)]
pub struct Assign {
    lhs: Box<E>,
    rhs: Box<E>,
}

impl Assign {
    pub fn new() -> Assign {
        Assign {
            lhs: Box::new(E::Ldc(WireVar::new())),
            rhs: Box::new(E::Ldc(WireVar::new())),
        }
    }

    pub fn lhs<T: Into<Box<E>>>(&mut self, lhs: T) -> Assign {
        self.lhs = lhs.into();
        self.clone()
    }

    pub fn rhs<T: Into<Box<E>>>(&mut self, rhs: T) -> Assign {
        self.rhs = rhs.into();
        self.clone()
    }

    fn print(&self) -> String {
        format!(
            "assign {} = {};",
            &decomp_ast(false, self.lhs.clone(), "", 0),
            &decomp_ast(false, self.rhs.clone(), "", 0)
        )
    }

    pub fn print_list(list: &[Assign]) -> String {
        if list.len() == 0 {
            return String::new();
        }
        list.iter()
            .map(|assign| format!("    {}\n", assign.print()))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[derive(Clone, Debug)]
pub struct Always {
    name: String,
    stmt: Vec<Box<E>>,
    posedges: Vec<WireVar>,
    negedges: Vec<WireVar>,
}

pub fn posedge<T: Into<Box<E>>>(edge: T) -> Always {
    let e = *edge.into();
    let mut tmp = Always {
        name: "block".to_string(),
        stmt: Vec::new(),
        posedges: Vec::new(),
        negedges: Vec::new(),
    };
    match e {
        E::Ldc(wr) => tmp.posedges.push(wr.clone()),
        _ => return tmp,
    }
    tmp.clone()
}

pub fn negedge<T: Into<Box<E>>>(edge: T) -> Always {
    let e = *edge.into();
    let mut tmp = Always {
        name: "block".to_string(),
        stmt: Vec::new(),
        posedges: Vec::new(),
        negedges: Vec::new(),
    };
    match e {
        E::Ldc(wr) => tmp.negedges.push(wr.clone()),
        _ => return tmp,
    }
    tmp.clone()
}

pub fn onedge() -> Always {
    Always {
        name: "block".to_string(),
        stmt: Vec::new(),
        posedges: Vec::new(),
        negedges: Vec::new(),
    }
}

impl Always {
    pub fn print(&self) -> String {
        let pos = self.posedges.iter().map(|p| format!("posedge {}", p.name));
        let neg = self.negedges.iter().map(|n| format!("negedge {}", n.name));
        let list = pos.chain(neg).collect::<Vec<_>>().join(" or ");
        let body = self
            .stmt
            .iter()
            .map(|stmt| decomp_ast(false, stmt.clone(), &self.clone().blockout(), 2))
            .collect::<Vec<_>>()
            .join("");
        vec![
            format!("    always @({}) begin", list),
            body,
            format!("    end"),
        ]
        .join("\n")
    }

    pub fn print_list(list: &[Self]) -> String {
        if list.len() == 0 {
            return String::new();
        }
        list.iter()
            .map(|always| always.print())
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Always {
    // Debug
    pub fn new() -> Always {
        Always {
            name: "block".to_string(),
            stmt: Vec::new(),
            posedges: Vec::new(),
            negedges: Vec::new(),
        }
    }

    fn blockout(&mut self) -> String {
        self.name.clone()
    }

    pub fn block(&mut self) -> Always {
        self.name = "block".to_string();
        self.clone()
    }

    pub fn non(&mut self) -> Always {
        self.name = "Non".to_string();
        self.clone()
    }

    pub fn posedge<T: Into<Box<E>>>(&mut self, edge: T) -> Always {
        let e = *edge.into();
        match e {
            E::Ldc(wr) => self.posedges.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    pub fn negedge<T: Into<Box<E>>>(&mut self, edge: T) -> Always {
        let e = *edge.into();
        match e {
            E::Ldc(wr) => self.negedges.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    pub fn if_<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let i = If(C.into(), S);
        self.stmt.push(i);
        self.clone()
    }

    pub fn else_if<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfElseAST {
                    if_: true,
                    cond: C.into(),
                    stmt: S,
                });
                self.stmt.push(Box::new(E::BL(p)));
            }
            _ => {
                return self.clone();
            }
        }
        self.clone()
    }

    /// 分岐 else 構文追加
    pub fn else_(&mut self, S: Vec<Box<E>>) -> Always {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfElseAST {
                    if_: false,
                    cond: Box::new(E::Null),
                    stmt: S,
                });
                self.stmt.push(Box::new(E::BL(p)));
            }
            _ => {}
        }
        self.clone()
    }

    /// Case文追加
    pub fn Case<T: Into<Box<E>>>(&mut self, Sel: T) -> Always {
        let c = Case(Sel.into());
        self.stmt.push(c);
        self.clone()
    }

    /// Case文内の分岐追加
    pub fn S<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C.into(), S);
                self.stmt.push(Box::new(E::CS(p)))
            }
            _ => {
                println!("abort");
                panic!("Not Case");
            }
        }
        self.clone()
    }

    /// Case文内のデフォルト追加
    pub fn Default(&mut self, S: Vec<Box<E>>) -> Always {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(Box::new(E::Null), S);
                self.stmt.push(Box::new(E::CS(p)))
            }
            _ => {
                println!("abort");
                panic!("Not Case");
            }
        }
        self.clone()
    }

    pub fn out_p_edge(&mut self) -> Vec<WireVar> {
        self.posedges.clone()
    }

    pub fn out_n_edge(&mut self) -> Vec<WireVar> {
        self.negedges.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Func {
    top: Box<E>,
    input: Vec<Box<E>>,
    stmt: Vec<Box<E>>,
}

impl Func {
    pub fn new(name: &str, width: i32) -> Func {
        Func {
            top: WireVar::new().wire(name, width),
            input: Vec::new(),
            stmt: Vec::new(),
        }
    }

    pub fn print(&self) -> String {
        let mut st: String = String::new();
        let e = self.top.clone();
        if let E::Ldc(wrtop) = (*e).clone() {
            st += &format!("\n    function [{}:0] ", wrtop.width - 1);
            st += &decomp_ast(false, e, "", 1);
        }
        st += "(\n";
        let mut i = 0;
        for inpt in self.input.clone() {
            if let E::Ldc(wr) = (*inpt).clone() {
                if wr.width > 0 {
                    st += &format!("        input [{}:0]", wr.width - 1);
                    st += &decomp_ast(false, inpt, "", 2);
                } else {
                    st += "        input ";
                    st += &decomp_ast(false, inpt, "", 2);
                }
                i += 1;
                if i != self.input.len() {
                    st += ",\n";
                }
            }
        }
        st += "\n    );\n";
        for s in self.stmt.clone() {
            st += &decomp_ast(false, s, "block", 2);
        }
        st += "    endfunction\n\n";
        st
    }

    fn print_list(funcs: &[Func]) -> String {
        if funcs.len() == 0 {
            return String::new();
        }
        funcs
            .iter()
            .map(|func| func.print())
            .collect::<Vec<_>>()
            .join("")
    }
}

#[macro_export]
macro_rules! func_args {
    ( $($x: expr),* ) => (
        {let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x.clone());
        )*
        temp_vec
		}
    )
}

impl Func {
    /// Functionのトップ文字列を格納したAST取得
    pub fn own(&mut self) -> Box<E> {
        self.top.clone()
    }

    /// debug:構文生成
    pub fn using(&mut self, args: Vec<Box<E>>) -> Box<E> {
        let tmp = Box::new(E::Func(self.top.clone(), args));
        tmp.clone()
    }

    /// 入力の追加
    pub fn Input(&mut self, Name: &str, Width: i32) -> Box<E> {
        let mut tmp = WireVar::new();
        let port = tmp.input(Name, Width);
        self.input.push(port.clone());
        port
    }

    /// 分岐 if 構文追加
    pub fn If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func {
        let i = If(C.into(), S);
        self.stmt.push(i);
        self.clone()
    }

    /// 分岐 else if 構文追加
    pub fn Else_If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfElseAST {
                    if_: true,
                    cond: C.into(),
                    stmt: S,
                });
                self.stmt.push(Box::new(E::BL(p)));
            }
            _ => {
                return self.clone();
            }
        }
        self.clone()
    }

    /// 分岐 else 構文追加
    pub fn Else(&mut self, S: Vec<Box<E>>) -> Func {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfElseAST {
                    if_: false,
                    cond: Box::new(E::Null),
                    stmt: S,
                });
                self.stmt.push(Box::new(E::BL(p)));
            }
            _ => {}
        }
        self.clone()
    }

    /// Case 文追加
    pub fn Case<T: Into<Box<E>>>(&mut self, Sel: T) -> Func {
        let c = Case(Sel.into());
        self.stmt.push(c);
        self.clone()
    }

    /// Case 文内の分岐追加
    pub fn S<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C.into(), S);
                self.stmt.push(Box::new(E::CS(p)))
            }
            _ => {
                println!("abort");
            }
        }
        self.clone()
    }

    /// Case 文のデフォルト追加
    pub fn default(&mut self, S: Vec<Box<E>>) -> Func {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(Box::new(E::Null), S);
                self.stmt.push(Box::new(E::CS(p)))
            }
            _ => {
                println!("abort");
                panic!("Not Case");
            }
        }
        self.clone()
    }
}

#[derive(Clone, Debug)]
pub struct IfElseAST {
    if_: bool,         // if文フラグ
    cond: Box<E>,      // if文条件式
    stmt: Vec<Box<E>>, // 実行式
}

impl IfElseAST {
    fn getIfFlag(&mut self) -> bool {
        self.if_.clone()
    }
    fn getTerms(&mut self) -> Box<E> {
        self.cond.clone()
    }
    fn getStatement(&mut self) -> Vec<Box<E>> {
        self.stmt.clone()
    }
}

impl IfElseAST {
    pub fn print_list(list: Vec<IfElseAST>, cnfg: &str, indent: i32) -> String {
        let tmp = list;
        let mut num = 0;
        let mut st = String::new();

        let mut nonBranch = false;

        for mut x in tmp {
            let n = x.getStatement();
            if num == 0 {
                let e = *x.clone().getTerms();
                match e {
                    E::Null => {
                        num = 0;
                        nonBranch = true;
                    }
                    _ => {
                        for _ in 0..indent {
                            st += "    ";
                        }
                        st += "if(";
                        num += 1;
                        st += &decomp_ast(false, x.getTerms(), "", 0);
                        st += ") begin\n";
                    }
                }
            } else if x.getIfFlag() {
                for _ in 0..indent {
                    st += "    ";
                }
                st += "else if(";
                st += &decomp_ast(false, x.getTerms(), "", 0);
                st += ") begin\n";
            } else {
                for _ in 0..indent {
                    st += "    ";
                }
                st += "else begin\n";
            }

            if nonBranch {
                for y in n.clone() {
                    st += &decomp_ast(false, y, cnfg, indent);
                }
                return st;
            }
            for y in n.clone() {
                st += &decomp_ast(false, y, cnfg, indent + 1);
            }

            for _ in 0..indent {
                st += "    ";
            }
            st += "end\n";
        }
        return st;
    }
}

pub fn If<T: Into<Box<E>>>(C: T, S: Vec<Box<E>>) -> Box<E> {
    let mut i = Vec::new();
    i.push(IfElseAST {
        if_: true,
        cond: C.into(),
        stmt: S,
    });
    Box::new(E::BL(i))
}

pub trait Ifset {
    fn Else_If<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E>;
    fn Else(self, S: Vec<Box<E>>) -> Box<E>;
}

impl Ifset for Box<E> {
    fn Else_If<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut p;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfElseAST {
                    if_: true,
                    cond: C.into(),
                    stmt: S,
                });
            }
            _ => return Box::new(E::Null),
        }
        return Box::new(E::BL(p));
    }

    fn Else(self, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut p;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfElseAST {
                    if_: false,
                    cond: Box::new(E::Null),
                    stmt: S,
                });
            }
            _ => return Box::new(E::Null),
        }
        return Box::new(E::BL(p));
    }
}

#[derive(Clone, Debug)]
pub struct CaseStmt {
    CaseVar: WireVar,
    Select: Vec<Case_>,
}

impl CaseStmt {
    pub fn SetCaseV(&mut self, V: WireVar) {
        self.CaseVar = V.clone()
    }

    pub fn SetCaseS<T: Into<Box<E>>>(&mut self, Cond: T, Stmt: Vec<Box<E>>) {
        self.Select.push(Case_ {
            CaseT: Cond.into(),
            CaseS: Stmt,
        })
    }

    pub fn getCaseV(&mut self) -> WireVar {
        self.CaseVar.clone()
    }

    pub fn getSelect(&mut self) -> Vec<Case_> {
        self.Select.clone()
    }

    pub fn print(case_stmt: CaseStmt, cnfg: &str, indent: i32) -> String {
        let mut tmp = case_stmt;
        let ctmp = tmp.clone().Select;
        let mut st = String::new();
        for _ in 0..indent {
            st += "    ";
        }
        st += &format!("case ({})\n", tmp.getCaseV().name);
        for x in ctmp {
            let e = x.CaseT.clone();
            let ef = x.CaseS.clone();
            let tm = *e.clone();
            for _ in 0..indent + 1 {
                st += "    ";
            }
            match tm {
                E::Null => {
                    st += "default ";
                }
                _ => {
                    st += &decomp_ast(false, e, cnfg, indent + 1);
                }
            }
            st += " :";
            let n = ef.len();
            if n > 1 {
                st += "begin \n";
            }
            for y in ef {
                if n > 1 {
                    st += &decomp_ast(false, y, cnfg, indent + 2);
                } else {
                    st += &decomp_ast(false, y, cnfg, 0);
                }
            }
            if n > 1 {
                for _ in 0..indent + 1 {
                    st += "    ";
                }
                st += "end \n";
            }
        }
        for _ in 0..indent {
            st += "    ";
        }
        st += "endcase\n";
        return st;
    }
}

fn Case<T: Into<Box<E>>>(Sel: T) -> Box<E> {
    let e = *Sel.into();
    let mut C = CaseStmt {
        CaseVar: WireVar::new(),
        Select: Vec::new(),
    };
    match e {
        E::Ldc(wr) => {
            C.SetCaseV(wr);
        }
        _ => {
            Box::new(E::Null);
        }
    }

    Box::new(E::CS(C))
}

pub trait Caseset {
    fn S<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E>;

    fn Default(self, S: Vec<Box<E>>) -> Box<E>;
}

impl Caseset for Box<E> {
    fn S<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut n;
        match e {
            E::CS(csast) => {
                n = csast;
            }
            _ => return Box::new(E::Null),
        }
        n.SetCaseS(C.into(), S);
        Box::new(E::CS(n))
    }

    fn Default(self, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut n;
        match e {
            E::CS(csast) => {
                n = csast;
            }
            _ => return Box::new(E::Null),
        }
        n.SetCaseS(Box::new(E::Null), S);
        Box::new(E::CS(n))
    }
}

#[derive(Clone, Debug)]
pub struct Case_ {
    pub CaseT: Box<E>,
    pub CaseS: Vec<Box<E>>,
}

pub fn Form<T: Into<Box<E>>>(formu: T) -> Vec<Box<E>> {
    let mut tmp = Vec::new();
    tmp.push(formu.into());
    return tmp;
}

pub trait AddForm<T>
where
    T: Into<Box<E>>,
{
    fn Form(self, formu: T) -> Vec<Box<E>>;
}

impl<T> AddForm<T> for Vec<Box<E>>
where
    T: Into<Box<E>>,
{
    fn Form(self, formu: T) -> Vec<Box<E>> {
        let mut tmp = self;
        tmp.push(formu.into());
        return tmp;
    }
}

// --------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum E {
    Null,
    Ldc(WireVar),                // 変数
    Num(i32),                    // 数値
    No(Box<E>),                  // Not構文
    Red(String, Box<E>),         // リダクション構文
    Bin(String, Box<E>, Box<E>), // 二項演算
    PL(Box<E>, Box<E>, Box<E>),  // 分岐構文
    SB(Box<E>, Box<E>),          // 代入文
    CS(CaseStmt),                // case文
    BL(Vec<IfElseAST>),          // if, else if, else文
    Func(Box<E>, Vec<Box<E>>),   // function文
    MEM(Box<E>, Box<E>),         // メモリ
    MBT(Box<E>, Box<E>, Box<E>), // 多ビット
    Node(String),                // 内部検索用
}

impl<'a> From<&'a Box<E>> for Box<E> {
    fn from(x: &'a Box<E>) -> Self {
        x.clone()
    }
}

impl<'a> From<&'a mut Box<E>> for Box<E> {
    fn from(x: &'a mut Box<E>) -> Self {
        x.clone()
    }
}

impl From<i32> for Box<E> {
    fn from(i: i32) -> Self {
        _Num(i)
    }
}

impl From<&i32> for Box<E> {
    fn from(i: &i32) -> Self {
        _Num(*i)
    }
}

// 変数出力関数
fn _V(V: WireVar) -> Box<E> {
    Box::new(E::Ldc(V))
}

// 数値出力関数
pub fn _Num(num: i32) -> Box<E> {
    Box::new(E::Num(num))
}

// 代入演算関数
pub fn _Veq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::SB(L.into(), R.into()))
}

// 分岐構文関数
pub fn _Branch<T: Into<Box<E>>, U: Into<Box<E>>, V: Into<Box<E>>>(
    Terms: T,
    TrueNode: U,
    FalseNode: V,
) -> Box<E> {
    Box::new(E::PL(Terms.into(), TrueNode.into(), FalseNode.into()))
}

// 演算子関数
/// "+" addition
fn _Add<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("add".to_string(), L.into(), R.into()))
}

/// "-" substruction
fn _Sub<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("sub".to_string(), L.into(), R.into()))
}

/// "*" multipication
fn _Mul<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("mul".to_string(), L.into(), R.into()))
}

/// "/" division
fn _Div<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("div".to_string(), L.into(), R.into()))
}

/// "%" modulo
fn _Mod<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("mod".to_string(), L.into(), R.into()))
}

/// "||" or
fn _LOr<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("lor".to_string(), L.into(), R.into()))
}

/// "&&" and
fn _LAnd<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("land".to_string(), L.into(), R.into()))
}

/// "|" or
fn _Or<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or".to_string(), L.into(), R.into()))
}

/// "&" and
fn _And<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("and".to_string(), L.into(), R.into()))
}

/// "^" exclusive or
fn _Xor<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("xor".to_string(), L.into(), R.into()))
}

/// "==" equal
pub fn _Eq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("equal".to_string(), L.into(), R.into()))
}

/// "!=" not equal
pub fn _Neq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("Not equal".to_string(), L.into(), R.into()))
}

/// "<<" left shift
fn _LSH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("lshift".to_string(), L.into(), R.into()))
}

/// ">>" right shift
fn _RSH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("rshift".to_string(), L.into(), R.into()))
}

/// ">>>" right arithmetic shift
pub fn _RSHA<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("rshifta".to_string(), L.into(), R.into()))
}

/// "<" more than
fn _MTH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("more_than".to_string(), L.into(), R.into()))
}

/// ">" less than
fn _LTH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("less_than".to_string(), L.into(), R.into()))
}

/// "<=" or more
fn _OMR<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or_more".to_string(), L.into(), R.into()))
}

/// ">=" or less
fn _OLS<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or_less".to_string(), L.into(), R.into()))
}

/**
 * 演算子実装メソッド
 *
 **/
pub trait Notc {
    fn not(&self) -> Box<E>;
}

impl Notc for Box<E> {
    fn not(&self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl Not for Box<E> {
    type Output = Box<E>;

    fn not(self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl<T> Add<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn add(self, other: T) -> Box<E> {
        _Add(self, other.into())
    }
}

impl<T> Add<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn add(self, other: T) -> Box<E> {
        _Add(self, &other.into())
    }
}

impl<T> Sub<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn sub(self, other: T) -> Box<E> {
        _Sub(self, other.into())
    }
}

impl<T> Sub<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn sub(self, other: T) -> Box<E> {
        _Sub(self, &other.into())
    }
}

impl<T> Mul<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn mul(self, other: T) -> Box<E> {
        _Mul(self, other.into())
    }
}

impl<T> Mul<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn mul(self, other: T) -> Box<E> {
        _Mul(self, &other.into())
    }
}

impl<T> Div<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn div(self, other: T) -> Box<E> {
        _Div(self, other.into())
    }
}

impl<T> Div<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn div(self, other: T) -> Box<E> {
        _Div(self, &other.into())
    }
}

impl<T> Rem<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn rem(self, other: T) -> Box<E> {
        _Mod(self, other.into())
    }
}

impl<T> Rem<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn rem(self, other: T) -> Box<E> {
        _Mod(self, &other.into())
    }
}

impl<T> BitOr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitor(self, other: T) -> Box<E> {
        _Or(self, other.into())
    }
}

impl<T> BitOr<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitor(self, other: T) -> Box<E> {
        _Or(self, &other.into())
    }
}

impl<T> BitAnd<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitand(self, other: T) -> Box<E> {
        _And(self, other.into())
    }
}

impl<T> BitAnd<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitand(self, other: T) -> Box<E> {
        _And(self, &other.into())
    }
}

impl<T> BitXor<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitxor(self, other: T) -> Box<E> {
        _Xor(self, other.into())
    }
}

impl<T> BitXor<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitxor(self, other: T) -> Box<E> {
        _Xor(self, &other.into())
    }
}

impl<T> Shl<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shl(self, other: T) -> Box<E> {
        _LSH(self, other.into())
    }
}

impl<T> Shl<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shl(self, other: T) -> Box<E> {
        _LSH(self, &other.into())
    }
}

impl<T> Shr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shr(self, other: T) -> Box<E> {
        _RSH(self, other.into())
    }
}

impl<T> Shr<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shr(self, other: T) -> Box<E> {
        _RSH(self, &other.into())
    }
}

// Equal,Not Equal構文生成
pub trait PartialEq<Rhs = Self> {
    fn eq(self, other: Rhs) -> Box<E>;

    fn ne(self, other: Rhs) -> Box<E>;
}

impl<T> PartialEq<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn eq(self, other: T) -> Box<E> {
        _Eq(self, other.into())
    }

    fn ne(self, other: T) -> Box<E> {
        _Neq(self, other.into())
    }
}

impl<T> PartialEq<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    fn eq(self, other: T) -> Box<E> {
        _Eq(self, &other.into())
    }

    fn ne(self, other: T) -> Box<E> {
        _Neq(self, &other.into())
    }
}

// compare構文生成
pub trait PartialOrd<Rhs = Self> {
    fn lt(self, other: Rhs) -> Box<E>;

    fn le(self, other: Rhs) -> Box<E>;

    fn gt(self, other: Rhs) -> Box<E>;

    fn ge(self, other: Rhs) -> Box<E>;
}

impl<T> PartialOrd<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn lt(self, other: T) -> Box<E> {
        _LTH(self, other.into())
    }

    fn le(self, other: T) -> Box<E> {
        _OLS(self, other.into())
    }

    fn gt(self, other: T) -> Box<E> {
        _MTH(self, other.into())
    }

    fn ge(self, other: T) -> Box<E> {
        _OMR(self, other.into())
    }
}

impl<T> PartialOrd<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    fn lt(self, other: T) -> Box<E> {
        _LTH(self, &other.into())
    }

    fn le(self, other: T) -> Box<E> {
        _OLS(self, &other.into())
    }

    fn gt(self, other: T) -> Box<E> {
        _MTH(self, &other.into())
    }

    fn ge(self, other: T) -> Box<E> {
        _OMR(self, &other.into())
    }
}

// 代入文生成
pub trait Subs<Rhs = Self> {
    fn sst(&self, other: Rhs) -> Box<E>;
}

impl<T> Subs<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn sst(&self, other: T) -> Box<E> {
        _Veq(self.clone(), other.into())
    }
}

// 論理演算子生成
pub trait Logi<Rhs = Self> {
    fn land(&self, other: Rhs) -> Box<E>;

    fn lor(&self, other: Rhs) -> Box<E>;
}

impl<T> Logi<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn land(&self, other: T) -> Box<E> {
        _LAnd(self, &other.into())
    }

    fn lor(&self, other: T) -> Box<E> {
        _LOr(self, &other.into())
    }
}

// メモリ、レジスタ用アドレス指定
pub trait Addr<Rs = Self> {
    fn addr(&self, address: Rs) -> Box<E>;
}

impl<T> Addr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn addr(&self, address: T) -> Box<E> {
        Box::new(E::MEM(self.clone(), address.into()))
    }
}

// レジスタ用多ビット指定
pub trait MBit<Rs = Self> {
    fn range(&self, hbit: Rs, lbit: Rs) -> Box<E>;
}

impl<T> MBit<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn range(&self, hbit: T, lbit: T) -> Box<E> {
        Box::new(E::MBT(self.clone(), hbit.into(), lbit.into()))
    }
}

/**
 * 出力、分解、デバッグ関数
 * 出力関数以外はデバッグ用関数のため削除しても問題はない
 **/

/// 分解出力関数
fn decomp_ast(Parenthesis: bool, ast: Box<E>, cnfg: &str, indent: i32) -> String {
    let e = *ast;
    let mut st = String::new();

    match e {
        E::Bin(ref bin, ref l, ref r) => {
            let tmp = bin.as_str();
            for _ in 0..indent {
                st += "    ";
            }
            if Parenthesis {
                match tmp {
                    "add" => st += "(",
                    "sub" => st += "(",
                    "or" => st += "(",
                    "lor" => st += "(",
                    _ => st += "",
                }
            }
            let mut pareset = false;
            st += &decomp_ast(false, l.clone(), cnfg, 0);
            match tmp {
                "add" => {
                    st += "+";
                }
                "sub" => {
                    st += "-";
                }
                "mul" => {
                    st += "*";
                    pareset = true
                }
                "div" => {
                    st += "/";
                    pareset = true
                }
                "mod" => {
                    st += "%";
                    pareset = true
                }
                "or" => {
                    st += "|";
                }
                "and" => {
                    st += "&";
                }
                "xor" => {
                    st += "^";
                }
                "lor" => {
                    st += "||";
                }
                "land" => {
                    st += "&&";
                }
                "lshift" => {
                    st += "<<";
                }
                "rshift" => {
                    st += ">>";
                }
                "rshifta" => {
                    st += ">>>";
                }
                "equal" => {
                    st += "==";
                }
                "Not equal" => {
                    st += "!=";
                }
                "more_than" => {
                    st += "<";
                }
                "less_than" => {
                    st += ">";
                }
                "or_more" => {
                    st += "<=";
                }
                "or_less" => {
                    st += ">=";
                }
                _ => panic!("No correspond syntax : error operator -- {}", tmp),
            }
            st += &decomp_ast(pareset, r.clone(), cnfg, 0);
            if Parenthesis {
                match tmp {
                    "add" => {
                        st += ")";
                    }
                    "sub" => {
                        st += ")";
                    }
                    "or" => {
                        st += ")";
                    }
                    "lor" => {
                        st += ")";
                    }
                    _ => {
                        st += "";
                    }
                }
            }
        }
        E::Ldc(ref wr) => {
            st += &format!("{}", wr.name);
        }
        E::Num(ref i) => {
            st += &format!("{}", i);
        }
        E::PL(ref d, ref t, ref f) => {
            st += "(";
            st += &decomp_ast(false, d.clone(), cnfg, 0);
            st += ")? ";
            st += &decomp_ast(false, t.clone(), cnfg, 0);
            st += ": ";

            st += &decomp_ast(false, f.clone(), cnfg, 0);
        }
        E::SB(ref l, ref r) => {
            for _ in 0..indent {
                st += "    ";
            }
            st += &decomp_ast(false, l.clone(), cnfg, indent);
            if cnfg.to_string() == "block".to_string() {
                st += " = ";
            } else {
                st += " <= ";
            }
            st += &decomp_ast(false, r.clone(), cnfg, 0);
            st += ";\n";
        }
        E::CS(ref c) => {
            let cn = &*c;
            st += &CaseStmt::print(cn.clone(), cnfg, indent);
        }
        E::BL(ref i) => {
            let iels = &*i;
            st += &IfElseAST::print_list(iels.clone(), cnfg, indent);
        }
        E::MEM(ref m, ref a) => {
            let ma = &*m;
            let aa = &*a;
            st += &decomp_ast(false, ma.clone(), cnfg, indent);
            st += &format!("[");
            st += &decomp_ast(false, aa.clone(), cnfg, 0);
            st += &format!("]");
        }
        E::MBT(ref m, ref a, ref b) => {
            let mn = &*m;
            let aa = &*a;
            let bb = &*b;
            st += &decomp_ast(false, mn.clone(), cnfg, indent);
            st += &format!("[");
            st += &decomp_ast(false, aa.clone(), cnfg, 0);
            st += &format!(":");
            st += &decomp_ast(false, bb.clone(), cnfg, 0);
            st += &format!("]");
        }
        E::Func(ref a, ref v) => {
            st += &decomp_ast(false, a.clone(), cnfg, 0);
            st += &format!("(");
            let mut i: usize = 0;
            for x in v.clone() {
                st += &decomp_ast(false, x.clone(), cnfg, 0);
                i += 1;
                if v.len() != i {
                    st += &format!(", ");
                }
            }
            st += &format!(")");
        }
        E::No(ref b) => {
            let bb = &*b;
            st += "~";
            st += &decomp_ast(false, bb.clone(), cnfg, 0);
        }
        E::Red(ref r, ref a) => {
            let tmp = r.as_str();
            match tmp.clone() {
                "and" => {
                    st += "&";
                }
                "or" => st += "|",
                "xor" => st += "^",
                "nand" => st += "~&",
                "nor" => st += "~|",
                "xnor" => st += "~^",
                _ => {
                    return st;
                }
            }
            st += &decomp_ast(false, a.clone(), cnfg, 0);
        }
        _ => {
            st += "";
        }
    }
    return st;
}

impl FsmModule {
    fn print(&self) -> String {
        let mut st = String::new();
        let tmp = self.clone();
        let clk = tmp.clone().StateClk();
        let rst = tmp.clone().StateRst();
        let reg = tmp.clone().StateReg();
        let p = tmp.clone().StateOut();
        st += &format!(
            "    always @(posedge {} or posedge {}) begin\n",
            _StrOut(clk.clone()),
            _StrOut(rst.clone())
        );
        st += &format!(
            "        if ({} == 1) begin \n            {} <= {}; \n        end\n",
            _StrOut(rst.clone()),
            _StrOut(reg.clone()),
            _StrOut(tmp.clone().init_state())
        );
        st += &format!(
            "        else begin \n            {} <= {}_Next; \n        end \n    end \n\n",
            _StrOut(reg.clone()),
            _StrOut(reg.clone())
        );
        st += &format!("    always@(posedge {}) begin\n", _StrOut(clk.clone()));
        st += &format!(
            "        if ({}) {}_Next <= {};\n",
            _StrOut(rst.clone()),
            _StrOut(reg.clone()),
            _StrOut(tmp.clone().init_state())
        );
        st += "        else begin\n";
        st += &format!("            case({})\n", _StrOut(reg.clone()));
        for s in p {
            st += &s.print();
        }
        st += "            endcase \n        end\n    end\n\n";

        return st;
    }
}

impl StateModule {
    fn print(&self) -> String {
        let mut s = self.clone();
        let stname = s.getStateName();
        let tmp = s.getBranch();

        let mut st = String::new();

        st += &format!("                {} : begin\n", stname);
        st += &IfElseAST::print_list(tmp.clone(), "Non", 5);
        st += "                end\n";

        return st;
    }
}

/// AXIインタフェース出力関数
fn print_axi(axi: Bus, num: i32) -> String {
    let tmp = axi.clone();
    let mut st = String::new();
    match tmp {
        Bus::AxiLite(x) => {
            st += &print_axi_lite_slave(x, num);
        }
        Bus::AxiSlave(x) => {
            st += &print_axis(x);
        }
        Bus::AxiMaster(_) => {
            unimplemented!();
        }
        Bus::AxiStream(_) => {
            unimplemented!();
        }
    }
    return st;
}

/// AXISLite構文出力関数--ほぼテンプレ
fn print_axi_lite_slave(axi: AxiLite, count: i32) -> String {
    let tmp = axi.clone();
    let mut st = String::new();

    // register
    let reg_tmp = tmp.reg_array.clone();

    // address space
    let mut addr_width = 0;

    // address width
    let reg_length = tmp.reg_array.len() as i32;
    let mut reg_addr_width: i32 = 1;
    loop {
        if 2i32.pow(reg_addr_width as u32) >= (reg_length * 4 - 1) {
            break;
        }
        reg_addr_width += 1;
    }

    st += &format!("    // AXI Lite Slave Port : Number {}\n", count);
    st += &format!("    reg r_en{};\n", count);
    st += &format!("    wire w_wdata_en{};\n", count);
    st += &format!("    wire w_rdata_en{};\n\n", count);

    st += "    // wready - waddress generating\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_wready{} <= 1'b0;\n            r_awready{0} <= 1'b0;\n            r_en{0} <= 1'b1;\n            r_awaddr{0} <= 0;\n",count);
    st += &format!("        end else begin\n");
    st += &format!(
        "            if( ~r_wready{} && w_awvalid{0} && w_wvalid{0} && r_en{0} ) begin\n",
        count
    );
    st += &format!("                r_wready{0} <= 1'b1;\n            end else begin\n                r_wready{0} <= 1'b0;\n            end\n\n",count);
    st += &format!(
        "            if( ~r_awready{} && w_awvalid{0} && w_wvalid{0} && r_en{0} ) begin\n",
        count
    );
    st += &format!("                r_awready{0} <= 1'b1;\n                r_en{0} <= 1'b0;\n                r_awaddr{0} <= i_s_awaddr{0};\n", count);
    st += &format!("            end else begin\n");
    st += &format!(
        "                if( w_bready{} && r_bvalid{0} ) begin\n",
        count
    );
    st += &format!(
        "                    r_en{} <= 1'b1;\n                end\n",
        count
    );
    st += &format!("                r_awready{0} <= 1'b0;\n", count);
    st += &format!("            end\n        end\n    end\n\n");

    st += &format!(
        "    assign w_wdata_en{} = r_awready{0} && r_wready{0} && w_awvalid{0} && w_wvalid{0};\n\n",
        count
    );

    st += "    // wdata generating\n";
    st += &format!(
        "    always@( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));

    for x in tmp.reg_array.clone() {
        st += &format!("            {} <= 32'd0;\n", _StrOut(x));
    }
    st += &format!(
        "        end\n        else begin\n            if( w_wdata_en{} == 1'd1 ) begin\n",
        count
    );
    st += &format!(
        "                case ( r_awaddr{}[{}:2] )\n",
        count,
        reg_addr_width - 1
    );

    st += "    // generate write register\n";
    for x in reg_tmp.clone() {
        // Unpack
        let reg = x;
        st += &format!(
            "                    {}'h{:02X} : begin\n",
            reg_addr_width - 2,
            addr_width
        );
        for addr_count in 0..4 {
            st += &format!(
                "                        if ( r_wstrb{}[{}] == 1'b1 ) {} <= w_wdata{0}[{}:{}];\n",
                count,
                addr_count,
                _StrOut(reg.clone()),
                8 * (addr_count + 1) - 1,
                8 * addr_count
            );
        }

        addr_width += 1;
        st += "                    end\n";
    }
    st += "                    default: begin\n";
    for x in reg_tmp.clone() {
        st += &format!(
            "                        {} <= {};\n",
            _StrOut(x.clone()),
            _StrOut(x.clone())
        );
    }
    st += "                    end\n                endcase\n            end\n";

    st += "    // Local write en\n";
    let write_tmp = tmp.wLocal_write.clone();
    let mut i = -1;
    for x in write_tmp.clone() {
        i += 1;
        if let E::Null = *(x.0.clone()) {
            continue;
        }
        st += &format!(
            "\n            if( {} ) begin \n",
            &decomp_ast(false, x.0, "", 0)
        );
        st += &format!(
            "                    {} <= {};\n",
            _StrOut(reg_tmp[i as usize].clone()),
            &decomp_ast(false, x.1, "", 0)
        );
        st += "            end\n";
    }
    st += "        end\n    end\n\n";

    st += "    // wready - waddress generating\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_bvalid{} <= 1'b0;\n", count);
    st += &format!(
        "            r_arready{} <= 1'b0;\n            r_araddr{0} <= 0;\n",
        count
    );
    st += &format!("            r_rvalid{} <= 1'b0;\n", count);
    st += "        end else begin\n";

    st += &format!("            if( r_awready{} && w_awvalid{0} && ~r_bvalid{0} && r_wready{0} && w_wvalid{0} ) begin\n", count);
    st += &format!("                r_bvalid{} <= 1'b1;\n            end else if( w_bready{0} && r_bvalid{0} ) begin\n                r_bvalid{0} <= 1'b0;\n            end\n\n",count);

    st += &format!(
        "            if( ~r_arready{} && w_arvalid{0} ) begin\n",
        count
    );
    st += &format!("                r_arready{} <= 1'b1;\n                r_araddr{0} <= i_s_araddr{0};\n            end else begin\n                r_arready{0} <= 1'b0;\n            end\n", count);

    st += &format!(
        "            if( r_arready{} && w_arvalid{0} && ~r_rvalid{0} ) begin\n",
        count
    );
    st += &format!("                r_rvalid{} <= 1'b1;\n            end else if ( r_rvalid{0} && w_rready{0} ) begin\n                r_rvalid{0} <= 1'b0;\n            end\n", count);
    st += "        end\n    end\n\n";

    st += "    // rdata generation\n";
    st += &format!(
        "    assign w_rdata_en{} = r_arready{0} && w_arvalid{0} && ~r_rvalid{0};\n\n",
        count
    );
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_rdata{} <= 32'd0; \n        end\n", count);
    st += "        else begin\n";
    st += &format!("            if( w_rdata_en{} ) begin\n", count);
    st += &format!(
        "                case( r_araddr{}[{}:2] )\n",
        count,
        reg_addr_width - 1
    );

    // 配列の生成
    i = -1;
    for x in reg_tmp.clone() {
        i += 1;
        st += &format!(
            "                    {}'h{:02X} : r_rdata{} <= {};\n",
            reg_addr_width - 2,
            i,
            count,
            _StrOut(x.clone())
        );
    }

    st += &format!(
        "                    default: r_rdata{} <= 32'hDEAD_DEAD;\n                endcase\n",
        count
    );
    st += "            end\n        end\n    end\n\n";

    return st;
}

fn print_axis(axi: Axi4Slave) -> String {
    let tmp = axi.clone();
    let mut st = String::new();

    // address space
    let mut addr_width: i32 = 1;
    loop {
        if 2i32.pow(addr_width as u32) >= (tmp.length) {
            break;
        }
        addr_width += 1;
    }

    st += &format!("    // AXI-full Slave Port\n\n");

    // -- not support wrap mode --
    st += "    reg            r_axi_awv_awr_flag;\n";
    st += "    reg            r_axi_arv_arr_flag;\n";
    st += "    reg    [7:0]   r_axi_awlen_count;\n";
    st += "    reg    [7:0]   r_axi_arlen_count;\n";
    st += "    reg    [1:0]   r_axi_arburst;\n";
    st += "    reg    [1:0]   r_axi_awburst;\n\n";

    if tmp.mem {
        st += &format!("    reg [31:0] axi_mem [0:{}];\n", tmp.length - 1);
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( r_axi_wready & w_axi_wvalid ) begin\n");
        st += &format!("            axi_mem[r_axi_awaddr] <= w_axi_wdata;\n");
        st += &format!("        end else if ( axis_wen ) begin\n");
        st += &format!("            axi_mem[axis_addr] <= axis_write;\n");
        st += &format!("        end\n    end\n\n");
    } else {
        st += "    assign axis_wen = r_axi_wready & w_axi_wvalid;\n";
        st += &format!("    assign axis_addr = (r_axi_awv_awr_flag) ? r_axi_awaddr[{0}:2] : \n                       (r_axi_arv_arr_flag) ? r_axi_araddr[{0}:2] : 0;", addr_width+1);
    }

    st += "    // awready - awv_awr_flag generating\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_axi_awready <= 1'b0;\n            r_axi_awv_awr_flag <= 1'b0;\n");
    st += "        end else begin\n";
    st += &format!("            if (~r_axi_awready && w_axi_awvalid && ~r_axi_awv_awr_flag && ~r_axi_arv_arr_flag ) begin\n");
    st += &format!(
        "                r_axi_awready <= 1'b1;\n            r_axi_awv_awr_flag <= 1'b1;\n"
    );
    st += &format!("            end else if ( w_axi_wlast && r_axi_wready ) begin\n                r_axi_awv_awr_flag <= 1'b0;\n");
    st += &format!("            end else begin\n                r_axi_awready <= 1'b0;\n");
    st += "            end\n        end\n    end\n\n";

    st += "    // waddress generation\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_axi_awaddr <= 0;\n            r_axi_awlen_count <= 0;\n            r_axi_awburst <= 0;\n            r_axi_awlen <= 0;\n");
    st += "        end else begin\n";
    st += &format!(
        "            if ( ~r_axi_awready && w_axi_awvalid && ~r_axi_awv_awr_flag ) begin\n"
    );
    st += &format!("                r_axi_awaddr <= i_saxi_awaddr;\n                r_axi_awburst <= i_saxi_awburst;\n                r_axi_awlen <= i_saxi_awlen;\n                r_axi_awlen_count <= 0;\n");
    st += &format!("            end else if ( ( r_axi_awlen_count <= r_axi_awlen ) && r_axi_wready && w_axi_wvalid ) begin\n");
    st += &format!("                r_axi_awlen_count <= r_axi_awlen_count + 1;\n\n");
    st += &format!("                case ( r_axi_awburst )\n");
    st += &format!("                    2'b00: begin\n                        r_axi_awaddr <= r_axi_awaddr;\n                    end\n");
    st += &format!("                    2'b01: begin\n                        r_axi_awaddr[{0}:2] <= r_axi_awaddr[{0}:2] + 1;\n                        r_axi_awaddr[1:0] <= 2'b00;\n                    end\n", addr_width+1);
    st += &format!("                    default: begin\n                        r_axi_awaddr <= r_axi_awaddr[{0}:2] + 1;\n                    end\n                endcase\n", addr_width+1);
    st += "            end\n        end\n    end\n\n";

    st += "    // wready generation\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_axi_wready <= 0;\n");
    st += "        end else begin\n";
    st += &format!("            if ( ~r_axi_wready && w_axi_wvalid && r_axi_awv_awr_flag ) begin\n                r_axi_wready <= 1'b1;\n            end else begin\n                r_axi_wready <= 1'b0;\n            end\n");
    st += "        end\n    end\n\n";

    st += "    // write response generation\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_axi_bvalid <= 0;\n");
    st += "        end else begin\n";
    st += &format!("            if ( r_axi_awv_awr_flag && r_axi_wready && w_axi_wvalid && ~r_axi_bvalid && w_axi_wlast ) begin\n");
    st += &format!("                r_axi_bvalid <= 1'b1;\n");
    st += &format!("            end else begin\n");
    st += &format!("                if ( w_axi_bready && r_axi_bvalid ) begin\n                    r_axi_bvalid <= 1'b0;\n                end\n");
    st += "            end\n        end\n    end\n\n";

    st += "    // arready - arv_arr_flag generation\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_axi_arready <= 1'b0;\n            r_axi_arv_arr_flag <= 1'b0;\n");
    st += "        end else begin\n";
    st += &format!("            if ( ~r_axi_arready && w_axi_arvalid && ~r_axi_awv_awr_flag && ~r_axi_arv_arr_flag ) begin\n");
    st += &format!(
        "                r_axi_arready <= 1'b1;\n                r_axi_arv_arr_flag <= 1'b1;\n"
    );
    st += &format!("            end else if ( r_axi_rvalid && w_axi_rready && r_axi_arlen_count == r_axi_arlen ) begin\n                r_axi_arv_arr_flag <= 1'b0;\n");
    st += &format!("            end else begin\n                r_axi_arready <= 1'b0;\n");
    st += "            end\n        end\n    end\n\n";

    st += "    // raddress generation\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_axi_araddr <= 0;\n            r_axi_arlen_count <= 0;\n            r_axi_arburst <= 0;\n            r_axi_arlen <= 0;\n            r_axi_rlast <= 0;\n");
    st += "        end else begin\n";
    st += &format!(
        "            if ( ~r_axi_arready && w_axi_arvalid && ~r_axi_arv_arr_flag ) begin\n"
    );
    st += &format!("                r_axi_araddr <= i_saxi_araddr;\n                r_axi_arburst <= i_saxi_arburst;\n                r_axi_arlen <= i_saxi_arlen;\n                r_axi_arlen_count <= 0;\n                r_axi_rlast <= 0;\n");
    st += &format!("            end else if ( ( r_axi_arlen_count <= r_axi_arlen ) && r_axi_rvalid && w_axi_rready ) begin\n");
    st += &format!("                r_axi_arlen_count <= r_axi_arlen_count + 1;\n                r_axi_rlast <= 0;\n");
    st += &format!("                case ( r_axi_arburst )\n");
    st += &format!("                    2'b00: begin\n                        r_axi_araddr <= r_axi_araddr;\n                    end\n");
    st += &format!("                    2'b01: begin\n                        r_axi_araddr[{0}:2] <= r_axi_araddr[{0}:2] + 1;\n                        r_axi_araddr[1:0] <= 2'b00;\n                    end\n", addr_width+1);
    st += &format!("                    default: begin\n                        r_axi_araddr <= r_axi_araddr[{0}:2];\n                    end\n                endcase\n", addr_width+1);
    st += &format!("            end else if ( ( r_axi_arlen_count == r_axi_arlen ) && ~r_axi_rlast && r_axi_arv_arr_flag ) begin\n                r_axi_rlast <= 1'b1;\n");
    st += &format!(
        "            end else if ( w_axi_rready ) begin\n                r_axi_rlast <= 1'b0;\n"
    );
    st += "            end\n        end\n    end\n\n";

    st += "    // rvalid generation\n";
    st += &format!(
        "    always @( posedge {} ) begin\n",
        _StrOut(tmp.clone().clk)
    );
    st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_axi_rvalid <= 0;\n");
    st += "        end else begin\n";
    st += &format!("            if ( ~r_axi_wready && w_axi_wvalid && r_axi_awv_awr_flag ) begin\n                r_axi_rvalid <= 1'b1;\n            end else begin\n                r_axi_rvalid <= 1'b0;\n            end\n");
    st += "        end\n    end\n\n";

    st += "    assign w_axi_wdata[0+:8] = i_saxi_wstrb[0] ? i_saxi_wdata[0+:8] : 0;\n";
    st += "    assign w_axi_wdata[8+:8] = i_saxi_wstrb[1] ? i_saxi_wdata[8+:8] : 0;\n";
    st += "    assign w_axi_wdata[16+:8] = i_saxi_wstrb[2] ? i_saxi_wdata[16+:8] : 0;\n";
    st += "    assign w_axi_wdata[24+:8] = i_saxi_wstrb[3] ? i_saxi_wdata[24+:8] : 0;\n";

    if tmp.mem {
        st += "\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!(
            "        r_axi_rdata <= axi_mem[r_axi_araddr[{}:2]];\n",
            addr_width + 1
        );
        st += &format!("        axis_read <= axi_mem[axis_addr];\n");
        st += &format!("    end\n\n");
    } else {
        st += "\n";
        st += &format!("    always @(*) begin\n");
        if let E::Null = *(tmp.clone().rdata) {
            st += &format!("        r_axi_rdata <= axis_read;\n");
        } else {
            st += &format!("        r_axi_rdata <= {};\n", _StrOut(tmp.clone().rdata));
        }
        st += &format!("    end\n\n");
        st += &format!("    assign axis_write = w_axi_wdata;\n");
    }

    return st;
}

/// NONAST
#[macro_export]
macro_rules! Blank {
    () => {
        Box::new(E::Null)
    };
}

/// FSMモジュール
#[derive(Debug, Clone)]
pub struct FsmModule {
    clk: Box<E>,
    rst: Box<E>,
    state_reg: Box<E>,
    states: Vec<StateModule>,
    Current_state: i32,
}

impl FsmModule {
    pub fn new<T: Into<Box<E>>, U: Into<Box<E>>>(clk: T, rst: U, state: &str) -> FsmModule {
        let state = WireVar::new().reg(state, 32);
        FsmModule {
            clk: clk.into(),
            rst: rst.into(),
            state_reg: state,
            states: Vec::new(),
            Current_state: 0,
        }
    }

    fn init_state(&mut self) -> Box<E> {
        self.states[0].getState()
    }

    // ステートレジスタの変更
    pub fn State(mut self, set_state: &str) -> FsmModule {
        self.state_reg = WireVar::new().reg(set_state, 32);
        self
    }

    // ステートの追加
    pub fn AddState(&mut self, State_name: &str) -> FsmModule {
        let mut p = WireVar::new();
        self.Current_state = self.states.len() as i32;
        p.parameter(State_name, self.Current_state);
        let tmp = StateModule {
            state: Box::new(E::Ldc(p)),
            branch: Vec::new(),
        };
        self.states.push(tmp);

        self.clone()
    }

    // カレントの移動
    pub fn Current(&mut self, State_name: &str) -> FsmModule {
        let mut count = 0;
        for x in &mut self.states {
            let Nx = x.getStateName();
            count += 1;
            if Nx == State_name.to_string() {
                self.Current_state = count;
            }
        }

        self.clone()
    }

    // カレントステートから次のステートへの定義
    pub fn goto<T: Into<Box<E>>>(&mut self, State_name: &str, Branch: T) -> FsmModule {
        let SelfS = self.state_reg.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() {
            st = wr.name.clone()
        };
        st = st + "_Next";
        let NState = WireVar::new().reg(&st, 0);
        let Goto_ = WireVar::new().parameter(State_name, 0);
        self.states[self.Current_state as usize].set_branch(Branch.into(), F!(NState = Goto_));

        self.clone()
    }

    // 指定ステートからカレントステートへの定義(指定ステートの作成後以降に使用可)
    pub fn from<T: Into<Box<E>>>(&mut self, State_name: &str, Branch: T) -> FsmModule {
        let SelfS = self.state_reg.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() {
            st = wr.name.clone()
        };
        st = st + "_Next";
        let NState = WireVar::new().reg(&st, 0);
        let NameCurrentState = self.states[((self.Current_state - 1) as usize)].getStateName();
        let branch = Branch.into();
        for x in &mut self.states {
            let Nx = x.getStateName();
            if Nx == State_name.to_string() {
                let Goto_ = WireVar::new().parameter(&NameCurrentState, 0);
                x.set_branch(branch.clone(), F!(NState = Goto_));
            }
        }

        self.clone()
    }

    // セットパラメータの取得
    pub fn Param(&mut self, name: &str) -> Box<E> {
        let SelfS = self.states.clone();
        for mut x in SelfS {
            let Nx = x.getStateName();
            if Nx == name.to_string() {
                return x.getState();
            }
        }
        return Box::new(E::Null);
    }

    // 内部メソッド(ステート格納レジスタを外部に出力)
    fn StateReg(self) -> Box<E> {
        let tmp = self.clone();
        tmp.state_reg
    }

    // 内部メソッド(クロックを外部に出力)
    fn StateClk(self) -> Box<E> {
        let tmp = self.clone();
        tmp.clk
    }

    // 内部メソッド(リセットを外部に出力)
    fn StateRst(self) -> Box<E> {
        let tmp = self.clone();
        tmp.rst
    }

    fn StateOut(self) -> Vec<StateModule> {
        let tmp = self.clone();
        tmp.states
    }
}

/// 1ステートモデル
#[derive(Debug, Clone)]
struct StateModule {
    state: Box<E>,
    branch: Vec<IfElseAST>,
}

impl StateModule {
    // ステート設定
    fn set_state(&mut self, stmt: Box<E>) {
        self.state = stmt
    }

    // ステート分岐先設定
    fn set_branch<T: Into<Box<E>>, U: Into<Box<E>>>(&mut self, Terms: T, Form: U) -> bool {
        let e = *(Terms.into());
        let mut tmp = Vec::new();
        tmp.push(Form.into());

        match e {
            E::Null => self.branch.push(IfElseAST {
                if_: true,
                cond: Box::new(e),
                stmt: tmp,
            }),
            _ => self.branch.push(IfElseAST {
                if_: true,
                cond: Box::new(e),
                stmt: tmp,
            }),
        }
        return true;
    }

    fn getState(&mut self) -> Box<E> {
        let tmp = self.clone();
        tmp.state
    }

    fn getStateName(&mut self) -> String {
        let tmp = *(self.clone().state);
        match tmp {
            E::Ldc(b) => b.name,
            _ => "Nothing".to_string(),
        }
    }

    fn getBranch(&mut self) -> Vec<IfElseAST> {
        self.clone().branch
    }
}

/// -------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Bus {
    AxiLite(AxiLite),
    AxiSlave(Axi4Slave),
    AxiMaster(AxiMaster),
    AxiStream(AxiStream),
}

pub trait AxiSlaveReg<T> {
    fn order_reg_set(&mut self, num: i32) -> T;
}

pub trait AxiSlaveLocalWrite<T, U>
where
    T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    fn reg_write(&mut self, write_en: U, write_data: T);
}

pub trait AXIStreamRegCtrl {
    fn write(&mut self) -> Box<E>;
    fn addr(&mut self) -> Box<E>;
    fn wen(&mut self) -> Box<E>;
    fn mem_if(&mut self) -> (Box<E>, Box<E>, Box<E>, Box<E>);
}

/// -------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct AxiStream;

#[derive(Debug, Clone)]
pub struct AxiMaster;

/// AXI Slave Lite インタフェースの作成
#[derive(Debug, Clone)]
pub struct AxiLite {
    clk: Box<E>,
    rst: Box<E>,
    reg_array: Vec<Box<E>>,
    wLocal_write: Vec<(Box<E>, Box<E>)>,
    current_reg: i32,
}

impl AxiLite {
    pub fn new<T: Into<Box<E>>, U: Into<Box<E>>>(clock: T, reset: U) -> AxiLite {
        AxiLite {
            clk: clock.into(),
            rst: reset.into(),
            reg_array: Vec::new(),
            wLocal_write: Vec::new(),
            current_reg: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Axi4Slave {
    clk: Box<E>,
    rst: Box<E>,
    length: i32,
    mem: bool,
    rdata: Box<E>,
}

impl Axi4Slave {
    pub fn new<T: Into<Box<E>>, U: Into<Box<E>>>(clock: T, reset: U) -> Axi4Slave {
        Axi4Slave {
            clk: clock.into(),
            rst: reset.into(),
            length: 0,
            mem: false,
            rdata: Box::new(E::Null),
        }
    }
    pub fn print(&self) -> String {
        let tmp = self.clone();
        let addr_width = ((tmp.length as f32).log2().ceil()) as i32;

        let mut st = String::new();

        st += "    reg            r_axi_awv_awr_flag;\n";
        st += "    reg            r_axi_arv_arr_flag;\n";
        st += "    reg    [7:0]   r_axi_awlen_count;\n";
        st += "    reg    [7:0]   r_axi_arlen_count;\n";
        st += "    reg    [1:0]   r_axi_arburst;\n";
        st += "    reg    [1:0]   r_axi_awburst;\n\n";

        if tmp.mem {
            st += &format!("    reg [31:0] axi_mem [0:{}];\n", tmp.length - 1);
            st += &format!(
                "    always @( posedge {} ) begin\n",
                _StrOut(tmp.clone().clk)
            );
            st += &format!("        if ( r_axi_wready & w_axi_wvalid ) begin\n");
            st += &format!("            axi_mem[r_axi_awaddr] <= w_axi_wdata;\n");
            st += &format!("        end else if ( axis_wen ) begin\n");
            st += &format!("            axi_mem[axis_addr] <= axis_write;\n");
            st += &format!("        end\n    end\n\n");
        } else {
            st += "    assign axis_wen = r_axi_wready & w_axi_wvalid;\n";
            st += &format!("    assign axis_addr = (r_axi_awv_awr_flag) ? r_axi_awaddr[{0}:2] : \n                       (r_axi_arv_arr_flag) ? r_axi_araddr[{0}:2] : 0;", addr_width+1);
        }

        st += "    // awready - awv_awr_flag generating\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
        st += &format!(
            "            r_axi_awready <= 1'b0;\n            r_axi_awv_awr_flag <= 1'b0;\n"
        );
        st += "        end else begin\n";
        st += &format!("            if (~r_axi_awready && w_axi_awvalid && ~r_axi_awv_awr_flag && ~r_axi_arv_arr_flag ) begin\n");
        st += &format!(
            "                r_axi_awready <= 1'b1;\n            r_axi_awv_awr_flag <= 1'b1;\n"
        );
        st += &format!("            end else if ( w_axi_wlast && r_axi_wready ) begin\n                r_axi_awv_awr_flag <= 1'b0;\n");
        st += &format!("            end else begin\n                r_axi_awready <= 1'b0;\n");
        st += "            end\n        end\n    end\n\n";

        st += "    // waddress generation\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
        st += &format!("            r_axi_awaddr <= 0;\n            r_axi_awlen_count <= 0;\n            r_axi_awburst <= 0;\n            r_axi_awlen <= 0;\n");
        st += "        end else begin\n";
        st += &format!(
            "            if ( ~r_axi_awready && w_axi_awvalid && ~r_axi_awv_awr_flag ) begin\n"
        );
        st += &format!("                r_axi_awaddr <= i_saxi_awaddr;\n                r_axi_awburst <= i_saxi_awburst;\n                r_axi_awlen <= i_saxi_awlen;\n                r_axi_awlen_count <= 0;\n");
        st += &format!("            end else if ( ( r_axi_awlen_count <= r_axi_awlen ) && r_axi_wready && w_axi_wvalid ) begin\n");
        st += &format!("                r_axi_awlen_count <= r_axi_awlen_count + 1;\n\n");
        st += &format!("                case ( r_axi_awburst )\n");
        st += &format!("                    2'b00: begin\n                        r_axi_awaddr <= r_axi_awaddr;\n                    end\n");
        st += &format!("                    2'b01: begin\n                        r_axi_awaddr[{0}:2] <= r_axi_awaddr[{0}:2] + 1;\n                        r_axi_awaddr[1:0] <= 2'b00;\n                    end\n", addr_width+1);
        st += &format!("                    default: begin\n                        r_axi_awaddr <= r_axi_awaddr[{0}:2] + 1;\n                    end\n                endcase\n", addr_width+1);
        st += "            end\n        end\n    end\n\n";

        st += "    // wready generation\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
        st += &format!("            r_axi_wready <= 0;\n");
        st += "        end else begin\n";
        st += &format!("            if ( ~r_axi_wready && w_axi_wvalid && r_axi_awv_awr_flag ) begin\n                r_axi_wready <= 1'b1;\n            end else begin\n                r_axi_wready <= 1'b0;\n            end\n");
        st += "        end\n    end\n\n";

        st += "    // write response generation\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
        st += &format!("            r_axi_bvalid <= 0;\n");
        st += "        end else begin\n";
        st += &format!("            if ( r_axi_awv_awr_flag && r_axi_wready && w_axi_wvalid && ~r_axi_bvalid && w_axi_wlast ) begin\n");
        st += &format!("                r_axi_bvalid <= 1'b1;\n");
        st += &format!("            end else begin\n");
        st += &format!("                if ( w_axi_bready && r_axi_bvalid ) begin\n                    r_axi_bvalid <= 1'b0;\n                end\n");
        st += "            end\n        end\n    end\n\n";

        st += "    // arready - arv_arr_flag generation\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
        st += &format!(
            "            r_axi_arready <= 1'b0;\n            r_axi_arv_arr_flag <= 1'b0;\n"
        );
        st += "        end else begin\n";
        st += &format!("            if ( ~r_axi_arready && w_axi_arvalid && ~r_axi_awv_awr_flag && ~r_axi_arv_arr_flag ) begin\n");
        st += &format!(
            "                r_axi_arready <= 1'b1;\n                r_axi_arv_arr_flag <= 1'b1;\n"
        );
        st += &format!("            end else if ( r_axi_rvalid && w_axi_rready && r_axi_arlen_count == r_axi_arlen ) begin\n                r_axi_arv_arr_flag <= 1'b0;\n");
        st += &format!("            end else begin\n                r_axi_arready <= 1'b0;\n");
        st += "            end\n        end\n    end\n\n";

        st += "    // raddress generation\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
        st += &format!("            r_axi_araddr <= 0;\n            r_axi_arlen_count <= 0;\n            r_axi_arburst <= 0;\n            r_axi_arlen <= 0;\n            r_axi_rlast <= 0;\n");
        st += "        end else begin\n";
        st += &format!(
            "            if ( ~r_axi_arready && w_axi_arvalid && ~r_axi_arv_arr_flag ) begin\n"
        );
        st += &format!("                r_axi_araddr <= i_saxi_araddr;\n                r_axi_arburst <= i_saxi_arburst;\n                r_axi_arlen <= i_saxi_arlen;\n                r_axi_arlen_count <= 0;\n                r_axi_rlast <= 0;\n");
        st += &format!("            end else if ( ( r_axi_arlen_count <= r_axi_arlen ) && r_axi_rvalid && w_axi_rready ) begin\n");
        st += &format!("                r_axi_arlen_count <= r_axi_arlen_count + 1;\n                r_axi_rlast <= 0;\n");
        st += &format!("                case ( r_axi_arburst )\n");
        st += &format!("                    2'b00: begin\n                        r_axi_araddr <= r_axi_araddr;\n                    end\n");
        st += &format!("                    2'b01: begin\n                        r_axi_araddr[{0}:2] <= r_axi_araddr[{0}:2] + 1;\n                        r_axi_araddr[1:0] <= 2'b00;\n                    end\n", addr_width+1);
        st += &format!("                    default: begin\n                        r_axi_araddr <= r_axi_araddr[{0}:2];\n                    end\n                endcase\n", addr_width+1);
        st += &format!("            end else if ( ( r_axi_arlen_count == r_axi_arlen ) && ~r_axi_rlast && r_axi_arv_arr_flag ) begin\n                r_axi_rlast <= 1'b1;\n");
        st += &format!(
            "            end else if ( w_axi_rready ) begin\n                r_axi_rlast <= 1'b0;\n"
        );
        st += "            end\n        end\n    end\n\n";

        st += "    // rvalid generation\n";
        st += &format!(
            "    always @( posedge {} ) begin\n",
            _StrOut(tmp.clone().clk)
        );
        st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
        st += &format!("            r_axi_rvalid <= 0;\n");
        st += "        end else begin\n";
        st += &format!("            if ( ~r_axi_wready && w_axi_wvalid && r_axi_awv_awr_flag ) begin\n                r_axi_rvalid <= 1'b1;\n            end else begin\n                r_axi_rvalid <= 1'b0;\n            end\n");
        st += "        end\n    end\n\n";

        st += "    assign w_axi_wdata[0+:8] = i_saxi_wstrb[0] ? i_saxi_wdata[0+:8] : 0;\n";
        st += "    assign w_axi_wdata[8+:8] = i_saxi_wstrb[1] ? i_saxi_wdata[8+:8] : 0;\n";
        st += "    assign w_axi_wdata[16+:8] = i_saxi_wstrb[2] ? i_saxi_wdata[16+:8] : 0;\n";
        st += "    assign w_axi_wdata[24+:8] = i_saxi_wstrb[3] ? i_saxi_wdata[24+:8] : 0;\n";

        if tmp.mem {
            st += "\n";
            st += &format!(
                "    always @( posedge {} ) begin\n",
                _StrOut(tmp.clone().clk)
            );
            st += &format!(
                "        r_axi_rdata <= axi_mem[r_axi_araddr[{}:2]];\n",
                addr_width + 1
            );
            st += &format!("        axis_read <= axi_mem[axis_addr];\n");
            st += &format!("    end\n\n");
        } else {
            st += "\n";
            st += &format!("    always @(*) begin\n");
            if let E::Null = *(tmp.clone().rdata) {
                st += &format!("        r_axi_rdata <= axis_read;\n");
            } else {
                st += &format!("        r_axi_rdata <= {};\n", _StrOut(tmp.clone().rdata));
            }
            st += &format!("    end\n\n");
            st += &format!("    assign axis_write = w_axi_wdata;\n");
        }

        return st;
    }
}

impl AxiSlaveReg<AxiLite> for AxiLite {
    fn order_reg_set(&mut self, num: i32) -> AxiLite {
        for x in 0..num {
            let Regname = format!("{}{}", "slv_reg".to_string(), x.to_string());
            let reg = WireVar::new().reg(&Regname, 32);
            self.reg_array.push(reg);
            self.wLocal_write
                .push((Box::new(E::Null), Box::new(E::Null)));
        }
        self.current_reg = num - 1;
        self.clone()
    }
}

impl AxiLite {
    pub fn named_reg_set(&mut self, name: &str) -> AxiLite {
        let reg = WireVar::new().reg(name, 32);
        self.reg_array.push(reg);
        self.wLocal_write
            .push((Box::new(E::Null), Box::new(E::Null)));
        self.current_reg = self.reg_array.len() as i32 - 1;
        self.clone()
    }

    pub fn named_reg(&mut self, name: &str) -> Box<E> {
        let SelfReg = self.reg_array.clone();
        for x in SelfReg {
            let Nx = *x.clone();
            if let E::Ldc(i) = Nx {
                if i.name == name.to_string() {
                    return x;
                }
            }
        }
        return Box::new(E::Null);
    }

    pub fn order_reg(&mut self, num: i32) -> Box<E> {
        let SelfReg = self.reg_array.clone();
        return SelfReg[num as usize].clone();
    }
}

impl AxiSlaveReg<Axi4Slave> for Axi4Slave {
    fn order_reg_set(&mut self, num: i32) -> Axi4Slave {
        self.length = num;
        self.clone()
    }
}

impl AXIStreamRegCtrl for Axi4Slave {
    fn write(&mut self) -> Box<E> {
        WireVar::new().wire("axis_write", 32)
    }

    fn addr(&mut self) -> Box<E> {
        WireVar::new().wire("axis_addr", 32)
    }

    fn wen(&mut self) -> Box<E> {
        WireVar::new().wire("axis_wen", 1)
    }

    fn mem_if(&mut self) -> (Box<E>, Box<E>, Box<E>, Box<E>) {
        self.mem = true;
        (
            WireVar::new().wire("axis_read", 32),
            WireVar::new().wire("axis_write", 32),
            WireVar::new().wire("axis_wen", 1),
            WireVar::new().wire("axis_addr", 32),
        )
    }
}

pub trait AxiSlaveReadcontrol<T>
where
    T: Into<Box<E>>,
{
    fn read(&mut self, rdata: T) -> Axi4Slave;
}

impl<T> AxiSlaveReadcontrol<T> for Axi4Slave
where
    T: Into<Box<E>>,
{
    fn read(&mut self, rdata: T) -> Axi4Slave {
        self.rdata = rdata.into();
        self.clone()
    }
}

impl<T, U> AxiSlaveLocalWrite<T, U> for AxiLite
where
    T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    fn reg_write(&mut self, write_en: U, write_data: T) {
        // localwrite AXI Register
        self.wLocal_write[self.current_reg.clone() as usize] = (write_en.into(), write_data.into());
        return;
    }
}

/// AST分解メソッド
pub fn _Decomp<T: Into<Box<E>>>(e: T, Sel: &str) -> Box<E> {
    let m = *e.into();
    match m {
        E::Bin(_, ref L, ref R) => {
            if Sel == "L" {
                Box::new(*L.clone())
            } else if Sel == "R" {
                Box::new(*R.clone())
            } else {
                Box::new(E::Null)
            }
        }
        E::PL(ref D, ref T, ref F) => {
            if Sel == "D" {
                Box::new(*D.clone())
            } else if Sel == "T" {
                Box::new(*T.clone())
            } else if Sel == "F" {
                Box::new(*F.clone())
            } else {
                Box::new(E::Null)
            }
        }
        E::SB(ref L, ref R) => {
            if Sel == "L" {
                Box::new(*L.clone())
            } else if Sel == "R" {
                Box::new(*R.clone())
            } else {
                Box::new(E::Null)
            }
        }
        _ => Box::new(E::Null),
    }
}

/// AST文字列抽出メソッド
pub fn _StrOut<T: Into<Box<E>>>(e: T) -> String {
    let m = *e.into();
    match m {
        E::Ldc(WR) => WR.name,
        E::Bin(ref Op, _, _) => Op.clone(),
        _ => "Null".to_string(),
    }
}

/// AST数値抽出メソッド
pub fn _NumOut<T: Into<Box<E>>>(e: T) -> i32 {
    let m = *e.into();
    match m {
        E::Ldc(WR) => WR.width,
        E::Num(i) => i,
        _ => 0,
    }
}
