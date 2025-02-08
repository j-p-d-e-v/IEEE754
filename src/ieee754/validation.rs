#[derive(Debug)]
pub enum ValidationError {
    ExponentAll1s,
    MantissaAll0s,
    InvalidBitLength,
    ParseError,
    EmptyValues,
    InvalidSignBit,
    InvalidExponent,
    InvalidMantissa,
    InvalidMSBMantissa,
    InvalidLSBMantissa,
    EmptySignBit,
    EmptyExponent,
    EmptyMantissa,
}
