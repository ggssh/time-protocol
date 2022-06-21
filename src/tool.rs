pub fn u8_to_i32(v: [u8; 4]) -> i32 {
    if v.len() < 4 {
        return 0;
    }
    unsafe {
        let i32ptr = v.as_ptr() as *const i32;
        return *i32ptr;
    }
}

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