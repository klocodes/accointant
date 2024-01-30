use serde::{Deserialize, Serialize};
use crate::errors::Error;
use crate::errors::server::ServerErrors::InternalServerError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    USD,
    EUR,
    KZT,
    RUB,
    GEL,
}

impl Currency {
    pub fn new(currency: &str) -> Result<Self, Error> {
        match currency {
            "USD" => Ok(Self::USD),
            "EUR" => Ok(Self::EUR),
            "KZT" => Ok(Self::KZT),
            "RUB" => Ok(Self::RUB),
            "GEL" => Ok(Self::GEL),
            _ => Err(
                Error::Server(
                    InternalServerError {
                        context: Some(
                            format!("Unknown currency: {}", currency).into()
                        )
                    }
                )
            ),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::KZT => "KZT",
            Self::RUB => "RUB",
            Self::GEL => "GEL",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::RUB => "₽",
            Currency::KZT => "₸",
            Currency::GEL => "₾",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Currency::USD => "Доллар",
            Currency::EUR => "Евро",
            Currency::RUB => "Рубль",
            Currency::KZT => "Тенге",
            Currency::GEL => "Лари",
        }
    }
}
