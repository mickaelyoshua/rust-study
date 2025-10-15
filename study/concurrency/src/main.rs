use trpl::Html;
use std::env;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    // let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];

    trpl::run(async {
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    });
}


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

// use std::thread;
// use std::time::Duration;
// use std::sync::mpsc;

// // CHANNELS
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
