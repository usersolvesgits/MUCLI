use anyhow::Error;

pub trait CommandsActions {
    fn run(&self) -> Result<(), Error>;
}