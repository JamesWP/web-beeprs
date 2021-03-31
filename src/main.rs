use gpio::{GpioIn, GpioOut};
use std::{thread, time};
use tiny_http::{Server, Response};

fn main() {
    println!("Hello, world!");

    // Let's open GPIO23 and -24, e.g. on a Raspberry Pi 2.
    let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
    let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();

    // GPIO24 will be toggled every second in the background by a different thread
    let mut value = false;
    thread::spawn(move || loop {
        gpio24.set_value(value).expect("could not set gpio24");
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
    });


    thread::spawn(move || loop {
        let server = Server::http("0.0.0.0:8000").unwrap();
        for request in server.incoming_requests() {
            println!("received request! method: {:?}, url: {:?}, headers: {:?}",
                request.method(),
                request.url(),
                request.headers()
            );

            let response = Response::from_string("hello world");
            request.respond(response);
        }
    });


    // The main thread will simply display the current value of GPIO23 every 100ms.
    loop {
        println!("GPIO23: {:?}", gpio23.read_value().unwrap());
        thread::sleep(time::Duration::from_millis(100));
    }
}
