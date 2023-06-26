pub(crate) fn get_internal_name(name: &String) -> String {
    name.replace(".", "#")
}

const CHARS: &[u8] = b"0123456789abcdef";

pub(crate) fn get_unique_key(size: i32) -> String {
    let mut key = String::new();
    for _ in 0..size {
        key.push(CHARS[rand::random::<usize>() % 16] as char);
    }
    key
}
