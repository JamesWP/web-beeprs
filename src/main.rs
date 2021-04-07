use gpio::{GpioOut};
use warp::Filter;
use futures::{join};
use std::convert::Infallible;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::{channel, Sender, Receiver};

#[tokio::main]
async fn main() {
    let (beep_transmit, beep_reciever) = channel::<()>(6);

    join!(web_server(beep_transmit), beeper(beep_reciever));
}

async fn beeper(mut queue: Receiver<()>){
    println!("Hello, beeper!");

    let mut gpio4 = gpio::sysfs::SysFsGpioOutput::open(4).unwrap();

    let mut value = false;
    
    while let Some(_) = queue.recv().await {
        for _ in 1..100 {
            gpio4.set_value(value).expect("could not set gpio24");
            sleep(Duration::from_millis(10)).await;
            value = !value;
        }
    }
}

async fn web_server(queue: Sender<()>) {
    println!("Hello, web!");
    let routes = warp::any().map(move || queue.clone()).and_then(beep_page);
    
    warp::serve(routes).run(([0,0,0,0], 8080)).await;
}

async fn beep_page(queue: Sender<()>) -> Result<impl warp::Reply, Infallible> {
    queue.try_send(()).and_then(|()| Ok("Beeep!")).or(Ok("No, Beep you!"))
}