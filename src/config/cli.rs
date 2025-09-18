use std::path::PathBuf;

#[derive(facet::Facet)]
// #[command(author="Dmitrii Kovanikov <kovanikov@gmail.com>", version, about="A CLI tool to manage other CLI tools", long_about = None)]
pub(crate) struct Args {
    #[facet(named, short = 'c', default=None)]
    pub config: Option<PathBuf>,
    #[facet(named, short = 'p', default=None)]
    pub proxy: Option<String>,

    #[facet(positional)]
    pub tool: String,
}
