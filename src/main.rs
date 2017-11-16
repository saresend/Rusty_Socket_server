
extern crate ws;
#[macro_use]
extern crate lazy_static;

use ws::*;
use std::sync::Mutex;
use std::ops::Deref;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::Cell;

struct Server {
    key: u32,
}

lazy_static!{
    static ref THREAD_POOL: Mutex<HashMap<u32, Sender>> = Mutex::new(HashMap::new());
}



impl Server {
    fn new(out: Sender, counter: Rc<Cell<u32>>) -> Server {

        let id = counter.get();
        THREAD_POOL.lock().unwrap().insert(id, out);
        counter.set(id + 1);
        Server { key: id,
        }
    }
}
impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("New Connection made!");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let mut counter = 0;
        for i in THREAD_POOL.lock().unwrap().deref() {
            counter += 1;
            let _ = i.1.send(msg.clone());
        }
        println!("Size of the Thread Pool {}", counter);
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, reason: &str) {
        println!("Closing Thread {} : {}", self.key, reason);
        THREAD_POOL.lock().unwrap().remove(&self.key);
    }
}
fn main() {
    let count = Rc::new(Cell::new(0));
    println!("Server starting on port: 3012");
    listen("0.0.0.0:3012", |out| Server::new(out, count.clone())).unwrap();

}
