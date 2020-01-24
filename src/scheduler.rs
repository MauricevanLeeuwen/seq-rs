use std::{thread, time};

fn control() {
    let t = time::Duration::from_millis(1); /* for smaller granularity you should spin, not sleep */
    thread::sleep(t);
}
