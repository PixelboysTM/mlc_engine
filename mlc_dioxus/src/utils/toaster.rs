use std::collections::{BTreeMap};
use std::time::Duration;
use dioxus::prelude::*;
use crate::icons;

pub struct Toaster {
    toasts: BTreeMap<usize, Toast>,
    id: id::ID,
}

impl Toaster {
    fn new() -> Self {
        Self {
            toasts: BTreeMap::new(),
            id: id::ID::new(),
        }
    }
    fn toasts(&self) -> Vec<Toast> {
        self.toasts.values().cloned().collect()
    }

    fn remove(&mut self, id: usize) {
        self.toasts.remove(&id);
    }

    pub fn create(&mut self, title: String, msg: String, level: ToastLevel, delay: i64) -> usize {
        let t = Toast {
            title,
            msg,
            level,
            id: self.id.add(),
            timeout: chrono::Local::now().timestamp() + delay,
        };
        let i = t.id;
        self.toasts.insert(i, t);
        i
    }

    pub fn info(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        self.create(title.into(), msg.into(), ToastLevel::Info, 3)
    }

    pub fn warning(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        self.create(title.into(), msg.into(), ToastLevel::Warning, 6)
    }

    pub fn error(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        self.create(title.into(), msg.into(), ToastLevel::Error, 10)
    }

    pub fn log(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        self.create(title.into(), msg.into(), ToastLevel::Log, 15)
    }
}

#[derive(Clone)]
struct Toast {
    title: String,
    msg: String,
    level: ToastLevel,
    id: usize,
    timeout: i64,
}

#[derive(PartialEq, Clone)]
pub enum ToastLevel {
    Info,
    Warning,
    Error,
    Log,
}

pub fn init_toaster() {
    let mut r = provide_root_context(Signal::new(Toaster::new()));
    use_future(move || async move {
        loop {
            {
                let mut toaster = r.write();
                let n = chrono::Local::now().timestamp();
                let ts = toaster.toasts();
                'l: for t in ts {
                    if t.timeout <= n {
                        toaster.remove(t.id);
                        break 'l;
                    }
                }
            }

            async_std::task::sleep(Duration::from_millis(250)).await;
        }
    });
}

#[component]
pub fn ToasterElement() -> Element {
    let mut r = use_context::<Signal<Toaster>>();

    rsx! {
        div {
            class: "toaster-provider",
            for toast in r.read().toasts() {
                div {
                    class: "toast-wrapper",
                    class: if toast.level == ToastLevel::Info {"info"},
                    class: if toast.level == ToastLevel::Warning {"warning"},
                    class: if toast.level == ToastLevel::Error {"error"},
                    class: if toast.level == ToastLevel::Log {"log"},

                    h3 {
                        class: "title",
                        span {
                            class: "dot"
                        }
                        {toast.title.clone()}
                    },

                    p {
                        class: "content",
                        {toast.msg.clone()}
                    },

                    button {
                        class: "close",
                        onclick: move |_| {
                            let i = toast.id;
                            r.write().remove(i);
                        },
                        icons::X {}
                    }
                }
            }
        }
    }
}

mod id {
    use std::fmt::Display;

    #[derive(Debug, Clone)]
    pub struct ID(usize);

    impl ID {
        pub fn new() -> Self {
            Self(10)
        }

        pub fn add(&mut self) -> usize {
            let current = self.0;
            if self.0 == usize::MAX {
                self.0 = 10;
            } else {
                self.0 += 1;
            }

            current
        }
    }

    impl Display for ID {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{}", self.0))
        }
    }
}

pub trait ToasterWriter {
    fn info(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize;
    fn warning(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize;
    fn error(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize;
    fn log(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize;
}

impl ToasterWriter for Signal<Toaster> {
    fn info(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        Toaster::info(&mut *self.write(), title, msg)
    }

    fn warning(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        Toaster::warning(&mut *self.write(), title, msg)
    }

    fn error(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        Toaster::error(&mut *self.write(), title, msg)
    }

    fn log(&mut self, title: impl Into<String>, msg: impl Into<String>) -> usize {
        Toaster::log(&mut *self.write(), title, msg)
    }
}