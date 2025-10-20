// Stream
use trpl::{ReceiverStream, Stream, StreamExt};
use std::time::Duration;
use std::pin::pin;

fn main() {
    trpl::run( async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        
        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task( async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: {message}")).unwrap();
        }
    });

    ReceiverStream::new(rx)
}


// // Building Async Abstraction
// use std::time::Duration;
// use trpl::Either;
//
// fn main() {
//     trpl::run( async {
//         let slow = async {
//             trpl::sleep(Duration::from_secs(2)).await;
//             "I finished!"
//         };
//
//         match timeout(slow, Duration::from_secs(5)).await {
//             Ok(message) => println!("Succeeded with '{message}'"),
//             Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
//         }
//     })
// }
//
// async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
//     let timer = trpl::sleep(max_time);
//     match trpl::race(future_to_try, timer).await {
//         Either::Left(output) => Ok(output),
//         Either::Right(_) => Err(max_time),
//     }
// }


// // Yeild control to the runtime
// use std::thread;
// use std::time::Duration;
//
// fn slow(name: &str, ms: u64) {
//     thread::sleep(Duration::from_millis(ms));
//     println!("{name} ran for {ms}ms");
// }
//
// fn main() {
//     trpl::run( async {
//         let a = async {
//             println!("'a' started");
//             slow("a", 30);
//             trpl::yield_now().await;
//             slow("a", 10);
//             trpl::yield_now().await;
//             slow("a", 20);
//             trpl::yield_now().await;
//             println!("'a' finished");
//         };
//
//         let b = async {
//             println!("'b' started");
//             slow("b", 75);
//             trpl::yield_now().await;
//             slow("b", 10);
//             trpl::yield_now().await;
//             slow("b", 15);
//             trpl::yield_now().await;
//             slow("b", 350);
//             trpl::yield_now().await;
//             println!("'b' finished");
//         };
//
//         trpl::race(a, b).await;
//     })
// }

// // Any number of futures
// use std::time::Duration;
// use std::pin::{ Pin, pin };
//
// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();
//
//         let tx1 = tx.clone();
//         let tx_fut1 = pin!( async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];
//
//             for val in vals {
//                 tx1.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });
//
//         let tx_fut = pin!( async move {
//             let vals = vec![
//                 String::from("more"),
//                 String::from("messages"),
//                 String::from("for"),
//                 String::from("you"),
//             ];
//
//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });
//
//         let rx_fut = pin!( async {
//             while let Some(value) = rx.recv().await {
//                 println!("{value}");
//             }
//         });
//
//         let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
//             vec![tx_fut, tx_fut1, rx_fut];
//
//         trpl::join_all(futures).await;
//     })
// }


// // Concurrency with async
// use std::time::Duration;
//
// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();
//
//         let tx1 = tx.clone();
//         let tx_fut1 = async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];
//
//             for val in vals {
//                 tx1.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };
//
//         let tx_fut = async move {
//             let vals = vec![
//                 String::from("more"),
//                 String::from("messages"),
//                 String::from("for"),
//                 String::from("you"),
//             ];
//
//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };
//
//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("{value}");
//             }
//         };
//
//         trpl::join3(tx_fut, tx_fut1, rx_fut).await;
//     })
// }

// // Futures and async
// use trpl::Html;
// use std::env;
//
// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let response_text = trpl::get(url).await.text().await;
//     let title = Html::parse(&response_text)
//         .select_first("title")
//         .map(|title| title.inner_html());
//     
//     (url, title)
// }
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let url1 = &args[1];
//     let url2 = &args[2];
//
//     trpl::run(async {
//         let title_fut_1 = page_title(url1);
//         let title_fut_2 = page_title(url2);
//
//         let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
//             trpl::Either::Left(left) => left,
//             trpl::Either::Right(right) => right,
//         };
//
//         println!("{url} returned first");
//         match maybe_title {
//             Some(title) => println!("Its page title was: {title}"),
//             None => println!("It had no title"),
//         }
//     });
// }


// // Mutex
// use std::sync::{ Mutex, Arc };
// use std::thread;
//
// fn main() {
//     let counter = Arc::new(Mutex::new(5));
//     let mut handles = vec![];
//
//     for i in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             println!("number {i}");
//             *num += 1;
//         });
//
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("\nResult: {}", *counter.lock().unwrap());
// }

// // CHANNELS
// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];
//
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received in rx {
//         println!("Got: {received}");
//     }
// }
