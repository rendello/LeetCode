
enum PatternType {
    Single,
    Star
}

pub fn is_match(s: String, p: String) -> bool {
    is_match_internal(s.as_bytes(), p.as_bytes())
}

fn is_match_internal(s: &[u8], p: &[u8]) -> bool {
    match (s.is_empty(), p.is_empty()) {
        (true, true) => true,
        (false, true) => false,
        (true, false) => {
            if p.len() % 2 != 0 {
                return false;
            } else {
                for i in (1..p.len()).step_by(2) {
                    if p[i] as char != '*' { return false; }
                }
                true
            }
        }
        (false, false) => {
            let first = s[0] as char;
            let rest = &s[1..];

            let p_first = p[0] as char;
            let (p_type, p_rest) = {
                if p.len() >= 2 && p[1] as char == '*' {
                    (PatternType::Star, &p[2..])
                } else {
                    (PatternType::Single, &p[1..])
                }
            };

            match (p_type, first==p_first || p_first=='.') {
                (PatternType::Single, true) => is_match_internal(&rest, &p_rest),
                (PatternType::Single, false) => false,
                (PatternType::Star, true) => match is_match_internal(&rest, &p) {
                    true => true,
                    false => is_match_internal(&s, &p_rest)
                }
                (PatternType::Star, false) => is_match_internal(&s, &p_rest)
            }
        }
    }
}