mod pepe_ez;
mod pepe_pepega;
mod pepe_punch;
mod pepe_sad;
mod pepe_sponge;
mod pepe_think;

use std::{collections::HashMap, fmt};

use super::{Pepe, Possition};

use self::{
    pepe_ez::PEPE_EZ, pepe_pepega::PEPE_PEPEGA, pepe_punch::PEPE_PUNCH, pepe_sad::PEPE_SAD,
    pepe_sponge::PEPE_SPONGE, pepe_think::PEPE_THINK,
};

#[derive(Debug, Clone)]
pub struct NotFoundError;

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid Pepe name")
    }
}

fn get_pepes() -> HashMap<&'static str, Pepe> {
    return HashMap::from([
        PEPE_EZ.get_with_name_key(),
        PEPE_PEPEGA.get_with_name_key(),
        PEPE_PUNCH.get_with_name_key(),
        PEPE_SAD.get_with_name_key(),
        PEPE_SPONGE.get_with_name_key(),
        PEPE_THINK.get_with_name_key(),
    ]);
}

pub fn avialable_pepes() -> String {
    let pepes = get_pepes();
    let mut valid_keys: String = "Pepes i know:\n".to_string();
    for key in pepes.keys() {
        valid_keys.push_str(key);
        valid_keys.push_str(", ");
    }
    return valid_keys;
}

pub fn select_pepe(name: &str) -> Result<Pepe, NotFoundError> {
    let pepes = get_pepes();
    return match pepes.get(name) {
        Some(&pepe) => Ok(pepe),
        _ => Err(NotFoundError),
    };
}
