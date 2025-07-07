use std::path::PathBuf;

#[derive(facet::Facet)]
// #[command(author="Dmitrii Kovanikov <kovanikov@gmail.com>", version, about="A CLI tool to manage other CLI tools", long_about = None)]
pub(crate) struct Args {
    #[facet(named, short = 'c', default=None)]
    pub config: Option<PathBuf>,
    #[facet(named, short = 'p', default=None)]
    pub proxy: Option<String>,
    #[facet(positional)]
    pub command: Command,
    #[facet(positional)]
    pub tool: Option<String>,
}

#[repr(u8)]
#[derive(facet::Facet)]
#[facet(rename_all = "kebab-case")]
#[allow(unused)] // facet renamed
pub enum Command {
    /// Print a default .tool.toml configuration to std out
    DefaultConfig {
        /// Print the default config file location instead
        #[facet(long)]
        path: bool,
    },

    /// Sync all tools specified in configuration file or the only one specified
    /// in the command line
    Sync,

    /// Install a tool if it is hardcoded into internal database
    Install,
}
