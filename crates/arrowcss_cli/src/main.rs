use arrowcss::{app::Application, config::ArrowConfig, css::ToCssString, preset::preset_tailwind};
use arrowcss_cli::io::write_file;
use clap::{arg, command, Parser};
use io::get_files;
use run::RunParallel;
use watch::WatchApp;

mod io;
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
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    let builder = Application::builder()
        .with_preset(preset_tailwind)
        .with_config(ArrowConfig::from_file(&opts.config).unwrap());

    match opts.cmd {
        None if opts.watch => {
            let mut app = builder.watch().build();
            app.watch(&opts.input, opts.output.as_deref());
        }
        None => {
            let mut app = builder.build();
            let res = app.run_parallel(get_files(&opts.input));
            if let Some(output) = opts.output {
                write_file(&res, output);
            } else {
                println!("{}", res);
            }
        }
        Some(SubCommand::Debug(cmd)) => {
            let app = builder.build();
            let r = app.ctx.generate(&cmd.input).unwrap();
            if cmd.print_ast {
                println!("{:#?}", r.rule);
            }
            println!("{}", &r.rule.to_css_string());
        }
    }
}
