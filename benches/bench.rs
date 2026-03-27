use crate::RandomCPFType::*;
use cpf_experiment::byte_cpf::ByteCPFValidator;
use cpf_experiment::byte_cpf_baseline::BaselineByteCPFValidator;
use cpf_experiment::byte_cpf_simd::SimdByteCPFValidator;
use cpf_experiment::cpf::CPFValidator;
use cpf_experiment::cpf_baseline::BaselineCPFValidator;
use cpf_experiment::cpf_simd::SimdCPFValidator;
use cpf_experiment::cpf_unrolled::UnrolledCPFValidator;
use cpf_experiment::int_cpf::IntCPFValidator;
use cpf_experiment::int_cpf_baseline::IntCPFNaiveValidator;
use cpf_experiment::int_cpf_unrolled::UnrolledIntCPFValidator;
use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, Criterion};

#[inline]
fn run_validation(validator: &mut impl CPFValidator, cpf: &str) -> Result<(), &'static str> {
    validator.validate(cpf)
}

#[inline]
fn run_int_validation(validator: &mut impl IntCPFValidator, cpf: u64) -> Result<(), &'static str> {
    validator.validate(cpf)
}

#[inline]
fn run_byte_validation(validator: &mut impl ByteCPFValidator, cpf: &[u8]) -> Result<(), &'static str> {
    validator.validate(cpf)
}


#[derive(Clone, Copy)]
enum RandomCPFType {
    Valid,
    InvalidFirst,
    InvalidSecond,
}

#[allow(dead_code)]
mod random_cpf {
    use crate::RandomCPFType;
    use crate::RandomCPFType::{InvalidFirst, InvalidSecond, Valid};
    use cpf_experiment::generator::CPFGenerator;

    fn next_valid() -> String {
        let num = rand::random_range::<u32, _>(0..=999_999_999);
        CPFGenerator::get_with_checksum_from_int(num)
    }

    fn next_invalid() -> String {
        let next = next_valid();
        let mut checksum: i32 = next[9..].parse().unwrap();
        if checksum == 0 {
            checksum = 67;
        } else {
            checksum = 0;
        }

        format!("{}{:02}", &next[0..9], checksum)
    }

    fn next_invalid_second() -> String {
        let next = next_valid();
        let mut checksum: i32 = next[9..].parse().unwrap();
        if checksum % 10 == 0 {
            checksum += 1;
        } else {
            checksum -= 1;
        }

        format!("{}{:02}", &next[0..9], checksum)
    }

    pub(crate) fn next(t: RandomCPFType) -> String {
        match t {
            Valid => next_valid(),
            InvalidFirst => next_invalid(),
            InvalidSecond => next_invalid_second(),
        }
    }

    pub(crate) fn next_int(t: RandomCPFType) -> u64 {
        cpf_experiment::convert::cpf_string_to_int(&next(t)).expect("Invalid CPF")
    }

    pub(crate) fn next_bytes(t: RandomCPFType) -> Vec<u8> {
        cpf_experiment::convert::cpf_string_to_byte(&next(t))
    }
}

fn validator_bench(c: &mut Criterion) {
    let mut baseline = BaselineCPFValidator::new();
    let mut unrolled = UnrolledCPFValidator::new();
    let mut simd = SimdCPFValidator::new();

    let mut int_baseline = IntCPFNaiveValidator::new();
    let mut int_unrolled = UnrolledIntCPFValidator::new();

    let mut byte_baseline = BaselineByteCPFValidator::new();
    let mut byte_simd = SimdByteCPFValidator::new();

    let param_valid = (Valid, "Valid CPFs".to_string());
    let param_invalid = (
        InvalidFirst,
        "Invalid CPFs (first checksum digit)".to_string(),
    );
    let param_invalid_second = (
        InvalidSecond,
        "Invalid CPFs (second checksum digit)".to_string(),
    );
    let params = vec![param_valid, param_invalid, param_invalid_second];

    let mut group = c.benchmark_group("cpf_validation");
    for (param, desc) in params {
        group.bench_with_input(
            criterion::BenchmarkId::new("String Baseline", &desc),
            &param,
            |b, val| {
                b.iter_batched(
                    || random_cpf::next(*val),
                    |n| run_validation(&mut baseline, &n),
                    SmallInput,
                )
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("String Unrolled", &desc),
            &param,
            |b, val| {
                b.iter_batched(
                    || random_cpf::next(*val),
                    |n| run_validation(&mut unrolled, &n),
                    SmallInput,
                )
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("String SIMD", &desc),
            &param,
            |b, val| {
                b.iter_batched(
                    || random_cpf::next(*val),
                    |n| run_validation(&mut simd, &n),
                    SmallInput,
                )
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("Integer Baseline", &desc),
            &param,
            |b, val| {
                b.iter_batched(
                    || random_cpf::next_int(*val),
                    |n| run_int_validation(&mut int_baseline, n),
                    SmallInput,
                )
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("Integer Unrolled", &desc),
            &param,
            |b, val| {
                b.iter_batched(
                    || random_cpf::next_int(*val),
                    |n| run_int_validation(&mut int_unrolled, n),
                    SmallInput,
                )
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("Byte Baseline", &desc),
            &param,
            |b, val| {
                b.iter_batched(
                    || random_cpf::next_bytes(*val),
                    |n| run_byte_validation(&mut byte_baseline, &n),
                    SmallInput,
                )
            },
        );
        group.bench_with_input(
            criterion::BenchmarkId::new("Byte SIMD", &desc),
            &param,
            |b, val| {
                b.iter_batched(
                    || random_cpf::next_bytes(*val),
                    |n| run_byte_validation(&mut byte_simd, &n),
                    SmallInput,
                )
            },
        );
    }

    group.finish();
}

criterion_group!(benches, validator_bench);
criterion_main!(benches);
