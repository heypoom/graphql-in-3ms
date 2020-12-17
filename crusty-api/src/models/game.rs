use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Serialize, GraphQLObject)]
pub struct Crustacean {
  pub amount: i32,
  pub level: i32,
}
