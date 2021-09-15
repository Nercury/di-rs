extern crate di;

use di::Deps;
use std::sync::Arc;

struct Window {
    pub resize_listeners: Vec<Box<dyn Fn(i32, i32) + Sync + Send>>,
}

struct Logger {
    pub log_fn: Arc<dyn Fn(&str) + Send + Sync>,
}

impl Window {
    fn new() -> Window {
        Window {
            resize_listeners: Vec::new(),
        }
    }

    fn resize(&self, w: i32, h: i32) {
        for listener in &self.resize_listeners {
            listener(w, h);
        }
    }
}

impl Logger {
    fn new<F>(log_function: F) -> Logger
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        Logger {
            log_fn: Arc::new(log_function),
        }
    }
}

fn main() {
    let mut deps = Deps::new();

    deps.bridge(|window: &mut Window, logger: &mut Logger| {
        // while window and logger both exist, log messages to logger
        let log_fn_clone = logger.log_fn.clone();
        window.resize_listeners.push(Box::new(move |w, h| {
            log_fn_clone(&format!("window resized to w: {} and h: {}", w, h));
        }));
        Ok(())
    });

    let mut window = deps.create(Window::new()).unwrap();
    deps.create(Logger::new(|message| println!("message: {:?}", message)))
        .unwrap();

    window.lock().unwrap().resize(12, 13);
}
