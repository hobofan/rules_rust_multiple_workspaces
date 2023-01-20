workspace(
    name = "field33",
)

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "aaaa4b9591a5dad8d8907ae2dbe6e0eb49e6314946ce4c7149241648e56a1277",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.16.1/rules_rust-v0.16.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "workspace1_crate_index",
    cargo_lockfile = "//workspace1:Cargo.lock",
    lockfile = "//workspace1:Cargo.Bazel.lock",
    manifests = [
        "//workspace1:Cargo.toml",
        "//workspace1/foo:Cargo.toml",
    ],
)

load("@workspace1_crate_index//:defs.bzl", workspace1_crate_repositories = "crate_repositories")

workspace1_crate_repositories()

crates_repository(
    name = "workspace2_crate_index",
    cargo_lockfile = "//workspace2:Cargo.lock",
    lockfile = "//workspace2:Cargo.Bazel.lock",
    manifests = [
        "//workspace2:Cargo.toml",
        "//workspace2/bar:Cargo.toml",
    ],
)

load("@workspace2_crate_index//:defs.bzl", workspace2_crate_repositories = "crate_repositories")

workspace2_crate_repositories()
