#[allow(clippy::upper_case_acronyms)]
pub enum ArithmeticTarget {
    // registers
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    // Double register
    HL,
    BC,
    DE,
    AF,

    // STACK
    SP,

    // HEAP

    // A double register can be interpreted as a pointer to the heap
    // Zone pointed by BC register
    BCH,
    // Zone pointed by DE register
    DEH,

    // Zone pointed by HL register
    // read/write to the heap
    HLH,
    // read/write then decrement
    HLDec,
    // read/write then increment
    HLInc,
}
