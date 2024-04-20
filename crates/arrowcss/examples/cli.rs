use arrowcss::{
    app::{get_files, Application},
    css::ToCssString,
    parser::to_css_rule,
};
use clap::{arg, command, Parser};

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

    let mut app = Application::new().unwrap();
    app.init();

    match opts.cmd {
        None => {
            if opts.watch {
                app.watch(&opts.input, opts.output.as_deref());
            } else {
                app.run_parallel(
                    get_files(&opts.input).as_slice(),
                    opts.output.as_deref(),
                );
            }
        }
        Some(SubCommand::Debug(cmd)) => {
            let (rule, _) = to_css_rule(&cmd.input, &app.ctx).unwrap();
            if cmd.print_ast {
                println!("{:#?}", rule);
            }
            println!("{}", &rule.to_css_string().unwrap());
        }
    }
}
