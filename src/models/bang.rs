
#[derive(Debug, PartialEq, Clone)]
pub enum BangMode {
    Filter,
    Redirect
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bang {
    pub name: String,
    pub mode : BangMode,
    pub url: String,
}