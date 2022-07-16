use crate::error::EcgResult;

pub trait Checker {
    fn check(&mut self, http: &ureq::Agent) -> EcgResult<bool>;
}
