use crate::consumer::MemphisConsumerOptions;

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

pub(crate) fn get_effective_consumer_name(options: &MemphisConsumerOptions) -> String {
    if options.consumer_group.is_empty() {
        get_internal_name(&options.consumer_name)
    } else {
        get_internal_name(&options.consumer_group)
    }
}
