mod build;
mod md2html;

use std::env;

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [FILE] [options],", program);
    print!("{}", opts.usage(&brief));
}

// TODO: complete program, input output flags,

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("a", "all", "rebuild all html files");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!("{}", e)
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("a") {
        build::build_blog();
    }
}
