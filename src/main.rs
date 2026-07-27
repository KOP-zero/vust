use vust::cli::{Arg, Commands, Parser};
fn main() {
    let arg = Arg::parse();
    match arg.command {
        Commands::New(mut arg) => {
            if arg.interactive {
                arg.interactive().unwrap();
            }
            vust::new_project(&arg);
        }
        Commands::Dev { path } => {
            vust::dev_start(path);
        }
        Commands::Build { path, output } => {
            vust::build_start(path, output);
        }
    }
}
