use clap::{Arg, Command};

mod commands;

fn main() {
    let matches = Command::new("MyCrawlerCLI")
        .subcommand_required(true)
        .subcommand(
            Command::new("startproject")
                .about("Creates a new project with a specified name and package.")
                .arg(Arg::new("project_name").required(true).index(1))
                .arg(Arg::new("package").required(true).index(2))
                .arg(Arg::new("add-quote-example")
                    .long("add-quote-example")
                    .required(false)
                    .help("Adds a quote example class to the project")),
        )
        .subcommand(
            Command::new("genspider")
                .about("Generates a new spider with a name and a start URL.")
                .arg(Arg::new("spider_name").required(true).index(1))
                .arg(Arg::new("start_url").required(true).index(2)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("startproject", sub_m)) => {
            let project_name = sub_m.get_one::<String>("project_name").unwrap();
            let package = sub_m.get_one::<String>("package").unwrap();
            let include_quote_example = sub_m.contains_id("add-quote-example");
            commands::start_project::start_project(project_name, package, include_quote_example);
        }
        Some(("genspider", sub_m)) => {
            let spider_name = sub_m.get_one::<String>("spider_name").unwrap();
            let start_url = sub_m.get_one::<String>("start_url").unwrap();
            commands::gen_spider::gen_spider(spider_name, start_url);
        }
        _ => unreachable!(),
    }
}
