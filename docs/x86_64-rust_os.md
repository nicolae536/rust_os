```json
{
  "llvm-target": "x86_64-unknown-none",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "arch": "x86_64",
  "target-endian": "little",
  "target-pointer-width": "64",
  "target-c-int-width": "32",
  // We don't target a specific os
  "os": "none",
  // Use [lld linker](https://lld.llvm.org/) because we don't provide a linker
  "linker-flavor": "ld.lld",
  "linker": "rust-lld",
  // On panic abort the process because we still don't support
  // [stack unwinding](https://www.bogotobogo.com/cplusplus/stackunwinding.php) 
  "panic-strategy": "abort",
  // 1. We're writing a kernel, so we'll need to handle interrupts at some point. 
  // To do that safely, we have to disable a certain stack pointer 
  // optimization called the “red zone”, because it would cause stack corruptions otherwise.
  // For more information, see our separate post about disabling the red zone.
  // 2. A problem with disabling SIMD is that floating point operations on x86_64 require SIMD registers by default. 
  // To solve this problem, we add the soft-float feature, which emulates all floating point operations through software functions based on normal integers.
  "disable-redzone": true,
  // mmx and sse features which determine support for [SIMD](https://en.wikipedia.org/wiki/SIMD) are disabled On kernel lead to prefomance problems
  "features": "-mmx,-sse,+soft-float",
  "executables": true
}
```