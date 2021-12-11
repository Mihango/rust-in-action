#![allow(unused)]

fn main() {
    let base = GroundStation {};
    let sat_ids = find_sat_ids();

    for sat_id in sat_ids {
        let mut sat = base.connect(sat_id);

        println!("t0: {:?}", sat);
        let mut mailbox = Mailbox::new();

        base.send(&mut mailbox, Message::new(sat_id, String::from("Hello Sat")));
        println!("t1: {:?}", sat);

        let msg = sat.recv(&mut mailbox);

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
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {

    fn new() -> Mailbox {
        Mailbox {
            messages: vec![]
        }
    }

    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(
        &mut self,
        recipient: &CubeSat
    ) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

impl Message {
    fn new(to: u64, msg: String) -> Message  {
        Message {
            to: to,
            content: msg,
        }
    }
}

// type Message = String;

struct GroundStation;

impl CubeSat {
    fn new(id: u64) -> CubeSat {
        CubeSat {
            id: id,
        }
    }

    // using references to solve ownership issue
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }
}
