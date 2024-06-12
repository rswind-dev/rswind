use std::ffi::OsString;

use clap::{command, Parser};
use colored::Colorize;
use rswind::{
    config::GeneratorConfig, css::ToCssString, io::write_output, preset::preset_tailwind,
    processor::GeneratorProcessor,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use watch::WatchApp;

pub mod watch;

#[derive(Debug, Parser)]
#[command(name = "rswind", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Option<SubCommand>,

    pub content: Vec<String>,

    #[arg(short, help = "Output path (default: stdout)")]
    pub output: Option<String>,

    #[arg(short, default_value_t = false, help = "Enable watch mode")]
    pub watch: bool,

    #[arg(short, long, help = "Enable strict mode")]
    pub strict: bool,

    #[arg(long, help = "Path to config file", default_value = "arrow.config.json")]
    pub config: String,

    #[arg(short, long, help = "Path to working directory", default_value = ".")]
    pub cwd: String,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    Debug(DebugCommand),
    Init(InitCommand),
}

#[derive(Debug, Parser)]
pub struct DebugCommand {
    pub input: String,

    #[arg(short, long, default_value_t = false)]
    pub print_ast: bool,
}

#[derive(Debug, Parser)]
pub struct InitCommand {}

pub fn cli<I>(args: I)
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RSWIND_LOG"))
        .init();

    let opts = Opts::parse_from(args);

    let mut app = GeneratorProcessor::builder()
        .with_preset(preset_tailwind)
        .with_config(GeneratorConfig::from_file(&opts.config).unwrap())
        .with_watch(opts.watch)
        .build()
        .unwrap();

    match opts.cmd {
        None if opts.watch => {
            app.watch(opts.output.as_deref());
        }
        None => {
            let res = app.generate_contents();
            write_output(&res.css, opts.output.as_deref());
        }
        Some(SubCommand::Debug(cmd)) => match app.processor.design.generate(&cmd.input) {
            Some(r) => {
                if cmd.print_ast {
                    println!("{:#?}", r.rule);
                }
                println!("Generated {}:\n", cmd.input.green());
                println!("{}", &r.rule.to_css_string());
            }
            None => {
                eprintln!("Not a valid utility: {}", cmd.input.red());
            }
        },
        Some(SubCommand::Init(_)) => {
            write_output("{}", Some("arrow.config.json"));
        }
    }
}
