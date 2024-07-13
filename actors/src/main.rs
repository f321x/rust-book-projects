use calculator_actor::CalculatorActorHandle;

mod calculator_actor;

#[tokio::main]
async fn main() {
    let multiplier = CalculatorActorHandle::new();

    let product = multiplier.let_actor_multiply(2, 2).await;
    println!("{product}");
}
