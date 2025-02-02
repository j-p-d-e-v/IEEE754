# IEEE 754 Floating-Point Representation

This document provides an overview of the IEEE 754 standard for single-precision (32-bit) and double-precision (64-bit) floating-point representations.

---

## Table of Contents

1. [Introduction](#introduction)
2. [Single-Precision (32-bit) Format](#single-precision-32-bit-format)
3. [Double-Precision (64-bit) Format](#double-precision-64-bit-format)
4. [Formula for Floating-Point Representation](#formula-for-floating-point-representation)
5. [Special Values](#special-values)
6. [References](#references)

---

## Introduction

The IEEE 754 standard defines how floating-point numbers are represented in binary format. It is widely used in computer systems for its ability to represent a wide range of values with a consistent level of precision. The standard supports two primary formats:

- **Single-Precision (32-bit)**
- **Double-Precision (64-bit)**

---

## Single-Precision (32-bit) Format

The single-precision format uses **32 bits** to represent a floating-point number. The bits are divided into three parts:

- **Sign Bit (1 bit)**: Determines the sign of the number (`0` for positive, `1` for negative).
- **Exponent (8 bits)**: Represents the exponent in biased form. The bias is **127**.
- **Fraction/Mantissa (23 bits)**: Represents the fractional part of the number.

### Bit Layout:

```
| Sign (1 bit) | Exponent (8 bits) | Fraction/Mantissa (23 bits) |
```

### Range:

- **Minimum Normalized Value**: \(2^{-126}\) ≈ \(1.18 \times 10^{-38}\)
- **Maximum Normalized Value**: \(2^{127} \times (2 - 2^{-23})\) ≈ \(3.4 \times 10^{38}\)

### Precision:

- Approximately **7 decimal digits** of precision.

---

## Double-Precision (64-bit) Format

The double-precision format uses **64 bits** to represent a floating-point number. The bits are divided into three parts:

- **Sign Bit (1 bit)**: Determines the sign of the number (`0` for positive, `1` for negative).
- **Exponent (11 bits)**: Represents the exponent in biased form. The bias is **1023**.
- **Fraction/Mantissa (52 bits)**: Represents the fractional part of the number.

### Bit Layout:

```
| Sign (1 bit) | Exponent (11 bits) | Fraction/Mantissa (52 bits) |
```

### Range:

- **Minimum Normalized Value**: \(2^{-1022}\) ≈ \(2.23 \times 10^{-308}\)
- **Maximum Normalized Value**: \(2^{1023} \times (2 - 2^{-52})\) ≈ \(1.8 \times 10^{308}\)

### Precision:

- Approximately **15 decimal digits** of precision.

---

## Formula for Floating-Point Representation

Both single-precision and double-precision formats use the same formula to calculate the value of a floating-point number:

\[
\text{Value} = (-1)^{\text{sign}} \times 2^{\text{exponent} - \text{bias}} \times \left(1 + \text{fraction}\right)
\]

Where:

- **sign**: The sign bit (`0` or `1`).
- **exponent**: The biased exponent value.
- **bias**: `127` for single-precision, `1023` for double-precision.
- **fraction**: The fractional part of the mantissa.

---

## Special Values

IEEE 754 defines special values for specific bit patterns:

- **Zero**: Exponent and mantissa are all zeros.
- **Infinity**: Exponent is all ones, and mantissa is all zeros.
- **NaN (Not a Number)**: Exponent is all ones, and mantissa is non-zero.
- **Denormalized Numbers**: Exponent is all zeros, and mantissa is non-zero (used for very small numbers close to zero).

---

## References

- [IEEE 754 Standard](https://ieeexplore.ieee.org/document/8766229)
- [Wikipedia: IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
