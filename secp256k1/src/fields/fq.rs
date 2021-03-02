use ark_ff::{
    biginteger::BigInteger320 as BigInteger,
    fields::{FftParameters, Fp320, Fp320Parameters},
};

pub type Fq = Fp320<FqParameters>;

pub struct FqParameters;

impl Fp320Parameters for FqParameters {}
impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 1;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (5^T)R mod p.
    // 115792089237316195423570985008687907853269984665561335858920850710301058792495
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xfffffffefffffc2f,
        0xfffffffefffffc2e,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x0,
    ]);
}

impl ark_ff::fields::FpParameters for FqParameters {
    // 115792089237316195423570985008687907853269984665640564039457584007908834671663
    const MODULUS: BigInteger = BigInteger([
        0xfffffffefffffc2f,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x0,
    ]);

    // R = 2^320 mod p = 79228180536733297607775879168
    const R: BigInteger = BigInteger([
        0x0,
        0x1000003d1,
        0x0,
        0x0,
        0x0,
    ]);

    // R2 = (2^320)^2 mod p = 18446752466076602529
    const R2: BigInteger = BigInteger([
        0x0,
        0x0,
        0x7a2000e90a1,
        0x1,
        0x0,
    ]);

    // 57896044618658097711785492504343953926634992332820282019728792003954417335831
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xffffffff7ffffe17,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x7fffffffffffffff,
        0x0,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    // T = 57896044618658097711785492504343953926634992332820282019728792003954417335831
    const T: BigInteger = BigInteger([
        0xffffffff7ffffe17,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x7fffffffffffffff,
        0x0,
    ]);

    // (T - 1) / 2 = 28948022309329048855892746252171976963317496166410141009864396001977208667915
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xffffffffbfffff0b,
        0xffffffffffffffff,
        0xffffffffffffffff,
        0x3fffffffffffffff,
        0x0,
    ]);

    // GENERATOR = 5
    // Encoded in Montgomery form, so the value here is 5R mod p.
    const GENERATOR: BigInteger = BigInteger([
        0x0,
        0x500001315,
        0x0,
        0x0,
        0x0,
    ]);

    const MODULUS_BITS: u32 = 256;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 1;

    // INV = -p^{-1} (mod 2^64)
    const INV: u64 = 15580212934572586289;
}