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

Benchmark via [criterion-rs](https://github.com/criterion-rs/criterion.rs), tabela feita usando [criterion-table](https://github.com/nu11ptr/criterion-table)

### MacOS

#### Specs

<details>
<summary>Macbook Pro 2023 (M3 Max)</summary>

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

</details>

#### Resultados

|                                            | `String Baseline`          | `String Unrolled`               | `String SIMD`                   | `Integer Baseline`             | `Integer Unrolled`             | `Byte Baseline`                 | `Byte Unrolled`                 | `Byte SIMD`                     | `Float Baseline`                  | `Float Unrolled`                   |
|:-------------------------------------------|:---------------------------|:--------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:----------------------------------|:---------------------------------- |
| **`Valid CPFs`**                           | `11.68 ns` (✅ **1.00x**)   | `11.96 ns` (✅ **1.02x slower**) | `11.49 ns` (✅ **1.02x faster**) | `8.41 ns` (✅ **1.39x faster**) | `7.96 ns` (✅ **1.47x faster**) | `9.07 ns` (✅ **1.29x faster**)  | `9.17 ns` (✅ **1.27x faster**)  | `9.15 ns` (✅ **1.28x faster**)  | `352.97 ns` (❌ *30.21x slower*)   | `347.44 ns` (❌ *29.74x slower*)    |
| **`Invalid CPFs (first checksum digit)`**  | `13.58 ns` (✅ **1.00x**)   | `13.37 ns` (✅ **1.02x faster**) | `14.50 ns` (✅ **1.07x slower**) | `8.65 ns` (✅ **1.57x faster**) | `8.46 ns` (✅ **1.60x faster**) | `10.73 ns` (✅ **1.26x faster**) | `10.97 ns` (✅ **1.24x faster**) | `11.47 ns` (✅ **1.18x faster**) | `296.59 ns` (❌ *21.85x slower*)   | `292.63 ns` (❌ *21.56x slower*)    |
| **`Invalid CPFs (second checksum digit)`** | `11.86 ns` (✅ **1.00x**)   | `12.12 ns` (✅ **1.02x slower**) | `11.92 ns` (✅ **1.00x slower**) | `8.42 ns` (✅ **1.41x faster**) | `8.02 ns` (✅ **1.48x faster**) | `9.21 ns` (✅ **1.29x faster**)  | `9.04 ns` (✅ **1.31x faster**)  | `9.16 ns` (✅ **1.29x faster**)  | `357.01 ns` (❌ *30.09x slower*)   | `351.42 ns` (❌ *29.62x slower*)    |
| **`Mixed valid and invalid CPFs`**         | `13.77 ns` (✅ **1.00x**)   | `13.38 ns` (✅ **1.03x faster**) | `14.55 ns` (✅ **1.06x slower**) | `9.75 ns` (✅ **1.41x faster**) | `9.48 ns` (✅ **1.45x faster**) | `10.92 ns` (✅ **1.26x faster**) | `11.14 ns` (✅ **1.24x faster**) | `11.48 ns` (✅ **1.20x faster**) | `353.26 ns` (❌ *25.65x slower*)   | `348.10 ns` (❌ *25.28x slower*)    |

### Windows

#### Specs

<details>
<summary>AMD Ryzen 7 5800X3D (32GB)</summary>

```
CPU-Z TXT Report
-------------------------------------------------------------------------

Binaries
-------------------------------------------------------------------------

CPU-Z version			2.15.0.x64

[...]

Processors Information
-------------------------------------------------------------------------

Socket 1			ID = 0
	Number of cores		8 (max 8)
	Number of threads	16 (max 16)
	Secondary bus #		0
	Number of CCDs		1
	Manufacturer		AuthenticAMD
	Name			AMD Ryzen 7 5800X3D
	Codename		Vermeer
	Specification		AMD Ryzen 7 5800X3D 8-Core Processor           
	Package 		Socket AM4 (1331)
  [...]
	Core Speed		4448.3 MHz
	Multiplier x Bus Speed	44.5 x 100.0 MHz
	Base frequency (cores)	100.0 MHz
	Instructions sets	MMX (+), SSE, SSE2, SSE3, SSSE3, SSE4.1, SSE4.2, SSE4A, x86-64, AES, AVX, AVX2, FMA3, SHA
	Microcode Revision	0xA201210
	L1 Data cache		8 x 32 KB (8-way, 64-byte line)
	L1 Instruction cache	8 x 32 KB (8-way, 64-byte line)
	L2 cache		8 x 512 KB (8-way, 64-byte line)
	L3 cache		96 MB (16-way, 64-byte line)


[...]

Chipset
-------------------------------------------------------------------------

Northbridge			AMD Ryzen SOC rev. 00
Southbridge			AMD X570 rev. 51
Bus Specification		PCI-Express 4.0 (16.0 GT/s)
Graphic Interface		PCI-Express 5.0
PCI-E Link Width		x16 (max 16x)
PCI-E Link Speed		2.5 GT/s (max 32.0 GT/s)
Memory Type			DDR4
Memory Size			32 GBytes
Channels			2 x 64-bit
Memory Frequency		1799.3 MHz (3:54)

[...]

```

</details>

#### Resultados

|                                            | `String Baseline`          | `String Unrolled`               | `String SIMD`                   | `Integer Baseline`              | `Integer Unrolled`              | `Byte Baseline`                 | `Byte Unrolled`                 | `Byte SIMD`                     | `Float Baseline`                | `Float Unrolled`                 |
|:-------------------------------------------|:---------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:--------------------------------|:-------------------------------- |
| **`Valid CPFs`**                           | `24.05 ns` (✅ **1.00x**)   | `20.19 ns` (✅ **1.19x faster**) | `20.51 ns` (✅ **1.17x faster**) | `14.64 ns` (✅ **1.64x faster**) | `13.93 ns` (✅ **1.73x faster**) | `26.58 ns` (✅ **1.10x slower**) | `21.54 ns` (✅ **1.12x faster**) | `18.90 ns` (✅ **1.27x faster**) | `78.74 ns` (❌ *3.27x slower*)   | `67.98 ns` (❌ *2.83x slower*)    |
| **`Invalid CPFs (first checksum digit)`**  | `18.56 ns` (✅ **1.00x**)   | `20.26 ns` (✅ **1.09x slower**) | `19.01 ns` (✅ **1.02x slower**) | `11.91 ns` (✅ **1.56x faster**) | `12.10 ns` (✅ **1.53x faster**) | `20.85 ns` (❌ *1.12x slower*)   | `22.89 ns` (❌ *1.23x slower*)   | `15.87 ns` (✅ **1.17x faster**) | `60.67 ns` (❌ *3.27x slower*)   | `60.27 ns` (❌ *3.25x slower*)    |
| **`Invalid CPFs (second checksum digit)`** | `27.00 ns` (✅ **1.00x**)   | `23.36 ns` (✅ **1.16x faster**) | `17.61 ns` (✅ **1.53x faster**) | `13.09 ns` (🚀 **2.06x faster**) | `12.06 ns` (🚀 **2.24x faster**) | `26.79 ns` (✅ **1.01x faster**) | `23.90 ns` (✅ **1.13x faster**) | `14.88 ns` (🚀 **1.81x faster**) | `76.36 ns` (❌ *2.83x slower*)   | `67.65 ns` (❌ *2.51x slower*)    |
| **`Mixed valid and invalid CPFs`**         | `25.24 ns` (✅ **1.00x**)   | `22.51 ns` (✅ **1.12x faster**) | `19.61 ns` (✅ **1.29x faster**) | `14.38 ns` (✅ **1.76x faster**) | `14.35 ns` (✅ **1.76x faster**) | `30.73 ns` (❌ *1.22x slower*)   | `26.30 ns` (✅ **1.04x slower**) | `17.57 ns` (✅ **1.44x faster**) | `76.24 ns` (❌ *3.02x slower*)   | `68.42 ns` (❌ *2.71x slower*)    |



