use ::HandleManager;

#[test]
fn simplest_use() {
    let mut x = HandleManager::new()
        .with_release_policy(::ReleasePolicy::DontTrack)
        .with_alloc_policy(::AllocPolicy::RecycleLowest);

    assert_eq!(x.next(), 0);
    assert_eq!(x.next(), 1);
    assert_eq!(x.next(), 2);

    x.release(1);

    assert_eq!(x.next(), 3);
}

#[test]
fn basic_recycling() {
    let mut x = HandleManager::new().with_alloc_policy(::AllocPolicy::RecycleLowest);

    assert_eq!(x.next(), 0);
    assert_eq!(x.next(), 1);
    assert_eq!(x.next(), 2);

    x.release(1);

    assert_eq!(x.next(), 1);
}
