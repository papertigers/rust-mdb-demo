use lazy_static::lazy_static;
use std::os::raw::c_int;

pub struct Modinfo(mdb_sys::mdb_modinfo_t);
unsafe impl Sync for Modinfo {}

pub struct Dcmd(mdb_sys::mdb_dcmd_t);
impl Dcmd {
    fn new() -> Self {
        Dcmd::default()
    }

    // XXX Ideally we would handle the str and \0 better
    fn name(mut self, name: &'static str) -> Self {
        self.0.dc_name = name.as_ptr() as _;
        self
    }

    // XXX Ideally we would handle the str and \0 better
    fn desc(mut self, desc: &'static str) -> Self {
        self.0.dc_descr = desc.as_ptr() as _;
        self
    }

    fn funcp(
        mut self,
        funcp: unsafe extern "C" fn(usize, u32, i32, *const mdb_sys::mdb_arg) -> i32,
    ) -> Self {
        self.0.dc_funcp = Some(funcp);
        self
    }
}

impl Default for Dcmd {
    fn default() -> Self {
        Dcmd(mdb_sys::mdb_dcmd_t {
            dc_name: std::ptr::null() as _,
            dc_usage: std::ptr::null(),
            dc_descr: std::ptr::null() as _,
            dc_funcp: None,
            dc_help: None,
            dc_tabp: None,
        })
    }
}
unsafe impl Sync for Dcmd {}

pub struct Dcmds(Box<[mdb_sys::mdb_dcmd_t]>);

impl Dcmds {
    fn from(dcmds: &[Dcmd]) -> Dcmds {
        let mut list: Vec<mdb_sys::mdb_dcmd_t> = dcmds.iter().map(|d| d.0).collect();
        list.push(Dcmd::default().0);
        Dcmds(list.into_boxed_slice())
    }
}

unsafe impl Sync for Dcmds {}

lazy_static! {
    pub static ref DCMDS: Dcmds = Dcmds::from(&[
        Dcmd::new()
            .name("rust_dcmd\0")
            .desc("hello from a rust mdb dcmd\0")
            .funcp(hello),
        Dcmd::new()
            .name("rprint\0")
            .desc("print a rust type\0")
            .funcp(hello),
    ]);
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
