Have you ever wanted to use nightly `rustfmt` features on stable rust? There are some *truly incredible* options you can put into `.rustfmt.toml` available, like...

- [`wrap_comments = true`](https://rust-lang.github.io/rustfmt/?version=v1.8.0&search#wrap_comments) to wrap all comments to be at most 80 characters
- [`format_code_in_doc_comments = true`](https://rust-lang.github.io/rustfmt/?version=v1.8.0&search#format_code_in_doc_comments) to format code in documentation comments
- Rust code, especially in larger repositories often suffers from **merge conflicts** because of imports re-ordering
  other imports and causing conflicts. I've personally experienced this at least a dozen times - when I had to rebase my PR
  just because of a conflict with the `main` branch - and the only conflict being imports being slightly different. This is unfortunate.

  To solve this, there is [`imports_granularity = "Item"`](https://rust-lang.github.io/rustfmt/?version=v1.8.0&search#Item%5C%3A) which causes
  each `use` to be import a single item. More verbose? Definitely. Less maintenance burden? Also true.

All 3 (and more!) of these fantastic options have been unstable for *years* - and it doesn't look like they will be stabilized any time soon.
You need to use a `nightly` Rust toolchain in order to make use of these features. But most codebases want to build with stable Rust, or a lower version.

This repo shows a trick you can use to allow usage of `nightly` rustfmt. You also get these benefits:

- rustc built-in lints, and clippy available on `nightly`.

  The [`mismatched_lifetime_syntaxes`](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint/lifetime_syntax/static.MISMATCHED_LIFETIME_SYNTAXES.html) lint is a good example.
  It was introduced in `1.89` but you can still enjoy it even if your project uses a lower MSRV.

- You can even use and enjoy any nightly `#![feature]` in tests!

- You can use the latest `nightly` version of `rust-analyzer` even for a project with an ancient MSRV like `1.24`,
  rust-analyzer frequently gains extreme quality-of-life updates to make Rust development more pleasant!

- Toolchain is automatically installed, and auto-formatting just works for free. Yes, you can use `cargo +nightly fmt` but this *won't* work with auto-save without having to configure
  every editor. This usually means cluttering your codebase with `.vscode`, `.helix`, `.zed` etc for every editor because we
  [don't have a single way to configure language servers](https://github.com/microsoft/language-server-protocol/issues/2127). And even then, after adding 10+ folders not all editors will be covered.

# The trick

1. In your [`Cargo.toml`](./Cargo.toml) set a `rust-version` to the stable Rust version that you'd like your project to compile with. 
1. In [`rust-toolchain.toml`](./rust-toolchain.toml) specify `toolchain` as `"nightly-yyyy-mm-dd"`

That's it!

If you try to use a feature from 1.85 but your `rust-version` is set to 1.74 for example, the
[`incompatible_msrv`](https://rust-lang.github.io/rust-clippy/master/index.html#/incompatible_msrv) lint will prevent you from doing so. 
