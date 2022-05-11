use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub enum Message {
    NewTask(Task),
    Exit,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Task = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(cap: usize) -> Self {
        assert!(cap > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(cap);
        for i in 0..cap {
            workers.push(Worker::new(i, receiver.clone()));
        }
        ThreadPool {
            workers: workers,
            sender: sender,
        }
    }
    pub fn execute<F: FnOnce() + 'static + Send>(&self, f: F) {
        self.sender.send(Message::NewTask(Box::new(f))).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Exit).unwrap()
        }
        println!("all thread stopping......");
        println!("{}", self.workers.len());
        for worker in &mut self.workers {
            println!("thread {} was stop!", worker.id);
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let t = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewTask(func) => {
                    println!("---thread({}) begin func---", id);
                    func();
                    println!("---thread({}) end func---", id);
                }
                Message::Exit => {
                    println!("thread({}) stop", id);
                    break;
                }
            }
        });
        Worker {
            id: id,
            thread: Some(t),
        }
    }
}
