use crate::state::{Address, Word};

type Register = usize;

#[derive(Debug)]
pub enum Instruction {
    ClearDisplay,
    Return,
    Jump(Address),
    Call(Address),
    SkipEqual(Register, Word),
    SkipNotEqual(Register, Word),
    SkipEqualXY(Register, Register),
    Load(Register, Word),
    Add(Register, Word),

    Move(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    Xor(Register, Register),
    AddXY(Register, Register),
    SubXY(Register, Register),
    ShiftRight(Register),
    SubYX(Register, Register),
    ShiftLeft(Register),

    SkipNotEqualXY(Register, Register),
    LoadI(Address),
    JumpV0(Address),
    Random(Register, Word),
    Draw(Register, Register, Word),

    SkipKeyPressed(Register),
    SkipKeyNotPressed(Register),

    LoadDelay(Register),
    WaitKeyPress(Register),
    SetDelay(Register),
    SetSound(Register),
    AddI(Register),
    LoadFont(Register),
    StoreBCD(Register),
    StoreRegisters(Register),
    LoadMemory(Register),
}
