use std::error::Error;

use crate::CLIContext;

use super::{
    delete::{delete, DeleteArgs},
    set::{set, SetArgs},
};

use clap::{command, Subcommand};

#[derive(Debug, Subcommand)]
pub enum AliasCommands {
    /// Attempts to create a new dictionary alias, failing if one already exists with the given name
    #[command(arg_required_else_help = true)]
    Add(SetArgs),

    /// Creates or updates an existing dictionary alias
    #[command(arg_required_else_help = true)]
    Set(SetArgs),

    /// Deletes an alias with the given name if it exists
    #[command(arg_required_else_help = true)]
    Delete(DeleteArgs),
}

pub fn alias(ctx: &mut CLIContext, command: &AliasCommands) -> Result<(), Box<dyn Error>> {
    match command {
        AliasCommands::Add(ref args) => set(ctx, args, false),
        AliasCommands::Set(ref args) => set(ctx, args, true),
        AliasCommands::Delete(ref args) => delete(ctx, args),
    }
}
