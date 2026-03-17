use std::{fs, path::PathBuf};

use clap::Parser;

use vsmap_lib::features::{Ore, Resource};
use vsmap_lib::map::{Map, render};

mod renderer;
use crate::renderer::SvgRenderer;

#[derive(Parser)]
struct Cli {
    spec: PathBuf,
    #[arg(default_value = "map.svg")]
    out: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let content = fs::read_to_string(cli.spec).expect("couldn't read file");
    let data: Map = toml::from_str(content.as_str()).expect("failed to parse");

    let o = Ore::Hematite.resource();
    if let Resource::Metal(m) = o {
        println!("Resource: {:?}", m.properties());
    }

    let out = fs::File::create(&cli.out).expect("couldn't open file for writing");
    let buf_out = std::io::BufWriter::new(out);
    let mut renderer = SvgRenderer { writer: buf_out };
    render(&data, &mut renderer);
}
