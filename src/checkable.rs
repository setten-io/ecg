use crate::error::Result;

pub trait Checkable {
    fn check(&mut self, http: &ureq::Agent) -> Result<bool>;
}
