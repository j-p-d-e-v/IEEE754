pub mod ieee754;

use crate::ieee754::{IEEE754_32bit, IEEE754_64bit, ValidationError};

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
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    #[test]
    fn test_32bit() {
        // 0.0
        let values = vec![0x00, 0x00, 0x00, 0x00];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", 0.0);
        assert_eq!(0.0, test.to_32bit().unwrap());

        // -0.0
        let values = vec![0x80, 0x00, 0x00, 0x00];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -0.0);
        assert_eq!(-0.0, test.to_32bit().unwrap());

        // -2.7182817
        let values = vec![0xc0, 0x2d, 0xf8, 0x54];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -2.7182817);
        assert_eq!(-2.7182817, test.to_32bit().unwrap());

        // Infinity (Positive)
        let values = vec![0x7f, 0x80, 0x00, 0x00];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error(Infinity)");
        assert!(test.to_32bit().is_err());

        // Infinity (Negative)
        let values = vec![0xff, 0x80, 0x00, 0x00];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error(Infinity)");
        assert!(test.to_32bit().is_err());

        // NaN
        let values = vec![0x7f, 0xc0, 0x00, 0x00];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error(NaN)");
        assert!(test.to_32bit().is_err());
    }

    #[test]
    fn test_64bit() {
        // -74.74597276138431
        let values = vec![0xc0, 0x52, 0xaf, 0xbe, 0x4, 0x89, 0x76, 0x8e];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -74.74597276138431);
        assert_eq!(-74.74597276138431, test.to_64bit().unwrap());

        // -3.125
        let values = vec![0xc0, 0x09, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -3.125);
        assert_eq!(-3.125, test.to_64bit().unwrap());

        // Infinity
        let values = vec![0x7f, 0xf0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(test.to_64bit().is_err());

        // -Infinity
        let values = vec![0xff, 0xf0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(test.to_64bit().is_err());

        // Quiet NaN
        // InProgress
        let values = vec![0x7f, 0xf8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(test.to_64bit().is_err());

        // Signal NaN
        let values = vec![0x7f, 0xf4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(test.to_64bit().is_err());

        // 0.0
        let values = vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", 0.0);
        assert_eq!(0.0, test.to_64bit().unwrap());

        // 3.141592653589793
        let values = vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", f64::consts::PI);
        assert_eq!(f64::consts::PI, test.to_64bit().unwrap());

        // 2.718281828459045
        let values = vec![0x40, 0x05, 0xbf, 0x0a, 0x8b, 0x14, 0x57, 0x69];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", f64::consts::E);
        assert_eq!(f64::consts::E, test.to_64bit().unwrap());
    }
}
