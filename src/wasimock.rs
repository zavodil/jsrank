#[no_mangle]
//pub extern "C" fn _tzset_js(_timezone: *const libc::c_long, _daylight: *const libc::c_int, _tzname: *const libc::c_char) {
pub extern "C" fn _tzset_js(_timezone: i32, _daylight: i32, _tzname: i32) {

}

#[no_mangle]
//pub extern "C" fn _localtime_js(_t: *const libc::time_t, _tm: *const libc::tm) {
pub extern "C" fn _localtime_js(_t: i32, _tm: i32) {
    
}

#[no_mangle]
pub extern "C" fn _emscripten_date_now() -> f64 {
    return 0.0;   
}

#[no_mangle]
//pub extern "C" fn __wasi_fd_write(_fd: i32, _iovs: i32, _iovs_len: usize, _result: *const libc::size_t) -> libc::size_t {
pub extern "C" fn __wasi_fd_write(_fd: i32, _iovs: i32, _iovs_len: usize, _result: i32) -> i32 {
    return 0;
}

#[no_mangle]
//pub extern "C" fn __wasi_fd_write(_fd: i32, _iovs: i32, _iovs_len: usize, _result: *const libc::size_t) -> libc::size_t {
pub extern "C" fn imported__wasi_fd_write(_fd: i32, _iovs: i32, _iovs_len: usize, _result: i32) -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn __wasi_fd_close(_fd: i32) -> i32 {
    return 0;
}

#[no_mangle]
//pub extern "C" fn __wasi_fd_seek(_fd: i32, _offset: i64, _whence: i32, _result: *const libc::size_t) -> i32 {
pub extern "C" fn __wasi_fd_seek(_fd: i32, _offset: i64, _whence: i32, _result: i32) -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn __wasi_proc_exit(_rval: i32) {

}

#[no_mangle]
//pub extern "C" fn __syscall_getcwd(_buf: i32, _size: libc::size_t) -> i32 {
pub extern "C" fn __syscall_getcwd(_buf: i32, _size: i32) -> i32 {
    return 0;
}

#[no_mangle]
//pub extern "C" fn __wasi_environ_sizes_get(_environ_count: libc::size_t, _environ_buf_size: libc::size_t) -> i32 {
pub extern "C" fn __wasi_environ_sizes_get(_environ_count: i32, _environ_buf_size: i32) -> i32 {
    return 0;
}

#[no_mangle]
//pub extern "C" fn __wasi_environ_get(_environ: *const libc::c_char, _environ_buf: *const libc::c_char) -> i32 {
pub extern "C" fn __wasi_environ_get(_environ: i32, _environ_buf: i32) -> i32 {
    return 0;
}

#[no_mangle]
//pub extern "C" fn __wasi_random_get(_buf: i32, _size: libc::size_t) -> i32 {
pub extern "C" fn __wasi_random_get(_buf: i32, _size: i32) -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn _timegm_js(_p1 :i32) -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn _mktime_js(_p1 :i32) -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn _gmtime_js(_p1 :i32, _p2 :i32) {
}

#[no_mangle]
pub extern "C" fn _emscripten_get_now_is_monotonic() -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn emscripten_get_now_res() -> f64 {
    return 0.0;
}

#[no_mangle]
pub extern "C" fn __wasi_fd_fdstat_get(_p1: i32, _p2: i32) -> i32 {
    return 0;
}

#[no_mangle]
pub extern "C" fn emscripten_asm_const_async_on_main_thread(_p1: i32, _p2: i32, _p3: i32) {

}

#[no_mangle]
pub extern "C" fn __wasi_clock_res_get(_p1: i32, _p2: i32) -> i32 {
    return 0;
}