use crate::estimate_size;

#[cfg(kani)]
#[kani::proof]
fn check_estimate_size() {
    let x: u32 = kani::any(); // 表示任何可能的输出
    estimate_size(x); // Call the function to be tested
}