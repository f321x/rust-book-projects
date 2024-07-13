use tokio::sync::{mpsc, oneshot};

// actor //
pub enum CalculationMessage {
    Multiply {
        n1: i64,
        n2: i64,
        respond_to: oneshot::Sender<i64>,
    },
}

pub struct CalculatorActor {
    receiver: mpsc::Receiver<CalculationMessage>,
}

impl CalculatorActor {
    fn new(receiver: mpsc::Receiver<CalculationMessage>) -> Self {
        CalculatorActor { receiver }
    }

    fn handle_message(&mut self, msg: CalculationMessage) {
        match msg {
            CalculationMessage::Multiply { n1, n2, respond_to } => {
                let result = n1 * n2;
                let _ = respond_to.send(result);
            }
        }
    }
}

async fn run_calculation_actor(mut actor: CalculatorActor) {
    while let Some(message) = actor.receiver.recv().await {
        actor.handle_message(message);
    }
}

// handle //

pub struct CalculatorActorHandle {
    sender: mpsc::Sender<CalculationMessage>,
}

impl CalculatorActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = CalculatorActor::new(receiver);
        tokio::spawn(run_calculation_actor(actor));
        Self { sender }
    }

    pub async fn let_actor_multiply(&self, n1: i64, n2: i64) -> i64 {
        let (send, recv) = oneshot::channel();
        let message = CalculationMessage::Multiply {
            n1,
            n2,
            respond_to: send,
        };
        let _ = self.sender.send(message).await;
        recv.await.expect("Actor has been killed")
    }
}
