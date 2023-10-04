use std::sync::Mutex;

static ALIAS: Mutex<u32> = Mutex::new(0);

pub fn generate_alias() -> u32 {
    let mut value = ALIAS.lock().unwrap();
    let current_value = value.clone();
    *value += 1;
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(generate_alias(), 0);
        assert_eq!(generate_alias(), 1);
        assert_eq!(generate_alias(), 2);
    }
}
