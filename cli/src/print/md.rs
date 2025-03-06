use std::env::current_dir;

use pulldown_cmark::Parser;
use pulldown_cmark_mdcat::{
    push_tty, resources::NoopResourceHandler, Environment, Settings, TerminalProgram, TerminalSize,
    Theme,
};
use syntect::parsing::SyntaxSet;

pub fn render_md(md: &String) -> color_eyre::Result<String> {
    // let settings = Settings {
    //     terminal_capabilities: TerminalProgram::Ansi.capabilities(),
    //     terminal_size: TerminalSize::default(),
    //     theme: Theme::default(),
    //     syntax_set: &SyntaxSet::default(),
    // };

    // let parser = Parser::new(md.as_ref());
    // let mut sink = Vec::new();
    // let env = Environment::for_local_directory(&current_dir()?)?;

    // push_tty(&settings, &env, &NoopResourceHandler, &mut sink, parser).unwrap();
    Ok(termimad::inline(md).to_string())
}
