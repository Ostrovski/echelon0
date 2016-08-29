use std::env;
use std::io;
use std::io::BufRead;
use std::process;

extern crate getopts;
use getopts::Options;

extern crate monstrio;


fn print_usage(opts: &Options, program: &String) {
    let brief = format!("Usage: {} [file ...] [-s start_time] [-f stop_time]",
                        program);
    println!("{}", opts.usage(&brief));
}

fn handle_bad_opts(err: &String, program: &String) {
    println!("{} Try \"{} -h\" for help.", err, program);
    process::exit(1);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("s",
                "start",
                "if presents only entries after it will be processed",
                "START_TIME");
    opts.optopt("f",
                "stop",
                "if presents only entries before it will be processed",
                "STOP_TIME");
    opts.optflag("h", "help", "show this message");
    let args = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => return handle_bad_opts(&f.to_string(), &program),
    };

    if args.opt_present("h") {
        print_usage(&opts, &program);
        return;
    }

    // let stdin = io::stdin();
    // let stdin_input = monstrio::Input::stdin(&stdin);

    let mut glob_input = monstrio::Input::glob(args.free.into_iter());
    let reader = glob_input.as_mut();

    loop {
        let mut line = String::new();
        reader.read_line(&mut line);
        println!("{}", line);
    }
}
