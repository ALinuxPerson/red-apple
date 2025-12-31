use std::ffi::{CStr, CString};
use libc::{c_char, c_int, c_void, mode_t, FILE, DIR, stat};
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf, Component};
use ctor::ctor;

#[ctor]
fn init() {
    // println!("Wet-appl library loaded!");
}

#[repr(C)]
pub struct Interpose {
    new_func: *const c_void,
    orig_func: *const c_void,
}

unsafe impl Sync for Interpose {}

macro_rules! interpose {
    ($interpose_name:ident, $my_func:ident, $libc_func:path) => {
        #[used]
        #[unsafe(link_section = "__DATA,__interpose")]
        pub static $interpose_name: Interpose = Interpose {
            new_func: $my_func as *const c_void,
            orig_func: $libc_func as *const c_void,
        };
    };
}

fn transform_path(path_str: &str) -> String {
    let path = Path::new(path_str);
    let mut new_path = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Normal(os_str) => {
                if let Some(s) = os_str.to_str() {
                    if s.len() > 255 {
                        let mut hasher = Sha256::new();
                        hasher.update(s.as_bytes());
                        let result = hasher.finalize();
                        let hash_hex = hex::encode(result);
                        // Use the hash as the new name.
                        // We use a prefix to ensure it doesn't collide with normal short names easily
                        // and to make it recognizable if needed.
                        // 64 chars hex + prefix is well within 255.
                        new_path.push(format!("SIG_{}", hash_hex));
                    } else {
                        new_path.push(os_str);
                    }
                } else {
                    new_path.push(os_str);
                }
            }
            _ => new_path.push(component),
        }
    }

    new_path.to_string_lossy().into_owned()
}

fn transform_cstring(ptr: *const c_char) -> Option<CString> {
    if ptr.is_null() {
        return None;
    }
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        if let Ok(s) = c_str.to_str() {
            let new_s = transform_path(s);
            // Only allocate new CString if the path actually changed
            if new_s != s {
                return Some(CString::new(new_s).unwrap());
            }
        }
    }
    None
}

// open
interpose!(INTERPOSE_OPEN, my_open, libc::open);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_open(path: *const c_char, flags: c_int, mode: c_int) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::open(ptr, flags, mode) }
}

// fopen
interpose!(INTERPOSE_FOPEN, my_fopen, libc::fopen);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE {
    let new_path = transform_cstring(filename);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(filename);
    unsafe { libc::fopen(ptr, mode) }
}

// opendir
interpose!(INTERPOSE_OPENDIR, my_opendir, libc::opendir);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_opendir(dirname: *const c_char) -> *mut DIR {
    let new_path = transform_cstring(dirname);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(dirname);
    unsafe { libc::opendir(ptr) }
}

// mkdir
interpose!(INTERPOSE_MKDIR, my_mkdir, libc::mkdir);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_mkdir(path: *const c_char, mode: mode_t) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::mkdir(ptr, mode) }
}

// rmdir
interpose!(INTERPOSE_RMDIR, my_rmdir, libc::rmdir);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_rmdir(path: *const c_char) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::rmdir(ptr) }
}

// unlink
interpose!(INTERPOSE_UNLINK, my_unlink, libc::unlink);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_unlink(path: *const c_char) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::unlink(ptr) }
}

// chdir
interpose!(INTERPOSE_CHDIR, my_chdir, libc::chdir);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_chdir(path: *const c_char) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::chdir(ptr) }
}

// stat
interpose!(INTERPOSE_STAT, my_stat, libc::stat);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_stat(path: *const c_char, buf: *mut stat) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::stat(ptr, buf) }
}

// lstat
interpose!(INTERPOSE_LSTAT, my_lstat, libc::lstat);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_lstat(path: *const c_char, buf: *mut stat) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::lstat(ptr, buf) }
}

// access
interpose!(INTERPOSE_ACCESS, my_access, libc::access);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_access(path: *const c_char, mode: c_int) -> c_int {
    let new_path = transform_cstring(path);
    let ptr = new_path.as_ref().map(|p| p.as_ptr()).unwrap_or(path);
    unsafe { libc::access(ptr, mode) }
}

// rename
interpose!(INTERPOSE_RENAME, my_rename, libc::rename);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_rename(old: *const c_char, new: *const c_char) -> c_int {
    let new_old = transform_cstring(old);
    let old_ptr = new_old.as_ref().map(|p| p.as_ptr()).unwrap_or(old);

    let new_new = transform_cstring(new);
    let new_ptr = new_new.as_ref().map(|p| p.as_ptr()).unwrap_or(new);

    unsafe { libc::rename(old_ptr, new_ptr) }
}
