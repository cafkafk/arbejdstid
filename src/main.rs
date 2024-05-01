use chrono::prelude::*;
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1].len() != 8 as usize {
        println!("usage: arbejdstid hh:mm:ss");
        std::process::abort();
    }

    let buffer = &args[1];

    let time_only = NaiveTime::parse_from_str(&buffer, "%H:%M:%S").unwrap();

    let local: DateTime<Local> = Local::now();

    println!("delta: \t\t{time_only} -> {buffer}");

    let dt = NaiveTime::signed_duration_since(local.time(), time_only);

    let hours = (dt.num_seconds() / 60) / 60;
    let minutes = (dt.num_seconds() / 60) % 60;
    let seconds = dt.num_seconds() % 60;

    println!("elapsed: \t{}:{}:{}", &hours, &minutes, &seconds);

    let total = format!("{}.{:.0}", &hours, 100. / 60. * minutes as f32)
        .parse::<f64>()
        .unwrap();

    println!("---",);

    println!(
        "{}",
        format!("onsite: \t{}", total - 0.5).replacen(".", ",", 1)
    );
    println!("lunch: \t\t0,5");

    println!("---",);

    println!("total: \t\t{},{:.0}", &hours, 100. / 60. * minutes as f32);
    Ok(())
}
