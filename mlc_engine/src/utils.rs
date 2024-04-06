use rocket::{Data, Request};
use rocket::fairing::{Fairing, Info, Kind};

#[macro_export]
macro_rules! send {
    ($info:expr, $msg:expr) => {
        let _ = $info.send($msg);
    };
}

pub struct BrowserGuard;

#[rocket::async_trait]
impl Fairing for BrowserGuard {
    fn info(&self) -> Info {
        Info {
            name: "Browser Guard",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let hs = req.headers();
        let agents = hs.get("User-Agent");
        for agent in agents {
            println!("Agent: {}", agent);
        }
    }
}

