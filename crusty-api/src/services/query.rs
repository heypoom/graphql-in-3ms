use juniper::graphql_object;
use crate::models::game::Crustacean;

pub struct Query;

#[graphql_object]
impl Query {
  pub async fn crabs() -> Crustacean {
    Crustacean {
      level: 50,
      amount: 50,
    }
  }

  pub async fn lobsters() -> Crustacean {
    Crustacean {
      level: 30,
      amount: 30,
    }
  }
}
