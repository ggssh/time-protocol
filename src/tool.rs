// 4*u8->i32
pub fn u8_to_i32(v: [u8; 4]) -> i32 {
    if v.len() < 4 {
        return 0;
    }
    unsafe {
        let i32ptr = v.as_ptr() as *const i32;
        return *i32ptr;
    }
}

/// i32->4*u8
pub fn i32_to_u8(v: i32) -> [u8; 4] {
    unsafe {
        let i32ptr = &v as *const i32;
        let u8ptr = i32ptr as *const u8;
        return [
            *u8ptr.offset(0),
            *u8ptr.offset(1),
            *u8ptr.offset(2),
            *u8ptr.offset(3),
        ];
    }
}

pub fn u8_to_i64(v: [u8; 8]) -> i64 {
    if v.len() < 8 {
        return 0;
    }
    unsafe {
        let i64ptr = v.as_ptr() as *const i64;
        return *i64ptr;
    }
}

pub fn i64_to_u8(v: i64) -> [u8; 8] {
    unsafe {
        let i64ptr = &v as *const i64;
        let u8ptr = i64ptr as *const u8;
        return [
            *u8ptr.offset(0),
            *u8ptr.offset(1),
            *u8ptr.offset(2),
            *u8ptr.offset(3),
            *u8ptr.offset(4),
            *u8ptr.offset(5),
            *u8ptr.offset(6),
            *u8ptr.offset(7),
        ];
    }
}

pub fn vec_to_string(v: &Vec<String>) -> String {
    let mut s = String::new();
    for i in v {
        s.push_str(i);
        s.push('\n');
    }
    s
}
