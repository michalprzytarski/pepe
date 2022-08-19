pub mod asset_selector;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Gimme Pepe name!");
        return;
    }
    let pepe_name = &args[1];

    if pepe_name == "help" {
        println!("{}", asset_selector::avialable_pepes());
        return ;
    }

    println!("{}", asset_selector::select_pepe(&pepe_name.to_string()));
}
