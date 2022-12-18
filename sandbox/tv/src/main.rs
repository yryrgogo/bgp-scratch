use std::{thread, time::Duration};

use crate::tv::{push_random_button_of_tv, TV};

mod tv;

fn main() {
    let mut tv = TV::new();
    tv.be_pushed_power_button();
    loop {
        push_random_button_of_tv(&mut tv);
        if let Some(event) = tv.event_queue.dequeue() {
            println!(
                "tv information: {{ now_state={:?}, volume={} }}\n \
            input_event: {:?}",
                tv.now_state, tv.volume, event
            );
            tv.handle_event(event);
        }
        thread::sleep(Duration::from_secs(2));
    }
}
