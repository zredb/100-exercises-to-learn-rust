// TODO: Implement the `to_dos` method. It must return a `Vec` of references to the tickets
//  in `TicketStore` with status set to `Status::ToDo`.
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl<'a> IntoIterator for &'a TicketStore{
    type Item = &'a Ticket;
    type IntoIter = std::slice::Iter<'a, Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
    
    pub fn to_dos(&self)->Vec<&Ticket>{
        let mut res=Vec::new();
         for x in self.into_iter() {
            if x.status==Status::ToDo{
                res.push(x)
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn todos() {
        let mut store = TicketStore::new();

        let todo = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(todo.clone());

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let todos: Vec<&Ticket> = store.to_dos();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], &todo);
    }
}
