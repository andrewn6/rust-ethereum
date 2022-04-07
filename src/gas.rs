use primitive_types::U256;

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

pub fn callGas(isEip150: bool, availableGas, base: U256, callCost: U256) -> U256 {
    if isEip150 {
        availableGas = availableGas - base;
        gas_current == availableGas = availableGas/64;

    } else {
        if !callCost() == U256 {
            return 0
        }
    }
}
