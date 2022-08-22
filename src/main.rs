pub mod pepe;

use std::env;

use pepe::animation_player::play_animation;
use pepe::asset_selector::avialable_pepes;
use pepe::asset_selector::select_pepe;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Gimme Pepe name!");
        return;
    }
    let pepe_name = &args[1];

    if pepe_name == "list" {
        println!("{}", avialable_pepes());
        return;
    }

    let selected_pepe = select_pepe(&pepe_name.to_string()).expect("Valid Pepe name");
    play_animation(selected_pepe).expect("Show terminal window with animated Pepe");
}
