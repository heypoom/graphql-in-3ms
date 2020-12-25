use juniper::GraphQLObject;
use serde::Serialize;

#[derive(Serialize, GraphQLObject)]
pub struct Crustacean {
  #[graphql(description = "Amount of crustaceans")]
  pub amount: i32,

  #[graphql(description = "The level of crustaceans")]
  pub level: i32,
}
