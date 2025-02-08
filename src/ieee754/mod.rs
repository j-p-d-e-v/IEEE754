pub mod ieee754_32bit;
pub mod ieee754_64bit;
pub mod validation;

pub use ieee754_32bit::IEEE754_32bit;
pub use ieee754_64bit::IEEE754_64bit;
pub use validation::ValidationError;

#[derive(Debug)]
pub struct IEEE754 {
    values: Vec<u32>,
}

impl IEEE754 {
    pub fn new(values: Vec<u32>) -> Self {
        Self { values }
    }

    pub fn to_binary(&self) -> Result<Vec<u8>, ValidationError> {
        let mut binaries: Vec<u8> = Vec::new();
        if self.values.is_empty() {
            return Err(ValidationError::EmptyValues);
        }
        for v in self.values.iter() {
            for x in &mut format!("{:08b}", v).chars() {
                match u8::from_str_radix(&x.to_string(), 2) {
                    Ok(value) => binaries.push(value),
                    Err(_) => return Err(ValidationError::ParseError),
                }
            }
        }
        Ok(binaries)
    }

    fn get_sign_bit(&self, binaries: &[u8]) -> Result<i8, ValidationError> {
        if binaries.is_empty() {
            return Err(ValidationError::EmptySignBit);
        }
        match binaries.first() {
            Some(value) => Ok(if value == &0 { 1 } else { -1 }),
            None => Err(ValidationError::InvalidSignBit),
        }
    }

    pub fn to_64bit(&self) -> Result<f64, ValidationError> {
        let binaries: Vec<u8> = self.to_binary()?;
        let sign_bit: i8 = self.get_sign_bit(&binaries)?;

        // Exponent
        match binaries.get(1..12) {
            Some(exponent_binaries) => {
                // Mantissa
                match binaries.get(12..) {
                    Some(mantissa_binaries) => {
                        IEEE754_64bit::validate(exponent_binaries, mantissa_binaries)?;
                        let exponent: i32 = IEEE754_64bit::get_exponent(exponent_binaries)?;
                        let value: f64 = IEEE754_64bit::get_mantissa(mantissa_binaries, exponent)?;
                        match sign_bit {
                            1 => Ok(value),
                            -1 => Ok(-(value)),
                            _ => Err(ValidationError::InvalidSignBit),
                        }
                    }
                    None => Err(ValidationError::InvalidMantissa),
                }
            }
            None => Err(ValidationError::InvalidExponent),
        }
    }

    pub fn to_32bit(&self) -> Result<f32, ValidationError> {
        let binaries: Vec<u8> = self.to_binary()?;
        let sign_bit: i8 = self.get_sign_bit(&binaries)?;

        // Exponent
        match binaries.get(1..9) {
            Some(exponent_binaries) => {
                // Mantissa
                match binaries.get(9..) {
                    Some(mantissa_binaries) => {
                        IEEE754_32bit::validate(exponent_binaries, mantissa_binaries)?;
                        let exponent: i32 = IEEE754_32bit::get_exponent(exponent_binaries)?;
                        let value: f32 = IEEE754_32bit::get_mantissa(mantissa_binaries, exponent)?;
                        match sign_bit {
                            1 => Ok(value),
                            -1 => Ok(-(value)),
                            _ => Err(ValidationError::InvalidSignBit),
                        }
                    }
                    None => Err(ValidationError::InvalidMantissa),
                }
            }
            None => Err(ValidationError::InvalidExponent),
        }
    }
}
