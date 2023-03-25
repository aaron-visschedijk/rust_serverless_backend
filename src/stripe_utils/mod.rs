use stripe::CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::OnSubscription;
use stripe::{
    CreateSubscriptionPaymentSettings, Customer, CustomerId, PriceId, StripeError, Subscription,
    SubscriptionPaymentBehavior,
};

use crate::dynamo::models::User;

pub async fn create_customer(
    stripe: &stripe::Client,
    user: &User,
) -> Result<CustomerId, StripeError> {
    let mut params = stripe::CreateCustomer::new();
    params.email = Some(&user.email);
    let customer = Customer::create(&stripe, params).await?;
    Ok(customer.id)
}

pub async fn create_subscription(
    stripe: &stripe::Client,
    user: &User,
    price_id: &PriceId,
) -> Result<Subscription, StripeError> {
    let stripe_id = user
        .stripe_id
        .clone()
        .expect("User does not have a stripe id");
    let mut params = stripe::CreateSubscription::new(stripe_id);
    params.items = Some(vec![stripe::CreateSubscriptionItems {
        price: Some(price_id.to_string()),
        ..Default::default()
    }]);
    params.payment_behavior = Some(SubscriptionPaymentBehavior::DefaultIncomplete);
    params.payment_settings = Some(CreateSubscriptionPaymentSettings {
        save_default_payment_method: Some(OnSubscription),
        payment_method_options: None,
        payment_method_types: None,
    });
    params.expand = &["latest_invoice.payment_intent"];
    let subscription = stripe::Subscription::create(&stripe, params).await?;

    Ok(subscription)
}
