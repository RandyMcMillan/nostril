use gnostr_bins::{blockheight, relays, weeble, wobble};

pub fn get_relays_list() -> String {
    relays().unwrap()
}
pub fn get_weeble() -> u8 {
    weeble().unwrap() as u8
}
pub fn get_blockheight() -> u8 {
    blockheight().unwrap() as u8
}
pub fn get_wobble() -> u8 {
    wobble().unwrap() as u8
}
pub fn div(left: usize, right: usize) -> usize {
    left / right
}
pub fn modulus(left: usize, right: usize) -> usize {
    left % right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weeble_mod_blockheight() {
        let result = get_weeble() % get_blockheight();
        assert_eq!(result % 1 as u8, 0);
    }
    #[test]
    fn blockheight_mod_weeble() {
        let result = get_blockheight() % get_weeble();
        assert_eq!(result % 1 as u8, 0);
    }
}
