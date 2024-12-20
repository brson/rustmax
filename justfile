check:
  #cargo deny check
  cargo check
  cargo check --features rmx-profile-no-std
  cargo check --features rmx-profile-std
  cargo check --features rmx-profile-full
  cargo check --features rmx-profile-max

doc-crate:
  cargo doc -p rmx --features rmx-profile-max
