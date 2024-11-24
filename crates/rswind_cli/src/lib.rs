use std::{ffi::OsString, path::PathBuf};

use clap::{command, Parser};
use colored::Colorize;
use rswind::{
    config::GeneratorConfig,
    generator::AppBuildError,
    io::{write_output, OutputChannel},
    preset::{tailwind_preset, tailwind_theme},
    processor::GeneratorProcessor,
};
use rswind_css::ToCssString;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use watch::WatchApp;

mod watch;

#[derive(Debug, Parser)]
#[command(name = "rswind", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Option<SubCommand>,

    pub content: Vec<String>,

    #[arg(short, help = "Output path", default_value_t = OutputChannel::Stdout)]
    pub output: OutputChannel,

    #[arg(short, long, default_value_t = false, help = "Enable watch mode")]
    pub watch: bool,

    #[arg(short, long, help = "Enable strict mode")]
    pub strict: bool,

    #[arg(long, help = "Path to config file", default_value = "rswind.config.json")]
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

pub fn cli<I>(args: I) -> Result<(), AppBuildError>
where
    I: IntoIterator,
    I::Item: Into<OsString> + Clone,
{
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RSWIND_LOG"))
        .init();

    let mut opts = Opts::parse_from(args);

    let mut app = GeneratorProcessor::builder()
        .with_theme(tailwind_theme)
        .with_preset(tailwind_preset)
        .with_config(GeneratorConfig::from_file(&opts.config)?)
        .with_watch(opts.watch)
        .with_base(Some(opts.cwd.clone()))
        .build()?;

    if let OutputChannel::FileSystem(path) = opts.output {
        opts.output = OutputChannel::FileSystem(PathBuf::from(&opts.cwd).join(path))
    }

    match opts.cmd {
        None if opts.watch => {
            app.watch(&opts.output);
        }
        None => {
            let res = app.generate_contents();
            write_output(&res.css, &opts.output);
        }
        Some(SubCommand::Debug(cmd)) => match app.processor.design.generate(&cmd.input) {
            Some(r) => {
                if cmd.print_ast {
                    println!("{:#?}", r.rule);
                }
                println!("Generated {}:\n", cmd.input.green());
                println!("{}", &r.rule.to_css_string());
                if let Some(extra) = &r.extra_css {
                    println!("{}", &extra.to_css_string());
                }
            }
            None => {
                eprintln!("Not a valid utility: {}", cmd.input.red());
            }
        },
        Some(SubCommand::Init(_)) => {
            write_output("{}", &OutputChannel::FileSystem("rswind.config.json".into()));
        }
    };

    Ok(())
}
