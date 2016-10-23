extern crate openzwave;

use openzwave::options::Options;
use openzwave::manager::{Manager, NotificationWatcher};
use openzwave::notification::Notification;

fn main() {
    let options = Options::create("config", "", "").expect("Cannot create options");
    let mut manager = Manager::create(options).expect("Cannot create manager");
    manager.add_watcher(DomusWatcher).expect("Cannot add watcher");
    manager.add_driver("/dev/ttyACM0").expect("Cannot add driver");

    // Driver ready notification should happen
    // AllAwakeNodesQueried
    // AllNodesQueried

    let tenth_of_sec = std::time::Duration::from_millis(100);   
    unsafe {
        signal(2, start_shutdown);

        while !shutdown_started{
            std::thread::sleep(tenth_of_sec);
        }
    }
}

struct DomusWatcher;

impl NotificationWatcher for DomusWatcher {
    fn on_notification(&self, notification: &Notification) {
        println!("{:?}", notification);
    }
}

extern "C" {
    fn signal(sig: u32, cb: extern fn(u32)) -> extern fn(u32);
}

static mut shutdown_started: bool = false;

extern fn start_shutdown(_:u32) {
    unsafe {
        shutdown_started = true;
    }
}

 
