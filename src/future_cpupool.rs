use futures_cpupool::{CpuFuture, CpuPool};

fn fuga (thread_pool: &CpuPool, num: i16) -> CpuFuture<(), ()> {
  thread_pool.spawn_fn(move || {
    thread::sleep(Duration::from_millis((num * 300) as u64));
    println!("{}", num);
    Ok(())
  })
}

fn main() {
  let start = SteadyTime::now();
  let thread_pool = CpuPool::new(4);
  let a = fuga(&thread_pool, 3);
  let b = fuga(&thread_pool, 2);

  let hoge = vec![a, b];
  let mut one = future::select_all(hoge);
  while let Ok((value, _idx, remaining)) = one.wait() {
      if remaining.is_empty() {
          break;
      }
      one = future::select_all(remaining);
  }

  thread::sleep(Duration::from_millis(1500));
  println!("{}", SteadyTime::now() - start);
}