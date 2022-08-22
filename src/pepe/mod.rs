pub mod asset_selector;
pub mod animation_player;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Possition {
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

impl Pepe {
    pub fn get_with_name_key(&self) -> (&str, Pepe) {
        return (self.name, *self);
    }
    pub fn get_width(&self) -> u16 {
        let mut width: u16 = 0;
        let mut current_width: u16 = 0;
        for char in self.pepe.chars() {
            if char != '\n' {
                current_width +=1;
            }
            else {
                if current_width > width {
                    width = current_width;
                }
                current_width = 0;
            }
        }
        return width;
    }
    pub fn get_height(&self) -> u16 {
        let mut height: u16 = 0;
        for char in self.pepe.chars() {
            if char == '\n' {
                height +=1;
            }
        }
        return height;
    }
}