pub fn is_retweet(s: &str) -> bool {
    s.len() > 2 && &s[0..2] == "RT" // TODO: better way
}
