#[derive(Clone, Copy, Debug, Eq, PartialEq])
pub struct Opcode(pub u8);

impl Opcode {
  // 0x0 range
  pub const STOP: Opcode = Opcode(0x00);
  pub const ADD: Opcode = Opcode(0x01);
  pub const MUL: Opcode = Opcode(0x02);
  pub const SUB: Opcode = Opcode(0x03);
  pub const DIV: Opcode = Opcode(0x04);
  pub const SDIV: Opcode = Opcode(0x05);
  pub const MOD: Opcode = Opcode(0x06);
  pub const SMOD: Opcode = Opcode(0x07);
  pub const ADDMOD: Opcode = Opcode(0x08);
  pub const MULMOD: Opcode = Opcode(0x09);
  pub const EXP: Opcode = Opcode(0xa);
  pub const SIGNEXTEND: Opcode = Opcode(0xb);
    
  // 0x10 range
  pub const LT: Opcode = Opcode(0x10);
  pub const GT: Opcode = Opcode(0x11);
  pub const SLT: Opcode = Opcode(0x12);
  pub const SGT: Opcode = Opcode(0x13);
  pub const EQ: Opcode = Opcode(0x14);
  pub const ISZERO: Opcode = Opcode(0x15);
  pub const AND: Opcode = Opcode(0x16);
  pub const OR: Opcode = Opcode(0x17);
  pub const XOR: Opcode = Opcode(0x18;)
  pub const NOT: Opcode = Opcode(0x19);
  pub const BYTE: Opcode = Opcode(0x1a);
  pub const SHL: Opcode = Opcode(0x1b);
  pub const SHR: Opcode = Opcode(0x1c);
  pub const SAR: Opcode = Opcode(0x1d);

  pub const KECCAK256: Opcode = Opcode(0x20);

  // 0x30 range
  pub const ADDRESS: Opcode = Opcode(0x30);
  pub const BALANCE: Opcode = Opcode(0x31);
  pub const ORIGIN: Opcode = Opcode(0x32);
  pub const CALLER: Opcode = Opcode(0x33); 
  pub const CALLVALUE: Opcode = Opcode(0x34);
  pub const CALLDATALOAD: Opcode = Opcode(0x35);
  pub const CALLDATASIZE: Opcode = Opcode(0x36);
  pub const CALLDATACOPY: Opcode = Opcode(0x37);
  pub const CODESIZE: Opcode = Opcode(0x38);
  pub const CODECOPY: Opcode = Opcode(0x39);
  pub const GASPRICE: Opcode = Opcode(0x3a);
  pub const EXTCODESIZE: Opcode = Opcode(0x3b);
  pub const EXTCODECOPY: Opcode = Opcode(0x3c);
  pub const RETURNDATASIZE: Opcode = Opcode(0x3d);
  pub const RETURNDATACOPY: Opcode = Opcode(0x3e);
  pub const EXTCODEHASH: Opcode = Opcode(0x3f);

  // 0x40
  pub const BLOCKHASH: Opcode = Opcode(0x40);
  pub const COINBASE: Opcode = Opcode(0x41);
  pub const TIMESTAMP: Opcode = Opcode(0x42);
  pu const NUMBER: Opcode = Opcode(0x43);
  pub const DIFFICULTY: Opcode = Opcode(0x44);
  pub const GASLIMIT: Opcode = Opcode(0x45);
 }
