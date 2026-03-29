# Experimento CPF

## Algoritmos

- CPF como String
    - [x] [String Baseline](src/cpf/baseline.rs) - baseline - itera dígito a digito da string, convertendo para
      inteiro
    - [x] [String Unrolled](src/cpf/unrolled.rs) - sem iteração - calcula todos os dígitos diretamente
    - [x] [String SIMD](src/cpf/simd.rs) - SIMD - utiliza instruções SIMD para calcular os digitos
- CPF como inteiro (`u64`)
    - [x] [Integer Baseline](src/int_cpf/baseline.rs) - algoritmo baseline - itera dígito a digito do inteiro,
      manipulando via módulo
    - [x] [Integer Unrolled](src/int_cpf/unrolled.rs) - algoritmo sem iteração - calcula todos os dígitos diretamente
- CPF como slice de bytes (`&[u8]`), dispensando conversão de ASCII para inteiro
    - [x] [Byte Baseline](src/byte_cpf/baseline.rs) - algoritmo baseline para array de bytes - itera dígito a digito
      do slice, convertendo para inteiro
    - [x] [String Unrolled](src/byte_cpf/unrolled.rs) - sem iteração - calcula todos os dígitos diretamente
    - [x] [Byte SIMD](src/byte_cpf/simd.rs) - algoritmo SIMD - utiliza instruções SIMD para calcular os digitos
- CPF como float (`f64`)
    - [x] [Float Baseline](src/float_cpf/baseline.rs) - algoritmo baseline - itera dígito a digito do float,
      manipulando via módulo
    - [x] [Float Unrolled](src/float_cpf/unrolled.rs) - algoritmo sem iteração - calcula todos os dígitos diretamente


## Benchmarks

### MacOS

#### Specs

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

#### Resultados

|                                            | `String Baseline`          | `String Unrolled`               | `String SIMD`                   | `Integer Baseline`             | `Integer Unrolled`             | `Byte Baseline`                 | `Byte Unrolled`                 | `Byte SIMD`                     | `Float Baseline`                  | `Float Unrolled`                   |
|:-------------------------------------------|:---------------------------|:--------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:----------------------------------|:---------------------------------- |
| **`Valid CPFs`**                           | `11.68 ns` (✅ **1.00x**)   | `11.96 ns` (✅ **1.02x slower**) | `11.49 ns` (✅ **1.02x faster**) | `8.41 ns` (✅ **1.39x faster**) | `7.96 ns` (✅ **1.47x faster**) | `9.07 ns` (✅ **1.29x faster**)  | `9.17 ns` (✅ **1.27x faster**)  | `9.15 ns` (✅ **1.28x faster**)  | `352.97 ns` (❌ *30.21x slower*)   | `347.44 ns` (❌ *29.74x slower*)    |
| **`Invalid CPFs (first checksum digit)`**  | `13.58 ns` (✅ **1.00x**)   | `13.37 ns` (✅ **1.02x faster**) | `14.50 ns` (✅ **1.07x slower**) | `8.65 ns` (✅ **1.57x faster**) | `8.46 ns` (✅ **1.60x faster**) | `10.73 ns` (✅ **1.26x faster**) | `10.97 ns` (✅ **1.24x faster**) | `11.47 ns` (✅ **1.18x faster**) | `296.59 ns` (❌ *21.85x slower*)   | `292.63 ns` (❌ *21.56x slower*)    |
| **`Invalid CPFs (second checksum digit)`** | `11.86 ns` (✅ **1.00x**)   | `12.12 ns` (✅ **1.02x slower**) | `11.92 ns` (✅ **1.00x slower**) | `8.42 ns` (✅ **1.41x faster**) | `8.02 ns` (✅ **1.48x faster**) | `9.21 ns` (✅ **1.29x faster**)  | `9.04 ns` (✅ **1.31x faster**)  | `9.16 ns` (✅ **1.29x faster**)  | `357.01 ns` (❌ *30.09x slower*)   | `351.42 ns` (❌ *29.62x slower*)    |
| **`Mixed valid and invalid CPFs`**         | `13.77 ns` (✅ **1.00x**)   | `13.38 ns` (✅ **1.03x faster**) | `14.55 ns` (✅ **1.06x slower**) | `9.75 ns` (✅ **1.41x faster**) | `9.48 ns` (✅ **1.45x faster**) | `10.92 ns` (✅ **1.26x faster**) | `11.14 ns` (✅ **1.24x faster**) | `11.48 ns` (✅ **1.20x faster**) | `353.26 ns` (❌ *25.65x slower*)   | `348.10 ns` (❌ *25.28x slower*)    |

---
(Benchmark via [criterion-rs](https://github.com/criterion-rs/criterion.rs), tabela feita
usando [criterion-table](https://github.com/nu11ptr/criterion-table))

