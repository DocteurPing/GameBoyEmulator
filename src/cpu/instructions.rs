pub(crate) enum Instruction {
    ADD(ArithmeticTarget),
}

pub(crate) enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}
