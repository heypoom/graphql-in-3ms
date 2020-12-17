use crate::services::query::Query;

use juniper::{EmptyMutation, EmptySubscription, RootNode};
use crate::services::subscriptions::Subscription;

pub type Schema = RootNode<'static, Query, EmptyMutation, Subscription>;

pub fn create_schema() -> Schema {
  Schema::new(Query {}, EmptyMutation::new(), Subscription {})
}
