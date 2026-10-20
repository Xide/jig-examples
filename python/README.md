# A Python project checked by Jig

A small Python module and its tests, with what Jig needs to check it:

- a planner, in `.jig/planner/`, which declares one required check, `test`,
  that runs the tests with Python's own `unittest`;
- two environments, taken from the server's package catalog: `python`,
  where the check runs, and `rust`, where the planner builds and runs. A
  planner is a small Rust program, whatever language the project is in.

The check reads only `greeting.py` and `tests/`. A change elsewhere, such
as to this file, keeps its result.

## What you need

A Jig repository and an empty working copy of it, as in
[Getting started](https://jig.nerds.casa/getting-started/), with a runner,
as in [Run a runner](https://jig.nerds.casa/run-a-runner/). The commands
below run in that working copy, `~/src/demo` here. The examples are in
`~/jig-examples`, downloaded as [their README](../README.md) says.

## Check it

Copy the example into your working copy, and lock its environments:

```sh
cp -r ~/jig-examples/python/. ~/src/demo/
jj check env lock
```

That writes a `lock.toml` in each environment, which pins exactly what it
holds. Your next jj command saves them, and Jig checks your work. The first
time, it builds both environments and the planner, which takes a few
minutes. Follow it with `jj check list`, until `test` has passed, and record
the change:

```sh
jj commit -m "A Python project checked by Jig"
```

## See a failing check hold a request back

Make the greeting say something else, without changing its test:

```sh
sed -i 's/Hello, {name}!/Hi, {name}!/' greeting.py
jj commit -m "Greet more casually"
jj request new
jj request ready
```

`test` fails, and `jj check log test` shows why. The request is ready, but
`jj request status` says its checks failed, and it stays out of the queue.

## Fix it, and land it

Fix the test, and fold the fix into the change the request holds:

```sh
sed -i 's/"Hello, Ada!"/"Hi, Ada!"/' tests/test_greeting.py
jj squash
jj request target set -r @-
```

`jj squash` moves your fix into the change below, which keeps its identity.
`jj request target set` makes the request's next proposal from it. Its
checks pass, and since the request is ready, it joins the queue by itself
and lands on `trunk@jig`, with the first change under it.
