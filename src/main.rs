extern crate gfacheck;
use clap::{Arg, App};
use std::path::Path;


fn main() {

    // Using Clap the first time
    // https://github.com/clap-rs/clap
    let matches = App::new("gfa-check")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Checking GFA files")
        .arg(Arg::new("gfa2")
            .short('g')
            .long("gfa")
            .about("Sets the input file to use")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("integer")
            .long("integer")
            .about("Check if all node ids are integer (u32)"))
        .arg(Arg::new("nodes in path")
            .short('n')
            .long("npath")
            .about("Checking if edges in path are present"))
        .arg(Arg::new("edges in path")
            .short('e')
            .long("epath")
            .about("Checking if edges in path are present"))
        .get_matches();

    // Read the command line arguments
    let input = matches.value_of("gfa2").unwrap();
    println!("This is the input {}", input);


    if Path::new(input).exists() {
        let h = gfacheck::read_gfa(input);
        let nothing = false;
        if matches.is_present("integer") & matches.is_present("nodes in path") & matches.is_present("nodes in path") & (matches.is_present("nodes in path") == false){
            nothing == false;
        }
        if matches.is_present("integer")  | nothing{
            println!("All nodes are integer: {}", gfacheck::is_int(&h));
        }
        if matches.is_present("nodes in path") | nothing{
            println!("Path nodes represented in nodes: {}", gfacheck::check_path_nodes(&h));
        }
        if matches.is_present("edges in path") | nothing {
            println!("Path edge represented in edges: {}", gfacheck::check_path_edges(&h));
        }
    }
    else {
        eprintln!("This is not a file");
    }

}
