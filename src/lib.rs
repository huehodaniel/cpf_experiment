#![feature(portable_simd)]
pub mod cpf;
pub mod generator;
pub mod convert;
pub mod cpf_baseline;
pub mod cpf_simd;
pub mod cpf_unrolled;
pub mod int_cpf;
pub mod int_cpf_baseline;
pub mod int_cpf_unrolled;
pub mod byte_cpf;
pub mod byte_cpf_baseline;
pub mod byte_cpf_simd;
