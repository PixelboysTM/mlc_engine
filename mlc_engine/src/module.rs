use rocket::{Build, Rocket};

pub trait Module {
    fn setup(&self, app: Rocket<Build>) -> Rocket<Build>;
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
        for m in self.modules {
            rocket = m.setup(rocket);
        }

        rocket
    }
}
