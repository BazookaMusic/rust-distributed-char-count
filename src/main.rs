use char_count::char_count::distributed_character_count;
use clap::{Arg, App};

mod char_count;
mod server;
mod worker;


fn main() {
    let matches = App::new("Distributed Char Count")
        .version("0.1.0")
        .author("Cl10")
        .about("An example rust program that sets up a client-server architecture to calculate random character counts.")
        .arg(Arg::with_name("n_clients")
                 .short('c')
                 .long("n_clients")
                 .takes_value(true)
                 .help("The amount of clients to spawn."))
        .get_matches();

    let n_clients_str = matches.value_of("n_clients").unwrap_or("0");
    let n_clients = n_clients_str.parse::<usize>().unwrap_or(0);

    distributed_character_count(n_clients);
}