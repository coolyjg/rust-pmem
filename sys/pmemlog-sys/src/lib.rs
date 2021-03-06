//! # FFI bindings to **libpmemlog**
//!
//! The **libpmemlog** library provides a pmem-resident log file.
//! This is useful for programs like databases that append frequently to a log file.
//!
//! > This is **not** an official port of the NVM Library.
//! >
//! > The official **libpmemlog** documentation can be found at: [http://pmem.io/nvml/libpmemlog/](http://pmem.io/nvml/libpmemlog/)

extern crate libc;

use ::libc::iovec;
use ::libc::{size_t, mode_t};
use ::libc::{c_void, c_char, c_int, c_longlong};


pub enum PMEMlogpool {}


#[allow(dead_code)]
#[link(name = "pmemlog")]
extern "C" {
    pub fn pmemlog_open(path: *const c_char) -> *mut PMEMlogpool;
    pub fn pmemlog_create(path: *const c_char, poolsize: size_t, mode: mode_t) -> *mut PMEMlogpool;
    pub fn pmemlog_close(plp: *mut PMEMlogpool);
    pub fn pmemlog_nbyte(plp: *mut PMEMlogpool) -> size_t;
    pub fn pmemlog_append(plp: *mut PMEMlogpool, buf: *const c_void, count: size_t) -> c_int;
    pub fn pmemlog_appendv(plp: *mut PMEMlogpool, iov: *const iovec, iovcnt: c_int) -> c_int;
    pub fn pmemlog_tell(plp: *mut PMEMlogpool) -> c_longlong;
    pub fn pmemlog_rewind(plp: *mut PMEMlogpool);
    pub fn pmemlog_walk(plp: *mut PMEMlogpool,
                        chunksize: size_t,
                        process_chunk: extern "C" fn(buf: *const c_void, len: size_t, arg: *mut c_void)
                                                     -> c_int,
                        // int (*process_chunk)(const void *buf, size_t len, void *arg),
                        arg: *mut c_void);
}
