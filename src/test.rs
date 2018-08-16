use ::HandleManager;

#[test]
fn simplest_use() {
    let mut x = HandleManager::new()
        .with_release_policy(::ReleasePolicy::DontTrack)
        .with_alloc_policy(::AllocPolicy::RecycleLowest);

    assert_eq!(x.next(), 0);
    assert_eq!(x.next(), 1);
    assert_eq!(x.next(), 2);

    assert!(x.release(1).is_ok());

    assert_eq!(x.next(), 3);
}

#[test]
fn basic_recycling() {
    let mut x = HandleManager::new()
        .with_release_policy(::ReleasePolicy::Tracked)
        .with_alloc_policy(::AllocPolicy::RecycleLowest);

    assert_eq!(x.next(), 0);
    assert_eq!(x.next(), 1);
    assert_eq!(x.next(), 2);
    assert_eq!(x.next(), 3);

    assert!(x.is_used(3));
    assert!(x.release(3).is_ok());
    assert!(!x.is_used(3));
    assert!(x.release(1).is_ok());

    assert_eq!(x.next(), 1);
    assert_eq!(x.next(), 3);
}

#[test]
fn double_free() {
    let mut x = HandleManager::new()
        .with_release_policy(::ReleasePolicy::Tracked)
        .with_alloc_policy(::AllocPolicy::RecycleLowest);

    assert_eq!(x.next(), 0);
    assert!(x.release(0).is_ok());
    assert!(x.release(0).is_err());
}
