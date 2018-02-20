fn main() {
    let a = Arc::new(AtomicUsize::new(0));
    let a_clone = a.clone();

    let b = Arc::new(AtomicUsize::new(0));
    let b_clone = b.clone();

    let c = Arc::new(AtomicUsize::new(0));
    let c_clone = c.clone();

    let d = Arc::new(AtomicUsize::new(0));
    let d_clone = d.clone();

    thread::spawn(move || {
      // thread::sleep(Duration::from_nanos(1));
      
      // a_clone.store(1, Ordering::Relaxed);
      // c.store(b.load(Ordering::Relaxed), Ordering::Relaxed);
      a_clone.store(1, Ordering::Release);
      c.store(b.load(Ordering::Acquire), Ordering::Release);
    });
    thread::spawn(move || {
      // b_clone.store(1, Ordering::Relaxed);
      // d.store(a.load(Ordering::Relaxed), Ordering::Relaxed);
      b_clone.store(1, Ordering::Release);
      d.store(a.load(Ordering::Acquire), Ordering::Release);
    });

    thread::sleep(Duration::from_millis(500));
    println!("c:{} d:{}", c_clone.load(Ordering::Acquire), d_clone.load(Ordering::Acquire))
    // println!("c:{} d:{}", c_clone.load(Ordering::Relaxed), d_clone.load(Ordering::Relaxed))
}