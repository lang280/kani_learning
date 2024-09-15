fn contract_for_nonzero_new(n: T) -> Option<NonZero<T>> {
    // 前置条件：n可以是任何值，不需要检查
    // 后置条件：
    if n != 0 {
        // 如果n非零，应该返回Some(NonZero)，且NonZero的值等于n
        assert!(NonZero::new(n).is_some());
        assert_eq!(NonZero::new(n).unwrap().get(), n);
    } else {
        // 如果n为零，应该返回None
        assert!(NonZero::new(n).is_none());
    }
}

unsafe fn contract_for_nonzero_new_unchecked(n: T) -> NonZero<T> {
    // 前置条件：调用者必须保证n不为零
    assert!(n != 0);

    // 后置条件：创建的NonZero对象值等于n
    let nonzero_value = NonZero::new_unchecked(n);
    assert_eq!(nonzero_value.get(), n);

    return nonzero_value;
}
