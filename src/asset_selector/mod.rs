mod pepe_ez;
mod pepe_pepega;
mod pepe_punch;
mod pepe_sad;
mod pepe_sponge;
mod pepe_think;

use std::collections::HashMap;

use self::{
    pepe_ez::PEPE_EZ, pepe_pepega::PEPE_PEPEGA, pepe_punch::PEPE_PUNCH, pepe_sad::PEPE_SAD,
    pepe_sponge::PEPE_SPONGE, pepe_think::PEPE_THINK,
};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Possition {
    Start,
    Center,
    End,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Pepe {
    name: &'static str,
    pepe: &'static str,
    horizontal: Possition,
    vertical: Possition,
}

fn get_pepes() -> HashMap<&'static str, Pepe> {
    return HashMap::from([
        (PEPE_EZ.name, PEPE_EZ),
        (PEPE_PEPEGA.name, PEPE_PEPEGA),
        (PEPE_PUNCH.name, PEPE_PUNCH),
        (PEPE_SAD.name, PEPE_SAD),
        (PEPE_SPONGE.name, PEPE_SPONGE),
        (PEPE_THINK.name, PEPE_THINK),
    ]);
}

pub fn avialable_pepes() -> String {
    let pepes = get_pepes();
    // let mut valid_keys: String = "Pepes i know:\n".to_string();
    let mut valid_keys: String = "Pepes i know:\n".to_string();
    for key in pepes.keys() {
        valid_keys.push_str(key);
        valid_keys.push_str(", ");
    }
    return valid_keys;
}

pub fn select_pepe(name: &str) -> &str {
    let pepes = get_pepes();
    match pepes.get(name) {
        Some(&pepe) => return pepe.pepe,
        _ => return "Don't know him ¯\\_(ツ)_/¯",
    }
}
