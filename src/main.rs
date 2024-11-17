use std::{
    ops::Add, process::Output, sync::mpsc::{self, Receiver, Sender}, thread::{self, JoinHandle}, time::Duration
};

#[derive(Debug, Clone)]
enum Message {
    Ping,
    Pong
}

struct Actor {
    name: String,
    handle: Option<JoinHandle<()>>,
    tx: Option<Sender<Message>>
}

impl Actor {
    pub fn new(name: String) -> Self {
        println!("Actor {name} started");
        Self { name, handle: None, tx: None }
    }

    pub fn recv(&mut self) {
        let (tx, rx) = mpsc::channel::<Message>();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(result) => {
                        match result {
                            Message::Ping => {
                                println!("Ping");
                            },
                            Message::Pong => {
                                println!("Ponh");
                            },
                        }
                    },
                    Err(err) => {
                        println!("error: {:?}", err);
                    },
                }
            }
        });

        self.tx = Some(tx);
        self.handle = Some(handle);
    }

    fn send(&self, msg: Message) {
        let tx = self.tx.clone().unwrap();
        tx.send(msg).unwrap();
    }
}


fn main() {
    let mut actor = Actor::new(String::from("My Actor"));

    actor.recv();

    thread::sleep(Duration::from_secs(1));
    actor.send(Message::Ping);
    actor.send(Message::Pong);

    let handle = actor.handle.unwrap();
    handle.join();
}
