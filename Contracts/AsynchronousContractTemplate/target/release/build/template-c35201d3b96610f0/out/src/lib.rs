#![no_std] pub use orig_project::*;

#[allow(improper_ctypes)]
mod fake_gsys {
    extern "C" {
        pub fn gr_reply(
            payload: *const u8,
            len: u32,
            value: *const u128,
            err_mid: *mut [u8; 36],
        );
    }
}

#[no_mangle]
extern "C" fn metahash() {
    const METAHASH: [u8; 32] = [67, 217, 71, 74, 47, 43, 147, 230, 185, 27, 155, 176, 187, 215, 90, 255, 154, 42, 114, 96, 93, 28, 153, 52, 61, 197, 183, 44, 82, 165, 168, 183];
    let mut res: [u8; 36] = [0; 36];
    unsafe {
        fake_gsys::gr_reply(
            METAHASH.as_ptr(),
            METAHASH.len() as _,
            u32::MAX as _,
            &mut res as _,
        );
    }
}
