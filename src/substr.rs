use core::num::Wrapping;

struct WindowHasher {
    state: Wrapping<u32>,
    window_size: u32,
    window: Vec<u8>,
}

const ALPHA: Wrapping<u32> = Wrapping(1_u32 << 8);

impl WindowHasher {
    fn new(first_window: &[u8]) -> Self {
        let window_size = first_window.len() as u32;
        let window = first_window.into();

        let mut state = Wrapping(0);
        for (i, c) in first_window.iter().copied().enumerate() {
            let i = i as u32;
            let alpha_term = ALPHA.pow(window_size - i - 1);

            state += alpha_term * Wrapping(c as u32);
        }

        WindowHasher {
            state,

            window_size,
            window,
        }
    }

    fn next(&mut self, new: u8) {
        // Remove the old byte
        let old = Wrapping(self.window.remove(0) as u32);

        // Add the new one
        self.window.push(new);

        // Update state
        let s = self.state - old * (ALPHA.pow(self.window_size - 1));
        self.state = ALPHA * s + Wrapping(new as u32);
    }

    fn hash(&self) -> u32 {
        self.state.0
    }
}

fn hash_it(text: &str) -> u32 {
    WindowHasher::new(text.as_bytes()).hash()
}

/// Returns the first byte offset of `pattern` in `text`.
pub fn substr(text: &str, pattern: &str) -> Option<usize> {
    if text.len() < pattern.len() || pattern.len() == 0 {
        return None;
    }

    let bytes = text.as_bytes();

    let p_len = pattern.as_bytes().len();
    let p_h = hash_it(pattern);

    // start and ending offsets into the byte stream to search
    let mut curr = 0;
    let mut end = pattern.as_bytes().len();

    let mut h = WindowHasher::new(&bytes[curr..end]);

    // Check if our pattern prefixes the text - our main loop will work and
    // then recheck this.
    if h.hash() == p_h {
        return Some(curr);
    }

    for b in bytes.iter().skip(p_len) {
        // Increment these first
        curr += 1;
        end += 1;

        h.next(*b);
        if h.hash() == p_h {
            return Some(curr);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_simple_match_head_one_ex() {
        let text = "Hello World!";
        let pattern = "H";

        let offset = substr(text, pattern);
        assert_eq!(Some(0), offset);
    }

    #[test]
    fn check_simple_match_head_ex() {
        let text = "Hello World!";
        let pattern = "Hello";

        let offset = substr(text, pattern);
        assert_eq!(Some(0), offset);
    }

    #[test]
    fn check_simple_match_tail_ex() {
        let text = "Hello World!";
        let pattern = "World!";

        let offset = substr(text, pattern);
        assert_eq!(Some(6), offset);
    }

    #[test]
    fn check_simple_match_tail_one_ex() {
        let text = "Hello World!";
        let pattern = "W";

        let offset = substr(text, pattern);
        assert_eq!(Some(6), offset);
    }

    #[test]
    fn check_simple_match_middle_ex() {
        let text = "Hello World, and to all who enjoy it.";
        let pattern = "World,";

        let offset = substr(text, pattern);
        assert_eq!(Some(6), offset);
    }
}
