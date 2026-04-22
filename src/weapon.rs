use crate::item::Item;


pub trait Weapon: Item {

    /// cannot store ammo number (in case of the weapon
    /// do not have ammo (infinite ammo, by default))
    /// it mut be overrided by subclasses
    /// -> use decorator maybe or bridge (for reccursive supers())
    /// if no value provided -> infinite ammo
    /// some of organic weapons -> no ammo, the system differ
    fn get_ammo() -> Option<u16>;


}

