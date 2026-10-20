//! The example's planner: the standard checks of its Cargo workspace,
//! `fmt`, `clippy` and `test`, all required.
use std::time::Duration;

#[jig::planner]
fn plan(p: &mut jig::Planner) -> jig::Result<()> {
    let rust = p.environment("rust")?;
    let workspace = jig_sdk_cargo::Workspace::read(p)?;
    let skeleton = workspace.skeleton(p, jig::Id::new("skeleton")?)?;
    // The download of the crates `Cargo.lock` pins, reused while it stays.
    let download = jig::Budgets {
        cpus: 1,
        memory: jig::Bytes::gib(1),
        scratch: jig::Bytes::gib(1),
        output: jig::Bytes::gib(1),
        time: Duration::from_secs(600),
    };
    let standard = jig_sdk_cargo::Standard {
        environment: &rust,
        source: workspace.select(p, jig::Id::new("source")?)?,
        vendor: workspace.vendor(p, jig::Id::new("vendor")?, &rust, skeleton, download)?,
        skeleton,
        // What each check may use.
        budgets: jig::Budgets {
            cpus: 2,
            memory: jig::Bytes::gib(2),
            scratch: jig::Bytes::gib(4),
            output: jig::Bytes::mib(64),
            time: Duration::from_secs(900),
        },
        // What building the third-party crates may use, once per lock.
        third_party: jig::Budgets {
            cpus: 2,
            memory: jig::Bytes::gib(2),
            scratch: jig::Bytes::gib(4),
            output: jig::Bytes::gib(2),
            time: Duration::from_secs(900),
        },
    };
    workspace.fmt(p, jig::Id::new("fmt")?, standard)?;
    workspace.clippy(p, jig::Id::new("clippy")?, standard)?;
    workspace.test(p, jig::Id::new("test")?, standard)
}
