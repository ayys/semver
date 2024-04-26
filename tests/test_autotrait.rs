#![allow(clippy::extra_unused_type_parameters)]

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn test() {
    assert_send_sync::<semver_eq::BuildMetadata>();
    assert_send_sync::<semver_eq::Comparator>();
    assert_send_sync::<semver_eq::Error>();
    assert_send_sync::<semver_eq::Prerelease>();
    assert_send_sync::<semver_eq::Version>();
    assert_send_sync::<semver_eq::VersionReq>();
    assert_send_sync::<semver_eq::Op>();
}
