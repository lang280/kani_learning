/*
  网页说明: https://model-checking.github.io/kani/tutorial-loop-unwinding.html
*/

// ANCHOR: code
fn initialize_prefix(length: usize, buffer: &mut [u8]) {
    // Let's just ignore invalid calls
    if length > buffer.len() {
        return;
    }

    for i in 0..=length {
        buffer[i] = 0;
    }
}
// ANCHOR_END: code

// ANCHOR: kani
#[cfg(kani)]
#[kani::proof]
/*
展开次数, 用于限制递归函数或循环的展开深度
如果过小, 会让测试无法完成, 造成结果UNDETERMINED
注: Kani needs the unwinding bound to be "one more than" the number of loop iterations

也可以通过命令行设置:
cargo kani --harness check_initialize_prefix --unwind 11
也可以在Cargo.toml中设置:
[workspace.metadata.kani.flags]
default-unwind = 1
 */
#[kani::unwind(11)]

fn check_initialize_prefix() {
    const LIMIT: usize = 10;
    let mut buffer: [u8; LIMIT] = [1; LIMIT]; // 一个长度为 LIMIT 的 u8 类型数组, 数组中的每个元素都被设置为 1

    let length = kani::any();
    kani::assume(length <= LIMIT);

    initialize_prefix(length, &mut buffer);
}
// ANCHOR_END: kani
