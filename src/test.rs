use ::HandleManager;

#[test]
fn basic_use() {
    let mut x = HandleManager::new().with_alloc_policy(::AllocPolicy::RecycleLowest);

    assert_eq!(x.next(), 0);
    assert_eq!(x.next(), 1);
    assert_eq!(x.next(), 2);

    x.release(1);

    assert_eq!(x.next(), 1);
}
