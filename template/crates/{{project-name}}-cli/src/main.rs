use rx::prelude::*;

fn main() -> AnyResult<()> {
    rx::extras::init();

    info!("hello world");

    Ok(())
}
