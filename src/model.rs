/*
  architecture schema:
    -----------
    |WEB (IPC)|
    -----------
  |Context| |Event|
  --------  ------
      |Model| <- Simplistic Model Layer + with Mock-Store Layer // not to use in production
      -------
      |Store|
      -------
*/

use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex}; // in memory store

// region: Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64, //creator_user_id
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

//TODO: update, delete ... e.g
// pub struct TicketForUpdate {
//     pub id: u64,
//     pub title: String,
// }

// endregion: Ticket Types

// region: Model Controller
#[derive(Clone)]
pub struct ModelController {
    ticket_store: Arc<Mutex<Vec<Option<Ticket>>>>, // :D do not use in production since it could grow infinitely
}

//Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ticket_store: Arc::default(),
        })
    }
}

//copilot is actually pretty smart on this one
impl ModelController {
    pub async fn create_ticket(
        &self,
        ctx: Ctx,
        ticket_for_create: TicketForCreate,
    ) -> Result<Ticket> {
        let mut ticket_store = self.ticket_store.lock().unwrap(); // handle mutex lock
        let id = ticket_store.len() as u64; // little hacky since index +1 only works since rust has exclusive ownership of the mutex
        let ticket = Ticket {
            id,
            cid: ctx.user_id,
            title: ticket_for_create.title,
        };
        ticket_store.push(Some(ticket.clone())); // push as option to the store as clone
        Ok(ticket)
    }

    pub async fn list_tickets(&self, ctx: Ctx) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().unwrap();
        let tickets = store
            .iter()
            .filter_map(|ticket| ticket.clone()) // filter out None
            .collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut store = self.ticket_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound {
            id, /*condensation */
        })
    }
}
// --end Constructor
// endregion: Model Controller
