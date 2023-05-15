use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{FromRef, Path}; // consider change of cargo toml to use axum-macros
use axum::routing::{delete, post};
use axum::Router;
use axum::{extract::State, Json};

//region : REST handlers
async fn create_ticket(
    State(model_controller): State<ModelController>, /* a trick, we use the modelcontrol as state  https://docs.rs/axum/latest/axum/#using-the-state-extractor*/
    Json(ticket_for_create): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("--> {:<12} create_ticket", "HANDLER");
    let ticket = model_controller.create_ticket(ticket_for_create).await?;
    Ok(Json(ticket))
}

//copilot again :-P he knows what i want
//TODO: we could implement filters for arguments here
async fn list_tickets(
    State(model_controller): State<ModelController>,
) -> Result<Json<Vec<Ticket>>> {
    println!("--> {:<12} list_tickets", "HANDLER");
    let tickets = model_controller.list_tickets().await?;
    Ok(Json(tickets))
}

//boilerplate from here on, since we have our stuff imlemented in the modelcontroller
async fn delete_ticket(
    State(model_controller): State<ModelController>,
    Path(ticket_id): Path<u64>, // from path to respect REST API
) -> Result<Json<Ticket>> {
    println!("--> {:<12} delete_ticket", "HANDLER");
    let ticket = model_controller.delete_ticket(ticket_id).await?;
    Ok(Json(ticket))
}
//endregion : REST handlers

//region : Router

//axum can combine states which we are not using yet, but as a glimpse
#[derive(Clone, FromRef)] // https://docs.rs/axum-macros/latest/axum_macros/derive.FromRef.html // either implement this themself, or alter cargo.toml to use axum-macros in feature
pub struct AppState {
    model_controller: ModelController, // is substate now
}

pub fn routes(model_controller: ModelController) -> Router {
    let app_state = AppState { model_controller }; // this enables us to use the state extractor for a certain module, or use even more states

    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:ticket_id", delete(delete_ticket))
        .with_state(app_state) // https://docs.rs/axum/latest/axum/struct.Router.html#method.with_state
}
//endregion : Router
