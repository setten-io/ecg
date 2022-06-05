use crate::error::EcgResult;

pub trait Checkable {
    fn check(&mut self, http: &ureq::Agent) -> EcgResult<bool>;
}
