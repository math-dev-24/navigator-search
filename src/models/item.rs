use crate::models::navigator::Navigator;

#[derive(Debug)]
pub struct Item {
    pub title: String,
    pub url: String,
    pub domain: Navigator
}