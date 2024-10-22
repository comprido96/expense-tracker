use axum::extract::Path;
use axum::routing::{delete, get};
use axum::Router;
use axum::{extract::State, Json};

use crate::error::{Error, Result};
use crate::model::{Expense, ExpenseForCreate, ModelController};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
    .route("/expenses", get(list_expenses).post(create_expense))
    .route("/expenses/:id", delete(delete_expense))
    .with_state(mc)
}

async fn create_expense(
    State(mc): State<ModelController>,
    Json(expense_for_create): Json<ExpenseForCreate>,
) -> Result<Json<Expense>> {
    println!("->> {:<12} create_expense", "HANDLER");

    let expense = mc.create_expense(expense_for_create).await?;

    Ok(Json(expense))
}

async fn list_expenses(
    State(mc): State<ModelController>,
) -> Result<Json<Vec<Expense>>> {
    println!("->> {:<12} list_expenses", "HANDLER");

    let expenses = mc.list_expenses().await?;

    Ok(Json(expenses))
}

async fn delete_expense(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Expense>> {
    println!("->> {:<12} delete_expense", "HANDLER");

    let expense = mc.delete_expense(id).await?;

    Ok(Json(expense))
}
