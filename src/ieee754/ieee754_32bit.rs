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
        // Infitnity Validation
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
}
