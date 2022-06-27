use crate::error::EcgResult;

pub trait Check {
    fn check(&mut self, http: &ureq::Agent) -> EcgResult<bool>;
}
