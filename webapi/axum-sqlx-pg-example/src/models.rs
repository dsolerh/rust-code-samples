//! Simplistic Model Layer
//! (with mock-store layer)

use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

// region:    --- Ticket Types

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketCreateInfo {
    pub title: String,
}

// endregion: --- Ticket Types

// region:    --- Model controller

#[derive(Debug, Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD operations
impl ModelController {
    pub async fn create_ticket(&self, ticket_info: TicketCreateInfo) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_info.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let items = store.iter().filter_map(|t| t.clone()).collect();

        Ok(items)
    }

    pub async fn delete_ticket(&self, ticket_id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(ticket_id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailedIdNotFound { id: ticket_id })
    }
}

// endregion: --- Model controller
