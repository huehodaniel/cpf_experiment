# Experimento CPF

## Algoritmos

- CPF como String
    - [x] [String Baseline](./src/cpf_baseline.rs) - baseline - itera dígito a digito da string, convertendo para
      inteiro
    - [x] [String Unrolled](./src/cpf_unrolled.rs) - sem iteração - calcula todos os dígitos diretamente
    - [x] [String SIMD](./src/cpf_simd.rs) - SIMD - utiliza instruções SIMD para calcular os digitos
- CPF como inteiro (`u64`)
    - [x] [Integer Baseline](./src/int_cpf_baseline.rs) - algoritmo baseline - itera dígito a digito do inteiro,
      manipulando via módulo
    - [x] [Integer Unrolled](./src/int_cpf_unrolled.rs) - algoritmo sem iteração - calcula todos os dígitos diretamente
- CPF como slice de bytes (`&[u8]`), dispensando conversão de ASCII para inteiro
    - [x] [Byte Baseline](./src/cpf_byte_baseline.rs) - algoritmo baseline para array de bytes - itera dígito a digito
      do slice, convertendo para inteiro
    - [x] [Byte SIMD](./src/cpf_byte_simd.rs) - algoritmo SIMD - utiliza instruções SIMD para calcular os digitos

## Benchmarks

### Specs

```shell
➜  ~ system_profiler -detailLevel basic
[...]
Hardware:
    Hardware Overview:
      Model Name: MacBook Pro
      Model Identifier: Mac15,11
      Model Number: MRW33LL/A
      Chip: Apple M3 Max
      Total Number of Cores: 14 (10 performance and 4 efficiency)
      Memory: 36 GB
[...]
Software:
    System Software Overview:
      System Version: macOS 15.7.4 (24G517)
      Kernel Version: Darwin 24.6.0
```

### Resultados

|                                            | `String Baseline`        | `String Unrolled`               | `String SIMD`                   | `Integer Baseline`             | `Integer Unrolled`             | `Byte Baseline`                 | `Byte SIMD`                     |
|:-------------------------------------------|:-------------------------|:--------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:--------------------------------|:--------------------------------|
| **`Valid CPFs`**                           | `12.96 ns` (✅ **1.00x**) | `12.00 ns` (✅ **1.08x faster**) | `11.89 ns` (✅ **1.09x faster**) | `8.49 ns` (✅ **1.53x faster**) | `8.10 ns` (✅ **1.60x faster**) | `9.46 ns` (✅ **1.37x faster**)  | `9.29 ns` (✅ **1.39x faster**)  |
| **`Invalid CPFs (first checksum digit)`**  | `14.67 ns` (✅ **1.00x**) | `13.62 ns` (✅ **1.08x faster**) | `14.13 ns` (✅ **1.04x faster**) | `8.56 ns` (✅ **1.71x faster**) | `8.43 ns` (✅ **1.74x faster**) | `10.98 ns` (✅ **1.34x faster**) | `11.44 ns` (✅ **1.28x faster**) |
| **`Invalid CPFs (second checksum digit)`** | `12.46 ns` (✅ **1.00x**) | `12.46 ns` (✅ **1.00x slower**) | `12.11 ns` (✅ **1.03x faster**) | `8.78 ns` (✅ **1.42x faster**) | `8.43 ns` (✅ **1.48x faster**) | `9.17 ns` (✅ **1.36x faster**)  | `9.48 ns` (✅ **1.31x faster**)  |

---
(Benchmark via [criterion-rs](https://github.com/criterion-rs/criterion.rs), tabela feita
usando [criterion-table](https://github.com/nu11ptr/criterion-table))

