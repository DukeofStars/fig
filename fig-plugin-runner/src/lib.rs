use std::io::Read;

use fig::{
    plugin::{Instruction, Plugin},
    repository::RepositoryBuilder,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<P: Plugin> {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("The instruction received was not valid")]
    InvalidInstruction(u8),
    #[error("Failed to open repository")]
    RepositoryError(#[from] fig::repository::Error),
    #[error("{}", .0)]
    PluginError(P::Err),
}

pub fn run_plugin<P: Plugin>(mut plugin: P, stream: &mut impl Read) -> Result<(), Error<P>> {
    loop {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf)?;
        let [instr, len] = buf;
        let mut body = String::new();
        stream.take(len.into()).read_to_string(&mut body)?;
        if (Instruction::RunOnRepository as u8) == instr {
            let repository = RepositoryBuilder::new(body.into()).open()?;
            plugin
                .run_on_repository(repository)
                .map_err(Error::PluginError)?;
        } else if (Instruction::RunOnFile as u8) == instr {
            plugin
                .run_on_file(body.into())
                .map_err(Error::PluginError)?;
        } else if (Instruction::Exit as u8) == instr {
            return Ok(());
        } else {
            return Err(Error::InvalidInstruction(instr));
        }
    }
}
pub fn run_plugin_stdin<P: Plugin>(plugin: P) -> Result<(), Error<P>> {
    run_plugin(plugin, &mut std::io::stdin())
}

#[cfg(test)]
mod tests {
    // TODO: Use mocking library to create mock plugin. Then create tests
}
