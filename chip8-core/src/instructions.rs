type Address = u16;
type Register = usize;

#[derive(Debug)]
pub enum Instruction {
    ClearDisplay,
    Return,
    Jump(Address),
    Call(Address),
    SkipEqual(Register, u8),
    SkipNotEqual(Register, u8),
    SkipEqualXY(Register, Register),
    Load(Register, u8),
    Add(Register, u8),

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
    Random(Register, u8),
    Draw(Register, Register, u8),

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
