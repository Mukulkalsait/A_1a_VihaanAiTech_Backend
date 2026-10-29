use crate::{errors::ApiError, modals::workshop::*};

pub mod open_workshop_handler;
pub mod workshop_handler;
pub mod workshop_registrations_handler;

impl Workshop {
    pub fn is_open(&self) -> bool { self.status == WorkshopStatus::Open }
    pub fn is_visible_to_public(&self) -> bool { matches!(self.status, WorkshopStatus::Open | WorkshopStatus::Closed) }
    pub fn display_price(&self) -> String {
        let symbol = match self.currency.as_str() {
            "INR" => "󰆯",
            "USD" => "",
            "EUR" => "",
            other => other,
        };
        if self.amount.fract() == 0.0 { format!("{}{}", symbol, self.amount as i64) } else { format!("{}{:.2}", symbol, self.amount) }
    }
    pub fn has_capacity(&self, current_count: i64) -> bool {
        match self.capacity {
            None => true,
            Some(cap) => current_count < cap as i64,
        }
    }
    pub fn parse_status(s: &str) -> Result<WorkshopStatus, ApiError> {
        match s {
            "draft" => Ok(WorkshopStatus::Draft),
            "open" => Ok(WorkshopStatus::Open),
            "closed" => Ok(WorkshopStatus::Closed),
            "cancelled" => Ok(WorkshopStatus::Cancelled),
            "archived" => Ok(WorkshopStatus::Archived),
            other => Err(ApiError::BadRequest(format!("invalid status:{}", other))),
        }
    }
}
