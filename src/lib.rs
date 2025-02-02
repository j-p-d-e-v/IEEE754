#[derive(Debug)]
pub struct IEEE754 {
    values: Vec<u32>,
}

impl IEEE754 {
    pub fn new(values: Vec<u32>) -> Self {
        Self { values }
    }

    pub fn to_binary(&self) -> Result<Vec<u8>, String> {
        let mut binaries: Vec<u8> = Vec::new();
        if self.values.is_empty() {
            return Err("no values found".to_string());
        }
        for v in self.values.iter() {
            binaries.append(
                &mut format!("{:08b}", v)
                    .chars()
                    .map(|x| match u8::from_str_radix(&x.to_string(), 2) {
                        Ok(value) => value,
                        Err(error) => {
                            panic!("unable to parse value: {:?}", error);
                        }
                    })
                    .collect::<Vec<u8>>(),
            );
        }
        Ok(binaries)
    }

    fn get_sign_bit(&self, binaries: &[u8]) -> Result<i8, String> {
        if binaries.is_empty() {
            return Err("values are empty".to_string());
        }
        match binaries.first() {
            Some(value) => Ok(if value == &0 { 1 } else { -1 }),
            None => Err("invalid sign bit".to_string()),
        }
    }

    pub fn to_64bit(&self) -> Result<f64, String> {
        let binaries: Vec<u8> = self.to_binary()?;
        let sign_bit: i8 = self.get_sign_bit(&binaries)?;

        // Exponent
        match binaries.get(1..12) {
            Some(exponent_binaries) => {
                let exponent: f64 = IEEE754_64bit::get_exponent(exponent_binaries)?;
                // Mantissa
                match binaries.get(12..) {
                    Some(mantissa_binaries) => {
                        let mantissa: f64 = IEEE754_64bit::get_mantissa(mantissa_binaries)?;
                        let value = exponent * mantissa;
                        match sign_bit {
                            1 => Ok(value),
                            -1 => Ok(-(value)),
                            _ => Err("invalid sign bit".to_string()),
                        }
                    }
                    None => Err("binaries for mantissa not found".to_string()),
                }
            }
            None => Err("binaries for exponent not found".to_string()),
        }
    }

    pub fn to_32bit(&self) {
        todo!("not implemented")
    }
}

#[derive(Debug)]
pub struct IEEE754_64bit {}

impl IEEE754_64bit {
    pub fn get_exponent(binaries: &[u8]) -> Result<f64, String> {
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
            Err(_error) => Err("unable to parse exponent binaries".to_string()),
        }
    }

    pub fn get_mantissa(binaries: &[u8]) -> Result<f64, String> {
        let first_bit: &u8 = match binaries.first() {
            Some(value) => value,
            None => return Err("invalid first mantissa bit value".to_string()),
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
            None => Err("unable to get mantissa binaries".to_string()),
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
        let test: IEEE754 = IEEE754::new(values);
        println!("{:?}", test.to_64bit().unwrap());

        // 3.141592653589793
        let values = vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18];
        let test: IEEE754 = IEEE754::new(values);
        println!("{:?}", test.to_64bit().unwrap());

        // -3.125
        let values = vec![0xc0, 0x09, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values);
        println!("{:?}", test.to_64bit().unwrap());

        // -Infinity
        // Still need to catch the exception
        let values = vec![0x7f, 0xf0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let test: IEEE754 = IEEE754::new(values);
        println!("{:?}", test.to_64bit().unwrap());
    }
}
