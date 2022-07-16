use crate::error::Result;

pub trait Checker {
    fn check(&mut self, http: &ureq::Agent) -> Result<bool>;
}
