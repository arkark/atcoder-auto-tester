use atcoder_auto_tester::{app::App, args::Args, config::Config};

fn main() {
    let args: Args = Args::load();

    if args.should_clean {
        App::clean(&args.test_directory);
    }
    if args.should_login {
        App::login();
    }

    let config: Config = Config::load(&args.config_file);
    App::run(args, config);
}
