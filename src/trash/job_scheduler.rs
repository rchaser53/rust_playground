// extern crate job_scheduler;

// use job_scheduler::{JobScheduler, Job};
// use std::time::Duration;

// fn main() {
//     let mut sched = JobScheduler::new();

//     sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
//         println!(2);
//     }));

//     loop {
//         sched.tick();

//         std::thread::sleep(Duration::from_millis(500));
//     }
// }