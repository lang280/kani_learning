use crate::estimate_size;

#[cfg(kani)]
#[kani::proof]
fn check_estimate_size() {
    let x: u32 = kani::any(); // Generate any u32 value
    estimate_size(x); // Call the function to be tested
}