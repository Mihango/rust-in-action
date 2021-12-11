#![allow(unused)]

fn main() {
    let base = GroundStation {};
    let sat_ids = find_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);
        base.send(&mut sat, Message::from("Hello Sat"));

        println!("t0: {:?}", sat);

        base.send(&mut sat, Message::from("Hello there!"));
        println!("t1: {:?}", sat);
    
        let msg = sat.recv();
    
        println!("t2: {:?}", sat);
    
        println!("msg: {:?}", msg);
    }
}

// acts like a db access of sat ids
fn find_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat {
            id: id,
            mailbox: Mailbox {
                messages: vec![],
            },
        }
    }

    // using references to solve ownership issue
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }
}

