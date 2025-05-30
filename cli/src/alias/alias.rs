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

pub fn alias(ctx: &mut CLIContext, command: &AliasCommands) -> anyhow::Result<()> {
    match command {
        AliasCommands::Add(args) => set(ctx, args, false),
        AliasCommands::Set(args) => set(ctx, args, true),
        AliasCommands::Delete(args) => delete(ctx, args),
    }
}
