//! Simplistic Model Layer
//! (with mock-store layer)

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
	pub id: u64,
	pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
	pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
	// sqlx store would come in here
	// only for mocking, since this vector will grow indefinitely
	tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
	//important here to control the constructor. That makes it easy to swap the implementation out later.
	pub async fn new() -> Result<Self> {
		Ok(Self {
			tickets_store: Arc::default(),
		})
	}
}

impl ModelController {
	pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
		let mut store = self.tickets_store.lock().unwrap();

		let id = store.len() as u64;
		let ticket = Ticket {
			id,
			title: ticket_fc.title,
		};
		store.push(Some(ticket.clone()));
		Ok(ticket)
	}

	pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
		let store = self.tickets_store.lock().unwrap();

		let tickets = store
			.iter()
			.filter_map(|t| t.clone()) // yields only Some-entries
			.collect();

		Ok(tickets)
	}

	pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
		let mut store = self.tickets_store.lock().unwrap();

		//Option.take returns the value of a Some and returns it while replacing the Option with a None
		let ticket = store.get_mut(id as usize).and_then(|t| t.take());

		ticket.ok_or(Error::TicketDeleteFailedIdNotFound { id })
	}
}

//CRUD Implementation
