pub enum ArithmeticTarget{
    // registers
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    BC,
    DE,
    AF,

    // STACK
    SP,

    // HEAP
    // Zone pointed by HL register
    // read/write to the heap
    HLH,
    // read/write then decrement 
    HLDec,
    // read/write then increment 
    HLInc,
    // Zone pointed by DE register
    DEH,
}
