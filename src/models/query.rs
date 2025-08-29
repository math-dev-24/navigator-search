use crate::models::bang::Bang;

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub text: String,
    pub bang: Option<Bang>
}