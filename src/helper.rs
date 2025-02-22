#[derive(Debug)]
pub struct ComputeMantissaBits;

impl ComputeMantissaBits {
    pub fn round_up(bits: &mut Vec<u8>) -> Result<Vec<u8>, String> {
        let mut is_overflow: bool = true;
        let mut i = bits.len() - 1;

        loop {
            if let Some(b) = bits.get_mut(i) {
                if *b == 1 {
                    *b = 0;
                } else {
                    *b = 1;
                    is_overflow = false;
                    break;
                }
            } else {
                return Err("unable to round up, no bit to compare".to_string());
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        if is_overflow {
            return Err("unable to round up, overflow".to_string());
        }
        Ok(bits.to_owned())
    }

    pub fn compute(input: Vec<u8>, length: usize) -> Result<Vec<u8>, String> {
        let mut bits = input.get(0..length).unwrap().to_vec();
        let guard_bit: u8 = *input.get(length).unwrap_or(&0);
        let round_bit: u8 = *input.get(length + 1).unwrap_or(&0);
        let sticky_bits: Vec<u8> = input.get(length + 2..).unwrap_or(&Vec::new()).to_vec();
        let mut is_round_up = false;
        if guard_bit == 1 {
            if round_bit == 1 {
                is_round_up = true;
            } else if round_bit == 0 {
                if sticky_bits
                    .clone()
                    .into_iter()
                    .filter(|v| *v == 1)
                    .collect::<Vec<u8>>()
                    .len()
                    > 0
                {
                    is_round_up = true;
                } else {
                    if let Some(last_bit) = bits.last() {
                        if *last_bit == 1 {
                            is_round_up = true;
                        }
                    }
                }
            }
        }
        if is_round_up {
            bits = Self::round_up(&mut bits)?;
        }
        Ok(bits)
    }
}

#[derive(Debug, Clone)]
pub struct SplitFloat;
impl SplitFloat {
    pub fn f32(input: f32) -> Result<(u8, u32, f32), String> {
        let sign: u8 = if input.is_sign_negative() { 1 } else { 0 };
        let value: String = input.to_string().replace("-", "");
        let vsplitted: Vec<&str> = if !value.contains(".") {
            vec![&"0", &"0"]
        } else {
            value.split(".").collect()
        };
        let integer_part: u32 = if let Some(v) = vsplitted.get(0) {
            match v.parse::<u32>() {
                Ok(v) => v,
                Err(error) => return Err(error.to_string()),
            }
        } else {
            return Err("invalid floating integer part value".to_string());
        };
        let fractional_part: f32 = if let Some(v) = vsplitted.get(1) {
            match format!("0.{}", v).parse::<f32>() {
                Ok(v) => v,
                Err(error) => return Err(error.to_string()),
            }
        } else {
            return Err("invalid floating fractional part value".to_string());
        };
        Ok((sign, integer_part, fractional_part))
    }
    pub fn f64(input: f64) -> Result<(u8, u64, f64), String> {
        let sign: u8 = if input.is_sign_negative() { 1 } else { 0 };
        let value: String = input.to_string().replace("-", "");
        let vsplitted: Vec<&str> = if !value.contains(".") {
            vec![&"0", &"0"]
        } else {
            value.split(".").collect()
        };
        let integer_part: u64 = if let Some(v) = vsplitted.get(0) {
            match v.parse::<u64>() {
                Ok(v) => v,
                Err(error) => return Err(error.to_string()),
            }
        } else {
            return Err("invalid floating integer part value".to_string());
        };
        let fractional_part: f64 = if let Some(v) = vsplitted.get(1) {
            match format!("0.{}", v).parse::<f64>() {
                Ok(v) => v,
                Err(error) => return Err(error.to_string()),
            }
        } else {
            return Err("invalid floating fractional part value".to_string());
        };
        Ok((sign, integer_part, fractional_part))
    }
}
