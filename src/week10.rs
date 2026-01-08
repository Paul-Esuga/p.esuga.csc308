use std::thread;

pub fn tasks() {
  // thread::spawn(|| {
  //   println!("Hello from a new thread");
  // });

  // println!("Hello from the main thread");
  // thread::sleep(std::time::Duration::from_millis(10));

  let worker1 = thread::spawn(|| {
    for i in 1..6  {
        println!("worker1: T{}", i);
    }
  });
  println!("Hello from the main thread");



  let worker2 = thread::spawn(|| {
    for i in 1..6  {
        println!("worker2: T{}", i);
    }
  });
  println!("Hello from the main thread");


  let worker3 = thread::spawn(|| {
    for i in 1..6  {
        println!("worker3: T{}", i);
    }
  });
  println!("Hello from the main thread");


  worker1.join().unwrap();
  worker2.join().unwrap();
  worker3.join().unwrap();
  println!("Main thread: done.")
}
