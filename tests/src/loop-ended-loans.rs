//@ [!lean] skip
//! Calling a `&mut self` library method on a temporary iterator before a loop
//! (here `<slice::Iter as Iterator>::next`) leaves the region abstraction of
//! the call in the context until the loop. Its borrows have ended, so it only
//! contains *ended* loans: an ended loan projector for the iterator, and, when
//! the call returned a reference, an ended shared loan. Matching the loop
//! entry context with the loop fixed point used to fail with "Could not match
//! the contexts" because the context matcher had no case for those.

/// `next` returns `Option<&u32>`, dereferenced before the loop: the call's
/// abstraction holds an ended loan projector and an ended shared loan.
pub fn count_first(v: &[u32], w: &[u32]) -> u32 {
    let c = match w.iter().next() {
        Some(c) => *c,
        None => return 0,
    };
    let mut n = 0;
    let mut i = 0;
    while i < v.len() {
        if v[i] == c {
            n += 1;
        }
        i += 1;
    }
    n
}

/// `Chars::next` returns an owned `Option<char>`: the call's abstraction only
/// holds an ended loan projector (for the iterator).
pub fn count_first_char(v: &[char], s: &str) -> u32 {
    let c = match s.chars().next() {
        Some(c) => c,
        None => return 0,
    };
    let mut n = 0;
    let mut i = 0;
    while i < v.len() {
        if v[i] == c {
            n += 1;
        }
        i += 1;
    }
    n
}
