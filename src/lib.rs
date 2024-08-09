#[cfg(test)]
mod tests {
    use vcore::*;
    #[test]
    fn it_works() {
        let mut m = VModule::new("LED");
        m.input("CLK", 1);
        m.input("RST", 1);
        assert!(!m.gen().is_empty(), "Code not generated successfully...");
    }
}

pub mod bus;
pub mod vcore;
