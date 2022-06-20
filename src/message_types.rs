use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;

#[derive(Clone)]
pub enum IsoMessageType {
    Authorization,
    AuthorizationResponse,
    AuthorizationAdvice,
    AuthorizationAdviceResponse,
    Financial,
    FinancialResponse,
    FinancialAdvice,
    FinancialAdviceResponse,
    Reversal,
    ReversalResponse,
    NetworkManagement,
    NetworkManagementResponse,
    TokenNotification,
    TokenNotificationResponse,
    TokenManagement,
    TokenManagementResponse,
}

impl IsoMessageType {
    fn value(&self) -> &str {
        match self {
            Self::Authorization => "0100",
            Self::AuthorizationResponse => "0110",
            Self::AuthorizationAdvice => "0120",
            Self::AuthorizationAdviceResponse => "0130",
            Self::Financial => "0200",
            Self::FinancialResponse => "0210",
            Self::FinancialAdvice => "0220",
            Self::FinancialAdviceResponse => "0230",
            Self::Reversal => "0302",
            Self::ReversalResponse => "0312",
            Self::NetworkManagement => "0420",
            Self::NetworkManagementResponse => "0430",
            Self::TokenNotification => "0620",
            Self::TokenNotificationResponse => "0630",
            Self::TokenManagement => "0800",
            Self::TokenManagementResponse => "0810",
        }
    }

    pub fn get_response_type(&self) -> Self {
        match self {
            Self::Authorization => Self::AuthorizationResponse,
            Self::AuthorizationAdvice => Self::AuthorizationAdviceResponse,
            Self::Financial => Self::FinancialResponse,
            Self::FinancialAdvice => Self::FinancialAdviceResponse,
            Self::Reversal => Self::ReversalResponse,
            Self::NetworkManagement => Self::NetworkManagementResponse,
            Self::TokenNotification => Self::TokenNotificationResponse,
            Self::TokenManagement => Self::TokenManagementResponse,
            _ => self.clone(),
        }
    }
}

impl fmt::Display for IsoMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self, self.value())
    }
}

lazy_static! {
    pub static ref ISO_MESSAGE_TYPE_MAP: HashMap<&'static str, IsoMessageType> = HashMap::from([
        ("0100", IsoMessageType::Authorization),
        ("0110", IsoMessageType::AuthorizationResponse),
        ("0120", IsoMessageType::AuthorizationAdvice),
        ("0130", IsoMessageType::AuthorizationAdviceResponse),
        ("0200", IsoMessageType::Financial),
        ("0210", IsoMessageType::FinancialResponse),
        ("0220", IsoMessageType::FinancialAdvice),
        ("0230", IsoMessageType::FinancialAdviceResponse),
        ("0302", IsoMessageType::TokenManagement),
        ("0312", IsoMessageType::TokenManagementResponse),
        ("0420", IsoMessageType::Reversal),
        ("0430", IsoMessageType::ReversalResponse),
        ("0620", IsoMessageType::TokenNotification),
        ("0630", IsoMessageType::TokenNotificationResponse),
        ("0800", IsoMessageType::NetworkManagement),
        ("0810", IsoMessageType::NetworkManagementResponse)
    ]);
}
