use std::sync::Arc;

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

struct Control; // Actual contents omitted for example
struct Data; // Actual contents omitted for example

struct Actor {
    data_rx: Receiver<Data>,
    ctrl_rx: Receiver<Control>,
}

impl Actor {
    async fn run(me: Arc<Mutex<Self>>) {
        let mut actor = me.lock().await;
        let actor = &mut *actor;
        loop {
            let _ = tokio::select! {
                Some(ctrl) = actor.ctrl_rx.recv() => {
                    eprintln!("ctrl");
                }
                Some(data) = actor.data_rx.recv() => {
                    eprintln!("ctrl");
                }
            };
        }
    }
}

struct Manager {
    actor: Arc<Mutex<Actor>>,
    data_tx: Sender<Data>,
    ctrl_tx: Sender<Control>,
}

impl Manager {
    async fn run(&self) {
        let actor = self.actor.clone();
        let actor_task = tokio::spawn(async move { Actor::run(actor).await });

        actor_task.await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (data_tx, data_rx) = mpsc::channel::<Data>(10);
    let (ctrl_tx, ctrl_rx) = mpsc::channel::<Control>(10);

    let manager = Manager {
        data_tx,
        ctrl_tx,
        actor: Arc::new(Mutex::new(Actor { data_rx, ctrl_rx })),
    };
}
