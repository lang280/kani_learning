/*
  原网址: https://model-checking.github.io/kani/tutorial-kinds-of-failure.html
*/
/// Wrap "too-large" indexes back into a valid range for the array
fn get_wrapped(i: usize, a: &[u32]) -> u32 {
    if a.len() == 0 {
        return 0;
    }
    return a[i % a.len() + 1];
}
// ANCHOR_END: code

/*
  单独运行这个验证的方式: cargo kani --harness bound_check
*/
#[cfg(kani)]
#[kani::proof]
fn bound_check() {
    let size: usize = kani::any();
    kani::assume(size < 4096);
    let index: usize = kani::any();
    let array: Vec<u32> = vec![0; size];
    get_wrapped(index, &array);
}

// // Alternative unsafe return for the above function:
// // return unsafe { *a.as_ptr().add(i % a.len() + 1) };

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use proptest::prelude::*;

//     // ANCHOR: proptest
//     proptest! {
//         #[test]
//         fn doesnt_crash(i: usize, a: Vec<u32>) {
//             get_wrapped(i, &a);
//         }
//     }
//     // ANCHOR_END: proptest
// }

// // ANCHOR: kani
// #[cfg(kani)]
// #[kani::proof]
// fn bound_check() {
//     let size: usize = kani::any();
//     kani::assume(size < 4096);
//     let index: usize = kani::any();
//     let array: Vec<u32> = vec![0; size];
//     get_wrapped(index, &array);
// }
// // ANCHOR_END: kani
