#[macro_use]

extern crate log;
extern crate rustc_serialize;
extern crate libc;
extern crate time;

use rustc_serialize::json;
use log::{LogRecord, LogLevel, LogMetadata, SetLoggerError, LogLevelFilter};
use libc::{c_char, c_int, size_t};
use std::str;
use std::ffi::CStr;
use std::io::prelude::*;
use std::net::TcpStream;
//use std::collections::HashMap;

extern {
  pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

struct SimpleLogger;

#[derive(RustcEncodable)]
pub struct GelfLogRecord {
    version: String,
    host: String,
    short_message: String,
    full_message: String,
    timestamp: i64,
    level: u32,
    facility: String,
    line: u32,
    file: String,
    //_additional: HashMap<String, String>
}

fn hostname() -> Result<String, &'static str> {
    let v = [0i8; 256];
    match unsafe { gethostname(v.as_ptr() as *mut c_char, 256) } {
        0 => {
            Ok(str::from_utf8(unsafe { CStr::from_ptr(v.as_ptr()).to_bytes() }).
               unwrap().to_string())
        },
        _ => Err("gethostname() failed")
    }
}

impl GelfLogRecord {
        fn new(record: &LogRecord) -> GelfLogRecord {
        let level = match record.level() {
            log::LogLevel::Error => 3,
            log::LogLevel::Warn  => 4,
            log::LogLevel::Info  => 6,
            log::LogLevel::Debug => 7,
            log::LogLevel::Trace => 7,
        };
        //let now = time::now().to_timespec();
        //let timestamp = now.sec * 1000;
        let timestamp: i64 = 0;

        GelfLogRecord { 
            version         : "1.0".to_string(), 
            facility        : record.target().to_string(),
            host            : hostname().unwrap(), 
            short_message   : record.args().to_string(), 
            full_message    : record.args().to_string(), 
            timestamp       : timestamp, 
            level           : level, 
            line            : record.location().line(),
            file            : record.location().file().to_string()
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(SimpleLogger)
    })
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let gelfrec = GelfLogRecord::new(record);
            if let Err(e) = self.send(&gelfrec) {
                println!("err: {}", e)
            }
        }
    }
}

impl SimpleLogger {
    fn send(&self, record: &GelfLogRecord) -> Result<usize, &'static str> {
        let mut socket = TcpStream::connect("thot.spinoff.ovh.net:12201").unwrap();
        let mut jdata = json::encode(record).unwrap();
        jdata.push('\0');

        match socket.write(jdata.as_bytes()) {
            Ok(n)   => Ok(n),
            _       => Err("Failed to write in socket")
        }
    }
}
