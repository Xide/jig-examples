# Jig examples

Small repositories that show how a project is checked by
[Jig](https://jig.nerds.casa/). Each README takes you from copying the
example into an empty working copy to its first change landed on trunk.
Jig's own tests run each one as its README says, so they stay true.

- [`rust/`](rust/): a one-file Rust crate, checked with the standard
  `fmt`, `clippy` and `test` checks.
- [`python/`](python/): a Python project, checked in a Python environment.
  It also shows a failing check holding a request back until it is fixed.

## Get them

Download them into `~/jig-examples`:

```sh
mkdir -p ~/jig-examples
curl -fsSL https://github.com/Xide/jig-examples/archive/refs/heads/main.tar.gz |
  tar -xz --strip-components=1 -C ~/jig-examples
```

They go with Jig's latest release. Each release's own are under a tag of
its version, such as `0.1.4`.

The examples are licensed under the Apache License 2.0, in `LICENSE`.
