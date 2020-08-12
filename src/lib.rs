use lazy_static::lazy_static;
use std::os::raw::{c_char, c_int};

pub struct Modinfo(mdb_sys::mdb_modinfo_t);
unsafe impl Sync for Modinfo {}

pub struct Dcmd(mdb_sys::mdb_dcmd_t);
unsafe impl Sync for Dcmd {}

pub struct Dcmds(Box<[mdb_sys::mdb_dcmd_t]>);

impl Dcmds {
    fn new() -> Dcmds {
        let dcmd = mdb_sys::mdb_dcmd_t {
            dc_name: std::ptr::null() as _,
            dc_usage: std::ptr::null(),
            dc_descr: std::ptr::null() as _,
            dc_funcp: None,
            dc_help: None,
            dc_tabp: None,
        };
        let mut list = vec![dcmd; 3].into_boxed_slice();
        list[0] = RUSTDCMD.0;
        Dcmds(list)
    }
}

unsafe impl Sync for Dcmds {}

pub static RUSTDCMD: Dcmd = Dcmd(mdb_sys::mdb_dcmd_t {
    dc_name: "rust_dcmd\0".as_ptr() as *const c_char,
    dc_usage: std::ptr::null(),
    dc_descr: "hello from a rust mdb dcmd\0".as_ptr() as *const c_char,
    dc_funcp: Some(hello),
    dc_help: None,
    dc_tabp: None,
});

lazy_static! {
    pub static ref DCMDS: Dcmds = Dcmds::new();
}

lazy_static! {
    pub static ref MODINFO: Modinfo = Modinfo(mdb_sys::mdb_modinfo_t {
        mi_dvers: mdb_sys::MDB_API_VERSION as _,
        mi_dcmds: DCMDS.0.as_ptr() as _,
        mi_walkers: std::ptr::null(),
    });
}

#[no_mangle]
pub extern "C" fn hello(
    _addr: usize,
    _flags: u32,
    _argc: i32,
    _argv: *const mdb_sys::mdb_arg,
) -> c_int {
    0
}

#[no_mangle]
pub extern "C" fn _mdb_init() -> *const mdb_sys::mdb_modinfo_t {
    &MODINFO.0 as _
}
