check:
  cargo deny check
  cargo check
  cargo check --features rx-profile-no-std
  cargo check --features rx-profile-std
  cargo check --features rx-profile-full

doc-crate:
  cargo doc -p rustx --features rx-profile-max
