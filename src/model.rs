use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Clone, Debug, Serialize)]
pub struct Expense {
    pub id: u64,
    pub submitted_at: DateTime<Utc>,
    pub payment_date: DateTime<Utc>,
    pub description: String,
    pub amount: u64,
    pub is_recurrent: bool,
}

#[derive(Deserialize)]
pub struct ExpenseForCreate {
    pub payment_date: DateTime<Utc>,
    pub description: String,
    pub amount: u64,
    pub is_recurrent: bool,
}

#[derive(Clone)]
pub struct ModelController {
    expenses_store: Arc<Mutex<Vec<Option<Expense>>>>
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            expenses_store: Arc::default()
        })
    }
}

impl ModelController {
    pub async fn create_expense(&self, expense: ExpenseForCreate) -> Result<Expense> {
        let mut store = self.expenses_store.lock().unwrap();

        let id = store.len() as u64;

        let expense = Expense {
            id,
            submitted_at: Utc::now(),
            payment_date: expense.payment_date,
            description: expense.description,
            amount: expense.amount,
            is_recurrent: expense.is_recurrent,
        };

        store.push(Some(expense.clone()));

        Ok(expense)
    }

    pub async fn list_expenses(&self) -> Result<Vec<Expense>> {
        let store = self.expenses_store.lock().unwrap();

        let expenses = store.iter().filter_map(|e| e.clone()).collect();

        Ok(expenses)
    }

    pub async fn delete_expense(&self, id: u64) -> Result<Expense> {
        let mut store = self.expenses_store.lock().unwrap();

        let expense = store.get_mut(id as usize).and_then(|e| e.take());

        expense.ok_or(Error::ExpenseDeleteFailIdNotFound { id: id })
    }
}