
### property test
传统测试方法, 例子:
```
proptest! {
    #[test]
    fn doesnt_crash(i: usize, a: Vec<u32>) {
        get_wrapped(i, &a);
    }
}
```

test harness
自动化测试的软件框架或环境。它包括一组测试工具和测试脚本，用于执行测试用例、记录测试结果、并分析输出

Formal verification tool
Example: Kani, SPIN, Coq, Z3
通过数学方法和逻辑推理来证明系统是否满足特定的属性或规范。与传统的测试方法不同，形式验证工具不依赖于具体的测试用例，而是通过系统化的分析来确保系统在所有可能的输入情况下都能正确运行


`cargo test` 传统的单元测试，基于具体的输入输出对。
`cargo kani` 用于形式验证，通过系统化的分析确保代码在所有可能的输入情况下都能正确运行

## 让库能正常运行
1. git clone https://github.com/RustyRelic/verify-rust-std.git
2. git submodule update --init --recursive
3. cargo check   // Should have 25 warnings
4. kani verify-std -Z unstable-options "../library" -Z function-contracts -Z mem-predicates --harness new_unchecked

## 运行kani
`kani verify-std -Z unstable-options "库所在目录" -Z function-contracts -Z mem-predicates --harness harness函数名称`
例子: `kani verify-std -Z unstable-options "." -Z function-contracts -Z mem-predicates --harness from_u8`

# 文档中的错误:
https://model-checking.github.io/kani/reference/attributes.html#kaniproof
Check the Stubbing section for more information about stubbing.

https://model-checking.github.io/kani/verification-results.html
FAILURE: This indicates that the check failed (i.e., the property doesn't hold). In this case, please see the concrete playback section for more help.