#[derive(Serialize, FromForm, Debug)]
pub struct PaymentForm {
    // pub amount: i32,
    pub amount: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    pub value: String,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestConfirmation {
    pub r#type: String,
    pub return_url: String,
}

#[derive(Serialize, Debug)]
pub struct PaymentBody {
    pub amount: Amount,
    pub capture: bool,
    pub confirmation: RequestConfirmation,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseConfirmation {
    pub r#type: String,
    pub confirmation_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipient {
    pub account_id: String,
    pub gateway_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentResponse {
    pub id: String,
    pub status: String,
    pub paid: bool,
    pub amount: Amount,
    pub confirmation: ResponseConfirmation,
    pub created_at: String,
    pub description: String,
    pub recipient: Recipient,
    pub refundable: bool,
    pub test: bool,
}
