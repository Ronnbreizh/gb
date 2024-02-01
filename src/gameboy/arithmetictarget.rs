#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum WideArithmeticTarget {
    // Double register
    HL,
    BC,
    DE,
    AF,
    // STACK
    SP,
    // MEMORY
    ReadWord,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum ArithmeticTarget {
    // registers
    A,
    B,
    C,
    D,
    E,
    H,
    L,

    // Read one byte
    ReadByte,

    // Read one byte in PC, then access `0xFF00 + byte`
    FFRead,

    // Access `0xFF00 + content of register C`
    FFC,

    // A double register can be interpreted as a pointer to the heap
    // Zone pointed by BC register
    BCTarget,
    // Zone pointed by DE register
    DETarget,
    // Zone pointed by HL register
    HLTarget,

    // CHECKME
    // read/write then decrement
    HLDec,
    // read/write then increment
    HLInc,
}
