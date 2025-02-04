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
                        if let Err(error) =
                            IEEE754_64bit::validate(exponent_binaries, mantissa_binaries)
                        {
                            return Err(error);
                        }
                        let exponent: f64 = IEEE754_64bit::get_exponent(exponent_binaries)?;
                        let mantissa: f64 = IEEE754_64bit::get_mantissa(mantissa_binaries)?;
                        let value = exponent * mantissa;
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

    pub fn to_32bit(&self) {
        todo!("not implemented")
    }
}

#[derive(Debug)]
pub enum ValidationError {
    ExponentAll1s,
    MantissaAll0s,
    InvalidBitLength,
    InvalidMantissaFirstBitValue,
    ParseError,
    EmptyValues,
    InvalidSignBit,
    InvalidExponent,
    InvalidMantissa,
    EmptySignBit,
    EmptyExponent,
    EmptyMantissa,
}

#[derive(Debug)]
pub struct IEEE754_64bit {}

impl IEEE754_64bit {
    pub fn validate(
        exponent_binaries: &[u8],
        mantissa_binaries: &[u8],
    ) -> Result<(), ValidationError> {
        if exponent_binaries.len() != 11 {
            return Err(ValidationError::InvalidBitLength);
        }
        // Infitnity Validation
        let all_1s: &[u8; 11] = &[1; 11];
        let all_0s: &[u8; 11] = &[0; 11];
        if exponent_binaries == all_1s {
            return Err(ValidationError::ExponentAll1s);
        }
        if mantissa_binaries == all_0s {
            return Err(ValidationError::MantissaAll0s);
        }

        Ok(())
    }

    pub fn get_exponent(binaries: &[u8]) -> Result<f64, ValidationError> {
        let value_str: String = binaries
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .concat();
        match u32::from_str_radix(&value_str, 2) {
            Ok(value) => {
                let bias: u32 = 1023;
                let exponent: f64 = 2i32.pow(value - bias) as f64;
                Ok(exponent)
            }
            Err(_error) => Err(ValidationError::ParseError),
        }
    }

    pub fn get_mantissa(binaries: &[u8]) -> Result<f64, ValidationError> {
        let first_bit: &u8 = match binaries.first() {
            Some(value) => value,
            None => return Err(ValidationError::InvalidMantissaFirstBitValue),
        };
        let starting_index: usize = if first_bit == &0 { 1 } else { 0 };

        match binaries.get(starting_index..) {
            Some(values) => {
                let mut mantissa: f64 = 1.0;

                for (index, value) in values.iter().enumerate() {
                    mantissa += value.to_owned() as f64 * 2f64.powi(-(index as i32 + 1));
                }
                Ok(mantissa)
            }
            None => Err(ValidationError::EmptyMantissa),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_64bit() {
        // -85.49194552276862
        let values = vec![0xc0, 0x52, 0xaf, 0xbe, 0x4, 0x89, 0x76, 0x8e];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", -85.49194552276862);
        assert_eq!(-85.49194552276862, test.to_64bit().unwrap());

        // 3.141592653589793
        let values = vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18];
        let test: IEEE754 = IEEE754::new(values.clone());
        println!("Input: {:x?}", values);
        println!("Expected Output: {}", 3.141592653589793);
        assert_eq!(3.141592653589793f64, test.to_64bit().unwrap());

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
        // InProgress
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
    }
}
