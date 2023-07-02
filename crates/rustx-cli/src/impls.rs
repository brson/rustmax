use super::*;

impl Tool {
    pub fn attrs(&self) -> ToolAttrs {
        use Tool::*;
        match self {
            Rustup => ToolAttrs {
                display_name: "rustup",
            },

            Cargo => ToolAttrs {
                display_name: "cargo",
            },
            CargoClippy => ToolAttrs {
                display_name: "cargo-clippy",
            },
            CargoFmt => ToolAttrs {
                display_name: "cargo-fmt",
            },
            CargoMiri => ToolAttrs {
                display_name: "cargo-miri",
            },
            Rustc => ToolAttrs {
                display_name: "rustc",
            },
            Rustdoc => ToolAttrs {
                display_name: "rustdoc",
            },
            Rustfmt => ToolAttrs {
                display_name: "rustfmt",
            },
            RustGdbGui => ToolAttrs {
                display_name: "rust-gdbgui",
            },
            RustGdb => ToolAttrs {
                display_name: "rust-gdb",
            },
            RustLldb => ToolAttrs {
                display_name: "rust-lldb",
            },

            RustAnalyzer => ToolAttrs {
                display_name: "rust-analyzer",
            },
            Miri => ToolAttrs {
                display_name: "miri",
            },
            Clippy => ToolAttrs {
                display_name: "clippy",
            },
            LlvmTools => ToolAttrs {
                display_name: "llvm-tools",
            },
            LlvmCov => ToolAttrs {
                display_name: "llvm-cov",
            },

            CargoAudit => ToolAttrs {
                display_name: "cargo-audit",
            },
            CargoBenchcmp => ToolAttrs {
                display_name: "cargo-benchcmp",
            },
            CargoCleanAll => ToolAttrs {
                display_name: "cargo-clean-all",
            },
            CargoDeny => ToolAttrs {
                display_name: "cargo-deny",
            },
            CargoDeps => ToolAttrs {
                display_name: "cargo-deps",
            },
            CargoEdit => ToolAttrs {
                display_name: "cargo-edit",
            },
            CargoExpand => ToolAttrs {
                display_name: "cargo-expand",
            },
            CargoFuzz => ToolAttrs {
                display_name: "cargo-fuzz",
            },
            CargoGeiger => ToolAttrs {
                display_name: "cargo-geiger",
            },
            CargoGenerate => ToolAttrs {
                display_name: "cargo-generate",
            },
            CargoHack => ToolAttrs {
                display_name: "cargo-hack",
            },
            CargoLlvmLines => ToolAttrs {
                display_name: "cargo-llvm-lines",
            },
            CargoOutdated => ToolAttrs {
                display_name: "cargo-outdated",
            },
            CargoUdeps => ToolAttrs {
                display_name: "cargo-udeps",
            },
            CargoTree => ToolAttrs {
                display_name: "cargo-tree",
            },
            CargoWatch => ToolAttrs {
                display_name: "cargo-watch",
            },
            CargoWorkspace => ToolAttrs {
                display_name: "cargo-workspace",
            },
            CargoSemver => ToolAttrs {
                display_name: "cargo-semver",
            },

            _ => ToolAttrs {
                display_name: "<unknown>",
            }
        }
    }
}
