pub mod helper;
pub mod ieee754;

use crate::ieee754::{IEEE754_32bit, IEEE754_64bit, ValidationError};

#[derive(Debug)]
pub struct IEEE754;

impl IEEE754 {
    pub fn to_hex(binary: Vec<u8>) -> Result<String, String> {
        let mut hex = String::new();
        for b in binary.chunks(4) {
            let byte_str: String = b
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .concat();

            match u8::from_str_radix(&byte_str, 2) {
                Ok(v) => {
                    hex.push_str(&format!("{:X}", v));
                }
                Err(error) => return Err(error.to_string()),
            }
        }
        Ok(hex)
    }

    pub fn to_32bit_hex(value: f32) -> Result<String, String> {
        let binary: Vec<u8> = IEEE754_32bit::get_binary(value)?;
        let hex: String = IEEE754::to_hex(binary)?;
        Ok(hex)
    }

    pub fn to_64bit_hex(value: f64) -> Result<String, String> {
        let binary: Vec<u8> = IEEE754_64bit::get_binary(value)?;
        let hex: String = IEEE754::to_hex(binary)?;
        Ok(hex)
    }

    pub fn to_binary(values: Vec<u32>) -> Result<Vec<u8>, ValidationError> {
        let mut binaries: Vec<u8> = Vec::new();
        if values.is_empty() {
            return Err(ValidationError::EmptyValues);
        }
        for v in values.iter() {
            for x in &mut format!("{:08b}", v).chars() {
                match u8::from_str_radix(&x.to_string(), 2) {
                    Ok(value) => binaries.push(value),
                    Err(_) => return Err(ValidationError::ParseError),
                }
            }
        }
        Ok(binaries)
    }

    fn get_sign_bit(binaries: &[u8]) -> Result<i8, ValidationError> {
        if binaries.is_empty() {
            return Err(ValidationError::EmptySignBit);
        }
        match binaries.first() {
            Some(value) => Ok(if value == &0 { 1 } else { -1 }),
            None => Err(ValidationError::InvalidSignBit),
        }
    }

    pub fn to_64bit_float(values: Vec<u32>) -> Result<f64, ValidationError> {
        let binaries: Vec<u8> = Self::to_binary(values)?;
        let sign_bit: i8 = Self::get_sign_bit(&binaries)?;

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
    pub fn to_32bit_float(values: Vec<u32>) -> Result<f32, ValidationError> {
        let binaries: Vec<u8> = Self::to_binary(values)?;
        let sign_bit: i8 = Self::get_sign_bit(&binaries)?;

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
    use crate::helper::ComputeMantissaBits;
    use std::f64;

    #[test]
    fn test_compute_mantissa_bits_computation() {
        assert_eq!(
            ComputeMantissaBits::compute(
                "10101010101010101010101 0 0"
                    .chars()
                    .filter(|&c| c != ' ')
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>(),
                23
            )
            .unwrap(),
            "10101010101010101010101"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        );
        assert_eq!(
            ComputeMantissaBits::compute(
                "10101010101010101010101 1 0"
                    .chars()
                    .filter(|&c| c != ' ')
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>(),
                23
            )
            .unwrap(),
            "10101010101010101010110"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        );

        assert!(ComputeMantissaBits::compute(
            "11111111111111111111111 1 0"
                .chars()
                .filter(|&c| c != ' ')
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
            23
        )
        .is_err());

        assert_eq!(
            ComputeMantissaBits::compute(
                "00000000000000000000000 1 0"
                    .chars()
                    .filter(|&c| c != ' ')
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>(),
                23
            )
            .unwrap(),
            "00000000000000000000000"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        );

        assert_eq!(
            ComputeMantissaBits::compute(
                "01111111111111111111111 1"
                    .chars()
                    .filter(|&c| c != ' ')
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>(),
                23
            )
            .unwrap(),
            "10000000000000000000000"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        );

        assert!(ComputeMantissaBits::compute(
            "11111111111111111111111 1"
                .chars()
                .filter(|&c| c != ' ')
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
            23
        )
        .is_err());
    }

    #[test]
    fn test_32bit() {
        // 0.0
        let values = vec![0x00, 0x00, 0x00, 0x00];
        let output = IEEE754::to_32bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", 0.0);
        assert_eq!(0.0, output.unwrap());

        // -0.0
        let values = vec![0x80, 0x00, 0x00, 0x00];
        let output = IEEE754::to_32bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -0.0);
        assert_eq!(-0.0, output.unwrap());

        // -2.7182817
        let values = vec![0xc0, 0x2d, 0xf8, 0x54];
        let output = IEEE754::to_32bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -2.7182817);
        assert_eq!(-2.7182817, output.unwrap());

        // Infinity (Positive)
        let values = vec![0x7f, 0x80, 0x00, 0x00];
        let output = IEEE754::to_32bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error(Infinity)");
        assert!(output.is_err());

        // Infinity (Negative)
        let values = vec![0xff, 0x80, 0x00, 0x00];
        let output = IEEE754::to_32bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error(Infinity)");
        assert!(output.is_err());

        // NaN
        let values = vec![0x7f, 0xc0, 0x00, 0x00];
        let output = IEEE754::to_32bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error(NaN)");
        assert!(output.is_err());
    }

    #[test]
    fn test_64bit_to_hex() {
        let values = 10.001;
        let output = IEEE754::to_64bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: 40240083126E978D");
        assert_eq!(output.unwrap(), "40240083126E978D");

        let values = -85.125;
        let output = IEEE754::to_64bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: C055480000000000");
        assert_eq!(output.unwrap(), "C055480000000000");

        let values = 0.0;
        let output = IEEE754::to_64bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: 0000000000000000");
        assert_eq!(output.unwrap(), "0000000000000000");

        let values = -33.33333333;
        let output = IEEE754::to_64bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: C040AAAAAAA38226");
        assert_eq!(output.unwrap(), "C040AAAAAAA38226");

        let values = -333.33333333;
        let output = IEEE754::to_64bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: C074D55555547045");
        assert_eq!(output.unwrap(), "C074D55555547045");

        let values = 333.33333333;
        let output = IEEE754::to_64bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: 4074D55555547045");
        assert_eq!(output.unwrap(), "4074D55555547045");
    }

    #[test]
    fn test_32bit_to_hex() {
        let values = 10.001;
        let output = IEEE754::to_32bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: 41200419");
        assert_eq!(output.unwrap(), "41200419");

        let values = -85.125;
        let output = IEEE754::to_32bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: C2AA4000");
        assert_eq!(output.unwrap(), "C2AA4000");

        let values = 0.0;
        let output = IEEE754::to_32bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: 00000000");
        assert_eq!(output.unwrap(), "00000000");

        let values = -33.33333333;
        let output = IEEE754::to_32bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: C2055555");
        assert_eq!(output.unwrap(), "C2055555");

        let values = -333.33333333;
        let output = IEEE754::to_32bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: C3A6AAAB");
        assert_eq!(output.unwrap(), "C3A6AAAB");

        let values = 333.33333333;
        let output = IEEE754::to_32bit_hex(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: 43A6AAAB");
        assert_eq!(output.unwrap(), "43A6AAAB");
    }

    #[test]
    fn test_64bit() {
        // -74.74597276138431
        let values = vec![0xc0, 0x52, 0xaf, 0xbe, 0x4, 0x89, 0x76, 0x8e];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -74.74597276138431);
        assert_eq!(-74.74597276138431, output.unwrap());

        // -3.125
        let values = vec![0xc0, 0x09, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -3.125);
        assert_eq!(-3.125, output.unwrap());

        // Infinity
        let values = vec![0x7f, 0xf0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(output.is_err());

        // -Infinity
        let values = vec![0xff, 0xf0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(output.is_err());

        // Quiet NaN
        // InProgress
        let values = vec![0x7f, 0xf8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(output.is_err());

        // Signal NaN
        let values = vec![0x7f, 0xf4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: Error");
        assert!(output.is_err());

        // 0.0
        let values = vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", 0.0);
        assert_eq!(0.0, output.unwrap());

        // 3.141592653589793
        let values = vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", f64::consts::PI);
        assert_eq!(f64::consts::PI, output.unwrap());

        // 2.718281828459045
        let values = vec![0x40, 0x05, 0xbf, 0x0a, 0x8b, 0x14, 0x57, 0x69];
        let output = IEEE754::to_64bit_float(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", f64::consts::E);
        assert_eq!(f64::consts::E, output.unwrap());
    }
}
