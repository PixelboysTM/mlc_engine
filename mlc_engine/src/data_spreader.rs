use std::{collections::HashMap, hash::Hash, sync::Arc};

use crossbeam::{
    channel::{Receiver, Sender, TryRecvError},
    queue::SegQueue,
};
use rocket::tokio::sync::Mutex;

use crate::side_worker::Work;

struct DataHandlerI<D, I> {
    subscriber: HashMap<I, SegQueue<D>>,
    rx: Receiver<D>,
}

pub struct DataSubscriber<D, I> {
    inner: Arc<Mutex<DataHandlerI<D, I>>>,
}

pub struct DataClient<D, I> {
    inner: Arc<Mutex<DataHandlerI<D, I>>>,
    id: I,
}

impl<D, I: Identifier + Eq + Hash + Clone> DataClient<D, I> {
    pub async fn recv(&self) -> Option<D> {
        let d = self.inner.lock().await;
        d.subscriber[&self.id].pop()
    }
}

impl<D, I: Identifier + Eq + Hash + Clone> DataSubscriber<D, I> {
    pub async fn subscribe(&self) -> DataClient<D, I> {
        let id = {
            let mut d = self.inner.lock().await;
            let id = I::new_id();
            d.subscriber.insert(id.clone(), SegQueue::new());
            id
        };

        DataClient {
            inner: self.inner.clone(),
            id,
        }
    }
}

pub struct DataSender<D> {
    tx: Sender<D>,
}

impl<D> DataSender<D> {
    pub fn send(&self, data: D) {
        self.tx.send(data).unwrap()
    }
}

pub struct DataHandlerWorker<D, I> {
    sub: Arc<Mutex<DataHandlerI<D, I>>>,
}

pub fn create<D, I: Identifier>() -> (DataHandlerWorker<D, I>, DataSender<D>, DataSubscriber<D, I>)
{
    let (tx, rx) = crossbeam::channel::unbounded();
    let i = Arc::new(Mutex::new(DataHandlerI {
        subscriber: HashMap::new(),
        rx,
    }));

    (
        DataHandlerWorker { sub: i.clone() },
        DataSender { tx },
        DataSubscriber { inner: i },
    )
}

impl<D, I> Work for DataHandlerWorker<D, I>
where
    D: Send + Clone,
    I: Send,
{
    fn run(&mut self) -> bool {
        println!("Outer run");
        pollster::block_on(async {
            let mut w = self.sub.lock().await;
            w.run()
        })
    }
}

impl<D, I> DataHandlerI<D, I>
where
    D: Send + Clone,
    I: Send,
{
    fn run(&mut self) -> bool {
        println!("Running");
        match self.rx.try_recv() {
            Ok(data) => {
                for (_, queue) in &self.subscriber {
                    queue.push(data.clone());
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(_) => {
                println!("Disconected");
                return false;
            }
        }

        true
    }
}

pub trait Identifier {
    fn new_id() -> Self;
}

impl Identifier for uuid::Uuid {
    fn new_id() -> Self {
        uuid::Uuid::new_v4()
    }
}
