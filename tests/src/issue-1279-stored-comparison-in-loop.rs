//@ [!lean] skip

//! Companion of `issue-1279-comparison-prop-type.rs`: a short-circuit `||`
//! stored in a boolean inside a loop body. Before the fix the bound
//! expression was `if a = 1 then ok true else ok (a = b)`, whose `else`
//! branch is a `Prop`, and the generated Lean did not type-check. The same
//! statement outside a loop, or as an `if` condition, already translated.

pub fn count_skips(chars: Vec<char>, c: char, total: i32) -> i32 {
    let mut seen = 0;
    let mut skips = 0;
    let mut j = 0;
    while j < chars.len() {
        let x = chars[j];
        let mut skip = false;
        if x == c {
            seen += 1;
            skip = seen == 1 || seen == total;
        }
        if skip {
            skips += 1;
        }
        j += 1;
    }
    skips
}
