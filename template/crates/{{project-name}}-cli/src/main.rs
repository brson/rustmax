use rx::prelude::*;

fn main() -> AnyResult<()> {
    rx::extras::init();

    log!("hello world");

    Ok(())
}
