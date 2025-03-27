use std::fmt::{Display, Formatter, Result};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_title: String,
    pub product_url: String,
    pub product_type: String,
    pub subscriber_name: String,
    pub status: String,
}

impl Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.status.to_uppercase().eq("CREATED") {
            write!(f,
                "Hello {}, let's try our new {} product: {}, only in BambangShop! Check it out: {}",
                self.subscriber_name, self.product_type.to_lowercase(), self.product_title, self.product_url,
            )
        } else if self.status.to_uppercase().eq("DELETED") {
            write!(f,
                "Hello {}, we are informing that our {} product named {} is already sold out...",
                self.subscriber_name, self.product_type.to_lowercase(), self.product_title,
            )
        } else {
            write!(f,
                "Hello {}, let's try our {} product: {}, grab it out before it runs out! Check it out: {}",
                self.subscriber_name, self.product_type.to_lowercase(), self.product_title, self.product_url,
            )
        }
    }
}
