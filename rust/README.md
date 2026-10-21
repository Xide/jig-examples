# A Rust crate checked by Jig

A one-file Rust library with its tests, and what Jig needs to check it:

- a planner, in `.jig/planner/`, which declares three required checks, the
  standard ones of the `jig-sdk-cargo` crate: `fmt`, `clippy` and `test`;
- a Rust toolchain, in `.jig/environments/rust/`, taken from the server's
  package catalog. The planner builds and runs in it, and so do the checks.

## What you need

A Jig repository and an empty working copy of it, as in
[Getting started](https://jig.nerds.casa/getting-started/), with a runner,
as in [Install a runner](https://jig.nerds.casa/install-a-runner/). The commands
below run in that working copy, `~/src/demo` here. The examples are in
`~/jig-examples`, downloaded as [their README](../README.md) says.

## Check it

Copy the example into your working copy:

```sh
cp -r ~/jig-examples/rust/. ~/src/demo/
```

Each environment needs a lock, which pins exactly what it holds. The server
computes it, and you keep it in the repository:

```sh
jj check env lock
```

That writes `.jig/environments/rust/lock.toml`. Your next jj command saves
it, and Jig checks your work. The first time, it builds the toolchain and
the planner, which takes a few minutes. Follow it with:

```sh
jj check list
```

until `fmt`, `clippy` and `test` have passed.

## Land it

Record the change, and ask Jig to land it:

```sh
jj commit -m "A Rust crate checked by Jig"
jj request new
jj request ready
```

`jj request status` follows the request. Its checks are answered from what
your save already ran, so it soon joins the queue. There it is checked on
top of trunk, and lands. `jj log` then shows `trunk@jig` on the commit that
landed, with your working copy on top of it.
