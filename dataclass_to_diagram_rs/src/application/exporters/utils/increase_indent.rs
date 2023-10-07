/// Увеличить отступ на 4 пробела
pub fn increase_indent(input_string: &str) -> String {
    input_string
        .split("\n")
        .map(|line| format!("    {}", line))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let origin = "123
456
    789";
        let target = "    123
    456
        789";
        assert_eq!(increase_indent(origin), target);
    }
}
