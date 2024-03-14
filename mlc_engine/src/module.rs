use rocket::{Build, Rocket};
use rocket_okapi::okapi::openapi3::OpenApi;
use rocket_okapi::settings::OpenApiSettings;

pub trait Module {
    fn setup(&self, app: Rocket<Build>, spec: &mut OpenApi) -> Rocket<Build>;
}

pub struct Application {
    modules: Vec<Box<dyn Module>>,
}

impl Application {
    pub fn create() -> Application {
        Application { modules: vec![] }
    }

    pub fn mount<M>(mut self, module: M) -> Self
        where
            M: Module + 'static,
    {
        self.modules.push(Box::new(module));
        self
    }

    pub fn launch(self) -> Rocket<Build> {
        let mut rocket = rocket::build();

        let mut spec = OpenApi::new();

        for m in self.modules {
            rocket = m.setup(rocket, &mut spec);
        }

        rocket.mount("/", vec![rocket_okapi::handlers::OpenApiHandler::new(spec)
            .into_route(&OpenApiSettings::new().json_path)])
    }
}
