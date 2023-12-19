use std::sync::Arc;

use rocket::{futures::lock::Mutex, tokio::task::JoinHandle};

pub trait Work: Send {
    fn run(&mut self) -> bool;
}

struct SideWorkerI {
    // Returns true if keep alive
    jobs: Vec<Box<dyn Work>>,
    started: Option<JoinHandle<()>>,
}

#[derive(Clone)]
pub struct SideWorker {
    inner: Arc<Mutex<SideWorkerI>>,
}

impl SideWorker {
    async fn spin_up_if_needed(&self) {
        let mut guard = self.inner.lock().await;
        if guard.started.is_none() {
            let worker = self.clone();
            let j = rocket::tokio::spawn(async move {
                let w = worker;
                'main: loop {
                    let mut data = w.inner.lock().await;
                    let mut new_jobs = Vec::new();
                    while let Some(mut job) = data.jobs.pop() {
                        if job.run() {
                            new_jobs.push(job);
                        }
                    }
                    data.jobs = new_jobs;

                    if data.jobs.is_empty() {
                        data.started = None;
                        println!("Stopping worker");
                        break 'main;
                    }
                }

                println!("Exiting");
            });

            guard.started = Some(j);
        }
    }

    pub async fn queue_job(&self, job: Box<dyn Work>) {
        {
            let mut guard = self.inner.lock().await;
            guard.jobs.push(job);
        }
        self.spin_up_if_needed().await;
    }
}

pub fn create_side_worker() -> SideWorker {
    SideWorker {
        inner: Arc::new(Mutex::new(SideWorkerI {
            jobs: vec![],
            started: None,
        })),
    }
}
