/*******************************************************************************
 *     ___                  _   ____  ____
 *    / _ \ _   _  ___  ___| |_|  _ \| __ )
 *   | | | | | | |/ _ \/ __| __| | | |  _ \
 *   | |_| | |_| |  __/\__ \ |_| |_| | |_) |
 *    \__\_\\__,_|\___||___/\__|____/|____/
 *
 *  Copyright (c) 2014-2019 Appsicle
 *  Copyright (c)  2019-2025 QuestDB
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 ******************************************************************************/

#![doc = include_str!("../README.md")]
#![allow(clippy::missing_safety_doc)]

use questdb_confstr::{parse_conf_str, ConfStr};
use std::collections::hash_map;
use std::os::raw::c_char;
use std::ptr;
use std::slice;

#[repr(C)]
pub struct questdb_conf_str {
    inner: ConfStr,
}

#[repr(C)]
pub struct questdb_conf_str_parse_err {
    msg: *const c_char,
    msg_len: usize,
    pos: usize,
}

fn new_err(msg: String, pos: usize) -> *mut questdb_conf_str_parse_err {
    let msg_len = msg.len();
    let msg = Box::into_raw(msg.into_boxed_str()) as *const c_char;
    Box::into_raw(Box::new(questdb_conf_str_parse_err { msg, msg_len, pos }))
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_parse_err_free(err: *mut questdb_conf_str_parse_err) {
    if !err.is_null() {
        let err = Box::from_raw(err);
        drop(Box::from_raw(err.msg as *mut c_char));
        drop(err);
    }
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_parse(
    str: *const c_char,
    len: usize,
    err_out: *mut *mut questdb_conf_str_parse_err,
) -> *mut questdb_conf_str {
    let input = slice::from_raw_parts(str as *const u8, len);
    let input_str = match std::str::from_utf8(input) {
        Ok(s) => s,
        Err(utf8err) => {
            let first_bad_byte = utf8err.valid_up_to();
            *err_out = new_err(
                format!("invalid UTF-8 sequence at position {}", first_bad_byte),
                first_bad_byte,
            );
            return ptr::null_mut();
        }
    };

    match parse_conf_str(input_str) {
        Ok(conf_str) => Box::into_raw(Box::new(questdb_conf_str { inner: conf_str })),
        Err(err) => {
            *err_out = new_err(err.to_string(), err.position());
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_service(
    conf_str: *const questdb_conf_str,
    len_out: *mut usize,
) -> *const c_char {
    if conf_str.is_null() {
        return ptr::null();
    }

    let conf_str = &(*conf_str).inner;
    let service = conf_str.service();
    *len_out = service.len();
    service.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_get(
    conf_str: *const questdb_conf_str,
    key: *const c_char,
    key_len: usize,
    val_len_out: *mut usize,
) -> *const c_char {
    if conf_str.is_null() || key.is_null() {
        return ptr::null();
    }

    let conf_str = &(*conf_str).inner;
    let key = slice::from_raw_parts(key as *const u8, key_len);
    let key_str = match std::str::from_utf8(key) {
        Ok(s) => s,
        Err(_) => return ptr::null(),
    };

    match conf_str.get(key_str) {
        Some(val) => {
            let val_str = val.as_ptr() as *const c_char;
            *val_len_out = val.len();
            val_str
        }
        None => ptr::null(),
    }
}

#[repr(C)]
pub struct questdb_conf_str_iter {
    inner: hash_map::Iter<'static, String, String>,
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_iter_pairs(
    conf_str: *const questdb_conf_str,
) -> *mut questdb_conf_str_iter {
    if conf_str.is_null() {
        return ptr::null_mut();
    }
    let conf_str = &(*conf_str).inner;
    let iter = questdb_conf_str_iter {
        inner: conf_str.params().iter(),
    };
    Box::into_raw(Box::new(iter))
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_iter_next(
    iter: *mut questdb_conf_str_iter,
    key_out: *mut *const c_char,
    key_len_out: *mut usize,
    val_out: *mut *const c_char,
    val_len_out: *mut usize,
) -> bool {
    let iter = &mut *iter;
    match iter.inner.next() {
        Some((key, val)) => {
            let key_str = key.as_ptr() as *const c_char;
            let val_str = val.as_ptr() as *const c_char;
            unsafe {
                *key_out = key_str;
                *key_len_out = key.len();
                *val_out = val_str;
                *val_len_out = val.len();
            }
            true
        }
        None => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_iter_free(iter: *mut questdb_conf_str_iter) {
    if !iter.is_null() {
        drop(Box::from_raw(iter));
    }
}

#[no_mangle]
pub unsafe extern "C" fn questdb_conf_str_free(conf_str: *mut questdb_conf_str) {
    if !conf_str.is_null() {
        drop(Box::from_raw(conf_str));
    }
}
