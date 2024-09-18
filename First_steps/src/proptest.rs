proptest! {
    #![proptest_config(ProptestConfig::with_cases(10000))]
    #[test]
    fn doesnt_crash(x: u32) {
        estimate_size(x);
    }
}