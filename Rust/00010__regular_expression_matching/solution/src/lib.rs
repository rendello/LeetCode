
pub fn is_match(s: String, p: String) -> bool {
    is_match_internal(s.as_bytes(), p.as_bytes())
}

fn is_match_internal(s: &[u8], p: &[u8]) -> bool {
    if p.is_empty() {
        if s.is_empty() { return true; } else { return false; }
    }

    let is_same = !s.is_empty() && (s[0] == p[0] || p[0] as char == '.');
    let is_star = p.len() >= 2 && p[1] as char == '*';

    let p_rest = if is_star { &p[2..] } else { &p[1..] };

    match (is_star, is_same) {
        (false, false) => false,
        (false, true) => is_match_internal(&s[1..], &p_rest),
        (true, false) => is_match_internal(&s, &p_rest),
        (true, true) => {
            if is_match_internal(&s[1..], &p) {
                true
            } else {
                is_match_internal(&s, &p_rest)
            }
        }
    }
}