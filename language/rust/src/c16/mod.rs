pub mod ex1_thread_usage;
pub mod ex2_join_threads;
pub mod ex5_closure_move;
pub mod ex7_mpsc;


pub mod ex8 {
    use std::sync::mpsc;
    use std::thread;

    pub fn first() {
        let (tx, rx) = mpsc::channel();


        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
}

// pub mod ex9 {
//     use std::sync::mpsc;
//     use std::thread;

//     fn main() {
//         let (tx, rx) = mpsc::channel();

//         thread::spawn(move || {
//             let val = String::from("hi");
//             tx.send(val).unwrap();
//             println!("val is {val}");
//         });

//         let received = rx.recv().unwrap();
//         println!("Got: {received}");
//     }
// }


pub mod ex10 {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn first() {
        let (tx, rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }
}


pub mod ex11 {
    use core::time;

    pub fn first() {
        use std::sync::mpsc;
        use std::thread;


        let (tx, rx) = mpsc::channel();


        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(time::Duration::from_millis(500));
            }
        });

        // let tx2 = tx.close();
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(time::Duration::from_secs(1));
            }
        });

        for mes in rx {
            println!("Recevied: {mes}");
        }

    }

}


pub mod ex12 {
    use std::sync::Mutex;

    pub fn first() {
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }

    println!("m = {m:?}");
    }

}


pub mod ex15 {
    use std::sync::{Arc, Mutex};
    use std::{thread, vec};


    pub fn first() {
        let counter = Arc::new(Mutex::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handler = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handlers.push(handler);
        }

        for handle in handlers {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}


/// trying `try_recv` method
pub mod myex1 {
    

    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub fn first() {
        let (tx, rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        loop {
            let value = rx.try_recv();

            match value {
                Ok(value) => println!("Got: {value}"),
                Err(value) => println!("No value received: {value}")
            }

            if t.is_finished() {
                t.join();
                break;
            }

            thread::sleep(Duration::from_millis(3000));
        }
    }
}


/// putting main thread in sleep.
/// and checking if we get all messages,
/// of some messages.
pub mod myex2 {
    use std::sync::mpsc;
    use std::thread;

    pub fn first() {
        let (tx, rx) = mpsc::channel();


        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
}
