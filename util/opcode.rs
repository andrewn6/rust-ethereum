#[derive(Clone, Copy, Debug, Eq, PartialEq])
pub struct Opcode(pub u8);

impl Opcode {
  pub const STOP: Opcode = Opcode(0x00);
  pub const ADD: Opcode = Opcode(0x01);
  pub const MUL: Opcode = Opcode(0x02);
  pub const SUB: Opcode = Opcode(0x03);
  pub const DIV: Opcode = Opcode(0x04);
  pub const SDIV: Opcode = Opcode(0x05);
} 
