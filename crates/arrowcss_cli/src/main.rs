use std::{fs::OpenOptions, io::Write};

use arrowcss::{app::Application, config::ArrowConfig, css::ToCssString};
use clap::{arg, command, Parser};
use config::{Config, File};
use read::get_files;
use run::RunParallel;
use watch::WatchApp;

mod read;
mod run;
mod watch;

#[derive(Debug, Parser)]
#[command(name = "arrowcss", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Option<SubCommand>,

    #[arg(short)]
    pub input: String,

    #[arg(short, help = "Output path (default: stdout)")]
    pub output: Option<String>,

    #[arg(short, default_value_t = false)]
    pub watch: bool,

    #[arg(short, long, help = "Enable strict mode")]
    pub strict: bool,

    #[arg(
        short,
        long,
        help = "Path to config file (default: arrow.config.{toml,yaml,json})",
        default_value = "arrow.config.toml"
    )]
    pub config: String,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    Debug(DebugCommand),
}

#[derive(Debug, Parser)]
pub struct DebugCommand {
    pub input: String,

    #[arg(short, long, default_value_t = false)]
    pub print_ast: bool,
}

fn main() {
    let opts = Opts::parse();

    let config = Config::builder()
        .add_source(File::with_name(&opts.config))
        .build()
        .map(|c| c.try_deserialize::<ArrowConfig>().unwrap_or_default())
        .unwrap_or_else(|_| ArrowConfig::default());

    let mut app = Application::new(config).init();

    match opts.cmd {
        None if opts.watch => {
            app.watch(&opts.input);
        }
        None => {
            let res = app.run_parallel(get_files(&opts.input));
            if opts.output.is_some() {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .append(false)
                    .open(opts.output.unwrap())
                    .unwrap()
                    .write(res.as_bytes())
                    .unwrap();
            } else {
                println!("{}", res);
            }
        }
        Some(SubCommand::Debug(cmd)) => {
            let r = app.ctx.generate(&cmd.input).unwrap();
            if cmd.print_ast {
                println!("{:#?}", r.rule);
            }
            println!("{}", &r.rule.to_css_string().unwrap());
        }
    }
}
