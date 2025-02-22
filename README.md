# IEEE 754 Floating-Point Representation

The IEEE Standard for Floating-Point Arithmetic (IEEE 754) is a technical standard for floating-point computation which was established in 1985 by the Institute of Electrical and Electronics Engineers (IEEE).

To learn more about IEEE754 kindly visit: https://www.geeksforgeeks.org/ieee-standard-754-floating-point-numbers/

### Example 1:

Converting a hexadecimal to 64-bit floating-point value using IEEE754 double precision.

```rust
use crate::ieee754::IEEE754;

let values = vec![0xc0, 0x52, 0xaf, 0xbe, 0x4, 0x89, 0x76, 0x8e];
let test: IEEE754 = IEEE754::to_64bit_float(values.clone());
println!("Input: {:x?}", values);
println!("Expected Output: {}", -74.74597276138431);
assert_eq!(-74.74597276138431, test);
```

### Example 2:

Converting a hexadecimal to 32-bit floating-point value using IEEE754 single precision.

```rust
use crate::ieee754::IEEE754;

let values = vec![0x00, 0x00, 0x00, 0x00];
let test: f32= IEEE754::to_32bit_float(values.clone()).unwrap();
println!("Input: {:x?}", values);
println!("Expected Output: {}", 0.0);
assert_eq!(0.0, test);
```

### Example 3:

Converting a 32-bit floating point to hexadecimal value using IEEE754 single precision.

```rust
use crate::ieee754::IEEE754;

let values = -33.33333333;
let output = IEEE754::to_32bit_hex(values.clone());
println!("Input: {:x?}", values);
println!("Expected Output: C2055555");
assert_eq!(output.unwrap(), "C2055555");
```

### Example 4:

Converting a 64-bit floating point to hexadecimal value using IEEE754 double precision.

```rust
use crate::ieee754::IEEE754;

let values = -33.33333333;
let output = IEEE754::to_64bit_hex(values.clone());
println!("Input: {:x?}", values);
println!("Expected Output: C040AAAAAAA38226");
assert_eq!(output.unwrap(), "C040AAAAAAA38226");
```

References:
- https://www.wikihow.com/Convert-a-Number-from-Decimal-to-IEEE-754-Floating-Point-Representation
- https://www.ascii-code.com/
- https://gregstoll.com/~gregstoll/floattohex/
- https://numeral-systems.com/ieee-754-converter/
