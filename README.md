# fuser-android

**日本語**: [`fuser`](https://github.com/cberner/fuser) 0.18.0(MIT)を、Android(`target_os = "android"`)でもpure-rust実装でビルドできるようにしたフォークです。`open-raid-z`のFUSEマウント(`fuse_backend`)がAndroid向けに使います。変更は「`target_os = "linux"`のゲートを`linux`または`android`へ広げる」ことと、bionic libcで`RENAME_*`がi32型になる差を`as u32`で吸収することだけです(`build.rs`のpure-rust許可OS一覧への`android`追加を含む)。

- 検証: `cargo check --target aarch64-linux-android` 成功(0.18.0版)。**実機でのマウント動作は未検証**(root権限・SELinuxの制約があり得る)。
- 上流が同等の対応を取り込んだら、このリポジトリは不要になります。
- ライセンス: 上流と同じMIT(`LICENSE.md`)。著作権表示は上流のものです。

**English**: A fork of [`fuser`](https://github.com/cberner/fuser) 0.18.0 (MIT) that builds with the pure-Rust mount implementation on Android (`target_os = "android"`). It is used by `open-raid-z`'s FUSE backend. The only changes: `target_os = "linux"` gates are widened to `linux` or `android` (including the pure-rust OS list in `build.rs`), and bionic's `i32` `RENAME_*` constants are cast with `as u32`.

- Verified: `cargo check --target aarch64-linux-android` succeeds (0.18.0). **Real-device mounting is not verified** (root and SELinux constraints may apply).
- This repository becomes unnecessary once upstream takes equivalent support.
- License: MIT, same as upstream (`LICENSE.md`); copyright notices are upstream's.

Crate name: `fuser-android` (library name `fuser_android`). Use it as `extern crate fuser_android as fuser;`.
