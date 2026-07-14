#[tokio::main]
async fn main() {
    const MAX_COUNT: u32 = 5;

    for i in 1..=MAX_COUNT {
        println!("Tick Main thread before spawning tasks: {}", i);

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }


    let task_1 = tokio::task::spawn(async {
        for i in 1..=MAX_COUNT {
            println!("Tick of task 1: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let task_2 = tokio::task::spawn(async {
        for i in 1..=MAX_COUNT {
            println!("Tick of task 2: {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        }
    });

    task_1.await.unwrap();
    task_2.await.unwrap();
}