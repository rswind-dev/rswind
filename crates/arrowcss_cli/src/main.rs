use arrowcss::{app::Application, config::ArrowConfig, css::ToCssString, source::SourceInput};
use clap::{arg, command, Parser};
use config::{Config, File};
use rayon::prelude::*;
use read::{get_files, ReadFromFile};
use watch::WatchApp;

mod read;
mod watch;

#[derive(Debug, Parser)]
#[command(name = "arrowcss", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Option<SubCommand>,

    #[arg(short, long, default_value = "example/html")]
    pub input: String,

    #[arg(short, long, help = "Output path (default: stdout)")]
    pub output: Option<String>,

    #[arg(short, long, default_value_t = false)]
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

    let mut app = Application::new(config);
    app.init();

    match opts.cmd {
        None => {
            if opts.watch {
                app.watch(&opts.input);
            } else {
                app.run_parallel(
                    get_files(&opts.input)
                        .par_iter()
                        .map(SourceInput::from_file),
                );
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
