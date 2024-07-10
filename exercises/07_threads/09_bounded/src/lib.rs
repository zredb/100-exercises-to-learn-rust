// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use crossbeam_channel as channel;
use crossbeam_channel::{Receiver, Sender, SendError};

pub mod data;
pub mod store;


#[derive(Clone)]
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, SendError<TicketId>> {
        let (sender, receiver) = channel::bounded(1);
        let _ = self.sender.send(Command::Insert {
            draft,
            response_channel: sender,
        });
        Ok(receiver.recv().unwrap())
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, SendError<Option<&Ticket>>> {
        let (sender, receiver) = channel::bounded(1);
        let _ = self.sender.send(Command::Get {
            id,
            response_channel: sender,
        });
        Ok(receiver.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = channel::bounded(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sender,
    }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                   draft,
                   response_channel,
               }) => {
                let id = store.add_ticket(draft);
                response_channel.send(id).unwrap()
            }
            Ok(Command::Get {
                   id,
                   response_channel,
               }) => {
                let ticket = store.get(id);
                response_channel.send(ticket.cloned()).unwrap()
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
