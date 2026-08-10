mod args;
mod backend;
mod util;

use ovation::err::OvationError;

use args::Args;
use util::{Result, TryResolve};

fn main() -> Result<()> {
    Args::execute().try_resolve(|e| match e {
        OvationError::ClapTerminal(t) => {
            println!("{t}");
            Result::<()>::Ok(())
        }
        OvationError::ClapError(e) => Err(e.into()),
        OvationError::CommandError(ctx, e) => {
            if !ctx.quiet {
                Err(e)
            } else {
                std::process::exit(2);
            }
        }
    })
}
