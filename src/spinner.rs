use crossterm::{
    ExecutableCommand,
    cursor::{RestorePosition, SavePosition},
};
use spinners::{Spinner, Spinners};
use std::io;
use std::sync::OnceLock;
use ureq::Agent;

static AGENT: OnceLock<Agent> = OnceLock::new();

pub(crate) fn agent() -> Agent {
    AGENT.get_or_init(|| Agent::config_builder().middleware(spinner_middleware).build().into()).clone()
}

fn spinner_middleware(
    req: ureq::http::Request<ureq::SendBody>,
    next: ureq::middleware::MiddlewareNext,
) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
    let mut sp = if io::stdout().execute(SavePosition).is_ok() {
        Some(Spinner::new(Spinners::Arc, String::new()))
    } else {
        None
    };
    let res = next.handle(req);
    if let Some(mut s) = sp.take() {
        s.stop();
        let _ = io::stdout().execute(RestorePosition);
    }
    res
}
