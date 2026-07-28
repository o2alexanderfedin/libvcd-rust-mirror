#![allow(unused_imports, dead_code)]

mod src;
mod test;
use crate::test::test::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() {
        return 0;
    }
    return __r.unwrap_err();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SFILE {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type FILE = SFILE;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn calloc(__count: u64, __size: u64) -> *mut ();
    fn fscanf(_: *mut FILE, _: *const i8, ...) -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strchr(__s: *const i8, __c: i32) -> *mut i8;
    fn sscanf(_: *const i8, _: *const i8, ...) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn __builtin_object_size(_: *const (), _: i32) -> u64;
    fn __builtin___strncpy_chk(_: *mut i8, _: *const i8, _: u64, _: u64) -> *mut i8;
    fn fopen(__filename: *const i8, __mode: *const i8) -> *mut FILE;
    fn fgetc(_: *mut FILE) -> i32;
    fn ungetc(_: i32, _: *mut FILE) -> i32;
    fn isspace(_c: i32) -> i32;
    fn fclose(_: *mut FILE) -> i32;
    fn free(_: *mut ()) -> ();
    fn printf(_: *const i8, ...) -> i32;
    fn exit(_: i32) -> ();
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, ...) -> i32;
}
