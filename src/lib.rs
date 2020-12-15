use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
    ;

    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u32_bytes(u: &u32) -> [u8; 4] {
    [
        (u >> (8 * 0)) as u8,
        (u >> (8 * 1)) as u8,
        (u >> (8 * 2)) as u8,
        (u >> (8 * 3)) as u8,
    ]
}

pub fn u64_bytes(u: &u64) -> [u8; 8] {
    [
        (u >> (8 * 0)) as u8,
        (u >> (8 * 1)) as u8,
        (u >> (8 * 2)) as u8,
        (u >> (8 * 3)) as u8,
        (u >> (8 * 4)) as u8,
        (u >> (8 * 5)) as u8,
        (u >> (8 * 6)) as u8,
        (u >> (8 * 7)) as u8,
    ]
}

pub fn u128_bytes(u: &u128) -> [u8; 16] {
    [
        (u >> (8 * 0)) as u8,
        (u >> (8 * 1)) as u8,
        (u >> (8 * 2)) as u8,
        (u >> (8 * 3)) as u8,
        (u >> (8 * 4)) as u8,
        (u >> (8 * 5)) as u8,
        (u >> (8 * 6)) as u8,
        (u >> (8 * 7)) as u8,
        (u >> (8 * 8)) as u8,
        (u >> (8 * 9)) as u8,
        (u >> (8 * 10)) as u8,
        (u >> (8 * 11)) as u8,
        (u >> (8 * 12)) as u8,
        (u >> (8 * 13)) as u8,
        (u >> (8 * 14)) as u8,
        (u >> (8 * 15)) as u8,
    ]
}