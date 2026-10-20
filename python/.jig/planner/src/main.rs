//! The example's planner: one required check, `test`, which runs the tests
//! with Python's own `unittest`.
use std::time::Duration;

#[jig::planner]
fn plan(p: &mut jig::Planner) -> jig::Result<()> {
    let python = p.environment("python")?;
    // What the tests read, and nothing else: a change to this planner or to
    // the README keeps the test's result.
    let selector = jig::TreeSelector::new()
        .file(p.repo_path("greeting.py")?)
        .tree(p.repo_path("tests")?);
    let source = p.select(jig::Id::new("source")?, selector)?;
    let command = jig::Command::new("python3")?
        .args(["-m", "unittest", "discover", "-s", "tests", "-v"])?
        .working_directory("/work/src")?;
    let mut test = p.job(jig::Id::new("test")?, &python, command);
    test.repository_input("source", "/work/src", source)?
        .budgets(jig::Budgets {
            cpus: 1,
            memory: jig::Bytes::mib(512),
            scratch: jig::Bytes::mib(64),
            output: jig::Bytes::mib(1),
            time: Duration::from_secs(120),
        });
    test.verdict("test")?;
    test.finish()
}
