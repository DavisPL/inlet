# Filesystem

This is the classic filesystem security example, which demonstrates how Inlet's origin tagging enforces capability safety. The functions in `std::fs` require a path that comes from the binary crate `program`. If a third-party crate tries calling one of these functions with its own path, an error will occur during checking.

Run this example with `cargo run -- -p examples/filesystem/program` from the Inlet project root.

