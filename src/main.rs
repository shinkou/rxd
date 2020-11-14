extern crate getopts;

use std::env;

use getopts::Options;

mod rxd;
use rxd::dump;

fn print_usage(progname: &str, opts: Options)
{
	println!
	(
		"{}"
		, opts.usage
		(
			&format!
			(
				"Usage: {} [options] file [ file ... ]"
				, progname
			)
		)
	);
}

fn do_args() -> (i64, u64, u8, Vec<String>)
{
	let mut offset: i64 = 0;
	let mut limit: u64 = 0;
	let mut bpl: u8 = 16;
	let args: Vec<String> = env::args().collect();
	let progname: &str = args[0].as_str();

	let mut opts = Options::new();
	opts.optflag("h", "help", "Show this help menu.");
	opts.optopt
	(
		"l"
		, "limit"
		, "Limit. (default: 0 meaning no limit)"
		, "NUM"
	);
	opts.optopt("o", "offset", "Offset. (default: 0)", "NUM");
	opts.optopt
	(
		"w"
		, "width"
		, "Width in number of bytes. (default: 16)"
		, "NUM"
	);
	let matches = match opts.parse(&args[1..])
	{
		Ok(m) => m
		, Err(e) => panic!(e.to_string())
	};

	if matches.opt_present("h") || matches.free.len() < 1
	{
		print_usage(&progname, opts);
		(offset, limit, bpl, Vec::new())
	}
	else
	{
		if let Some(s) = matches.opt_str("l")
		{
			limit = s.parse::<u64>().unwrap();
		}

		if let Some(s) = matches.opt_str("o")
		{
			offset = s.parse::<i64>().unwrap();
		}

		if let Some(s) = matches.opt_str("w")
		{
			bpl = s.parse::<u8>().unwrap();
		}

		(offset, limit, bpl, matches.free)
	}
}

fn main()
{
	let t = do_args();
	let mut delimit_line = false;

	for s in t.3
	{
		if delimit_line {println!();}

		if let Err(e) = dump(&s, t.0, t.1, t.2)
		{
			println!("Error: {}", e);
		}

		delimit_line = delimit_line || true;
	}
}
