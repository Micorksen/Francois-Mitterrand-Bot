/// Formate une chaîne de caractères en majuscules avec la première lettre en majuscule et le reste en minuscules.
pub fn title_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut start = true;

    for char in s.chars() {
        if start {
            out.extend(char.to_uppercase());
        } else {
            out.extend(char.to_lowercase());
        }

        start = char == ' ' || char == '-';
    }

    out
}
