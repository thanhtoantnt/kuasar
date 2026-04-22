use proptest::prelude::*;

const KNOWN_MOUNT_FLAGS: &[&str] = &[
    "async",
    "atime",
    "bind",
    "defaults",
    "dev",
    "diratime",
    "dirsync",
    "exec",
    "mand",
    "noatime",
    "nodev",
    "nodiratime",
    "noexec",
    "nomand",
    "norelatime",
    "nostrictatime",
    "nosuid",
    "rbind",
    "relatime",
    "remount",
    "ro",
    "rw",
    "strictatime",
    "suid",
    "sync",
];

pub fn known_mount_flag() -> impl Strategy<Value = String> {
    prop::sample::select(KNOWN_MOUNT_FLAGS).prop_map(|s| s.to_string())
}

pub fn unknown_mount_option() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{0,10}".prop_map(|s| s)
}

pub fn mount_option() -> impl Strategy<Value = String> {
    prop_oneof![
        3 => known_mount_flag(),
        1 => unknown_mount_option(),
    ]
}

pub fn mount_options() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(mount_option(), 0..10)
}
