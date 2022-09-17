// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    input.trim().to_string()

    // 1. whitespace가 아닌 문자가 나올 때 까지 index를 조정 (start index)
    // 2. whitespace인 문자가 나올 때 end index 설정 

    // let mut start_index = -1;
    // let mut end_index = -1;

    // for (index, b) in input.bytes().enumerate() {
    //     if start_index == -1 && b != b' ' {
    //         start_index = index;
    //     }

    //     if b != b' ' {
    //         end_index = index;
    //     }
    // }

    // for (index, c) in input.chars().enumerate() {
    //     if start_index == -1 && !c.is_whitespace() {
    //         start_index = index;
    //     }

    //     if !c.is_whitespace() {
    //         end_index = index;
    //     }
    // }

    // start_index, -1 이거나, whitespace가 아닌 첫 등장 문자
    // start_index == -1 => 모든 문자열이 whitespace 였다.

    // end_index, -1 이거나, 마지막 whitespace 뭉치가 시작하기 바로 전 문자
    // end_index == -1 => 모든 문자열이 whitespace 였다.
    // if start_index == -1 || end_index == -1 {
    //     return String::new();
    // }

    // input.bytes()[start_index..=end_index]

    // // left, right point 유지
    // // left -> 기록 시작할 

    // let mut s = String::with_capacity(input.len());

}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!

    // ? format!("{} world!", str) 도 가능할 것 같음? 무슨 차이?
    // let mut input = input.to_string();
    // input.push_str(" world!");
    // input

    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let before_word = "cars";
    let after_word = "balloons";

    let mut s = input.to_string();

    let start_index = match s.find(before_word) {
        None => return s,
        Some(index) => index
    };

    s.replace_range(start_index..start_index+before_word.len(), after_word);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
