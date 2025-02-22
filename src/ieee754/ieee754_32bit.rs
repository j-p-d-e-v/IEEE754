use crate::helper::{ComputeMantissaBits, SplitFloat};
use crate::ieee754::validation::ValidationError;
#[derive(Debug)]
pub struct IEEE754_32bit {}

impl IEEE754_32bit {
    pub fn validate(
        exponent_binaries: &[u8],
        mantissa_binaries: &[u8],
    ) -> Result<(), ValidationError> {
        if exponent_binaries.len() != 8 {
            return Err(ValidationError::InvalidBitLength);
        }
        // Infinity Validation
        let all_1s: &[u8; 8] = &[1; 8];
        let all_0s: &[u8; 8] = &[0; 8];
        if exponent_binaries == all_1s {
            return Err(ValidationError::ExponentAll1s);
        }
        if mantissa_binaries == all_0s {
            return Err(ValidationError::MantissaAll0s);
        }

        Ok(())
    }

    pub fn get_exponent(binaries: &[u8]) -> Result<i32, ValidationError> {
        let value_str: String = binaries
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .concat();

        match u32::from_str_radix(&value_str, 2) {
            Ok(value) => {
                if value == 0 {
                    return Ok(0);
                }
                let bias: i32 = 127;
                let actual_exponent: i32 = value as i32 - bias;
                Ok(actual_exponent)
            }
            Err(_error) => Err(ValidationError::ParseError),
        }
    }

    pub fn get_mantissa(binaries: &[u8], exponent: i32) -> Result<f32, ValidationError> {
        if binaries == [0; 23] && exponent == 0 {
            return Ok(0.0);
        }
        match binaries.get(0..exponent as usize) {
            Some(msb_values) => {
                let mut msb_values: Vec<u8> = msb_values.to_vec();
                msb_values.insert(0, 1);
                let msb_value_str = msb_values
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .concat();
                match i32::from_str_radix(&msb_value_str, 2) {
                    Ok(msb_value) => match binaries.get(exponent as usize..) {
                        Some(lsb_values) => {
                            let mut lsb_value: f32 = 0.0;
                            for (index, value) in lsb_values.iter().enumerate() {
                                lsb_value +=
                                    value.to_owned() as f32 * 2f32.powi(-(index as i32 + 1));
                            }
                            Ok(msb_value as f32 + lsb_value)
                        }
                        None => Err(ValidationError::InvalidLSBMantissa),
                    },
                    Err(_) => Err(ValidationError::InvalidMSBMantissa),
                }
            }
            None => Err(ValidationError::EmptyMantissa),
        }
    }

    pub fn get_binary(value: f32) -> Result<Vec<u8>, String> {
        let split_float: (u8, u32, f32) = SplitFloat::f32(value)?;

        let sign_bit: u8 = split_float.0;

        // Integer Part
        let mut integer_part: u32 = split_float.1;
        let mut integer_part_bin: Vec<u8> = Vec::new();
        loop {
            if integer_part == 0 {
                break;
            }
            let remainder: u8 = (integer_part % 2) as u8;
            integer_part_bin.push(remainder);
            integer_part = integer_part / 2;
        }

        // Fractional Part
        let mut fractional_part: f32 = split_float.2;
        let mut fractional_part_bin: Vec<u8> = Vec::new();
        loop {
            if fractional_part == 0.0 || fractional_part_bin.len() > 23 {
                break;
            }
            fractional_part = fractional_part * 2.0;
            let front_number: u8 = fractional_part as u8;
            fractional_part_bin.push(front_number);
            fractional_part = SplitFloat::f32(fractional_part)?.2;
        }

        let bias: i32 = 127;
        let mut exponent: i32 = 0;

        if integer_part_bin.len() > 0 {
            exponent = (integer_part_bin.len() - 1) as i32;
            exponent = exponent + bias;
        }

        let mut exponent_bin: Vec<u8> = Vec::new();
        loop {
            if exponent == 0 || exponent_bin.len() == 8 {
                break;
            }
            let remainder: u8 = (exponent % 2) as u8;
            exponent_bin.push(remainder);
            exponent = exponent / 2;
        }
        if exponent_bin.len() > 0 {
            exponent_bin.reverse();
        }
        if integer_part_bin.len() > 0 {
            integer_part_bin.reverse();
            integer_part_bin.remove(0);
        }

        let mut mantissa_bin: Vec<u8> = Vec::new();
        mantissa_bin.append(&mut integer_part_bin);
        mantissa_bin.append(&mut fractional_part_bin);
        if mantissa_bin.len() > 23 {
            mantissa_bin = ComputeMantissaBits::compute(mantissa_bin, 23usize)?;
        }
        let mut binary: Vec<u8> = Vec::new();
        binary.push(sign_bit);
        binary.append(&mut exponent_bin);
        binary.append(&mut mantissa_bin);

        binary.resize(32, 0);
        Ok(binary)
    }
}
