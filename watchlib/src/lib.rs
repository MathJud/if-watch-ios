//! Test iOS Linking of if-watch

use futures::StreamExt;
use if_watch::smol::IfWatcher;
use env_logger;

/// start the library & run the tests
#[no_mangle]
pub extern fn watchlib_start() {
    env_logger::init();

    std::thread::spawn(move || {
        smol::block_on(async {
            let mut set = IfWatcher::new().unwrap();
            loop {
                let event = set.select_next_some().await;
                println!("Got event {:?}", event);
            }
        });
    });
}
