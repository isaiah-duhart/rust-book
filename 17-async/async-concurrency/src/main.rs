// use std::time::Duration;

use std::time::Duration;
// use std::pin::{pin, Pin};

use trpl::{self, Either};

fn main() {
    // trpl::run( async {
    //     let fut_1= trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("Message {i} from task 1");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
        
    //     let fut_2 = trpl::spawn_task(async {
    //         for i in 1..5 {
    //             println!("Message {i} from task 2");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         } 
    //     });

    //    trpl::join(fut_1, fut_2).await;
    // })

    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();


    //     let tx_fut = pin!(async move {
    //         let vals = vec![
    //             String::from("Hello"),
    //             String::from("From"),
    //             String::from("The"),
    //             String::from("Channel")
    //         ];

    //         for val in vals {
    //             tx.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     let rx_fut = pin!(async {
    //         while let Some(mesg) = rx.recv().await {
    //             println!("{mesg}");
    //         }
    //     });

    //     let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, rx_fut];

    //     trpl::join_all(futures).await;
    // });
    trpl::run(async {let slow = async {
            trpl::sleep(Duration::from_millis(1)).await;
            return "I finished";
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(message) => println!("Succeeded with message '{message}'"),
            Err(duration) => println!("Failed after {} seconds", duration.as_secs())
        };
    });
}

async fn timeout<F: Future>(f: F, timeout: Duration) -> Result<F::Output, Duration> {
    match trpl::race(f, trpl::sleep(timeout)).await {
        Either::Left(result) => Ok(result),
        Either::Right(_) => Err(timeout)
    }
}