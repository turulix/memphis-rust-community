pub(crate) fn get_internal_name(name: &str) -> String {
    name.replace('.', "#")
}

const CHARS: &[u8] = b"0123456789abcdef";

pub(crate) fn get_unique_key(size: i32) -> String {
    let mut key = String::new();
    for _ in 0..size {
        key.push(CHARS[rand::random::<usize>() % 16] as char);
    }
    key
}

pub(crate) fn sanitize_name(name: &mut String, generate_suffix: bool) {
    *name = name.to_lowercase();

    if generate_suffix {
        *name = format!("{}_{}", name, get_unique_key(8));
    }
}
