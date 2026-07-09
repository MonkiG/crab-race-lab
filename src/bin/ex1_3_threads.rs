use std::thread;
fn main (){
    for i in 1..=5 {
        println!("Tick Main thread before spawning threads: {}", i);

        thread::sleep(std::time::Duration::from_millis(1500));
    }

    let thread_a = thread::spawn(|| {
        for i in 1..=5 {
            println!("Tick A: {}", i);

            thread::sleep(std::time::Duration::from_millis(1500));
        }
    });

    let thread_b = thread::spawn(|| {
        for i in 1..=5 {
            println!("Tick B: {}", i);

            thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    for i in 1..=5 {
        println!("Tick Main thread after spawning threads: {}", i);

        thread::sleep(std::time::Duration::from_millis(1500));
    }
    thread_a.join().unwrap();
    thread_b.join().unwrap();

    println!("All threads completed.");
    for i in 1..=5 {
        println!("Tick Main thread after threads completed: {}", i);

        thread::sleep(std::time::Duration::from_millis(1500));
    }

    println!("--- End of threads activity ---");
}