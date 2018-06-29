# log4rs-ldp

`log4rs-ldp` - very simple TCP/Gelf appender for [OVH Log Data Platform](https://www.ovh.com/fr/data-platforms/logs), 
it uses [log4rs-gelf](https://github.com/cdumay/log4rs-gelf) to format log entries.

**Work in progress, for testing only !**

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
log4rs_ldp = "0.1"
```

Example code:

```rust,no_run
#[deny(warnings)]
extern crate log4rs;
extern crate log4rs_gelf;
extern crate log4rs_ldp;
extern crate serde_json;

#[macro_use]
extern crate log;

use log4rs::config::{Config, Appender, Root};
use log4rs_gelf::append::tcp::TCPAppender;
use log4rs_gelf::builder::Builder;
use log4rs_ldp::encoders::OvhGelfEncoderBuilder;
use log::LevelFilter;
use serde_json::Value;
use std::{thread, time};


fn main() {
    let gelf = OvhGelfEncoderBuilder::new()
        .ovh_token("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
        .null_character(true)
        .add_field("MyCustomField", Value::from("75874f9c-d4f9-45bd-a5fc-9a1ca201f70e"))
        .add_field("cpu", Value::from(15))
        .build().unwrap();

    let gelf_tcp_input = TCPAppender::builder()
        .encoder(Box::new(gelf))
        .host("127.0.0.1:12202")
        .max_cache_size(15000)
        .batch_size(1000)
        .build().unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("gelf_tcp", Box::new(gelf_tcp_input)))
        .build(Root::builder().appender("gelf_tcp").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
    for idx in 0..5000 {
        info!("Test {}", idx)
    }

    // We wait a little to exit...
    thread::sleep(time::Duration::from_secs(3));
}
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
