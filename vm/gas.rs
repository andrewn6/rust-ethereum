#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Gas(pub u64);

impl Gas {
    pub const GasQuickStep: Gas = Gas(2);
    pub const GasFatestStep: Gas = Gas(3);
    pub const GasFastStep: Gas = Gas(5);
    pub const GasMidStep: Gas = Gas(8);
    pub const GasSlowStep: Gas = Gas(10);
    pub const GasExtStep: Gas = Gas(20);
}

fn callGas () {
}
