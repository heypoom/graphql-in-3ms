use std::{env, pin::Pin, time::Duration};
use juniper::{futures, FieldError};
use juniper::graphql_object;
use juniper::graphql_subscription;

use crate::models::game::Crustacean;

use juniper::futures::StreamExt;

type CrustaceanStream =
    Pin<Box<dyn futures::Stream<
        Item = Result<Crustacean, FieldError>
    > + Send>>;


pub struct Subscription;

#[graphql_subscription]
impl Subscription {
    #[graphql(description = "Get the Crab's level in real-time.")]
    async fn crabs() -> CrustaceanStream {
        let mut level = 0;
        let mut amount = 0;

        let delay = Duration::from_millis(10);

        let stream = tokio::time::interval(delay)
            .map(move |_| {
                level += 1;
                amount += 1111;

                Ok(Crustacean { level, amount })
            });

        Box::pin(stream)
    }
}