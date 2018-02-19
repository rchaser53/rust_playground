struct Client {
    nyan: i32,
}

impl Client {
    pub fn hoge(&self, a: i32) -> i32 {
        return self.nyan + a;
    }
}

struct Runner {
    client: Arc<Client>,
}

impl Runner {
    fn run(&self, data: Vec<i32>) {
        let (tx, rx) = mpsc::channel();
        for &x in data.iter() {
            let mut client = self.client.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                tx.send(client.hoge(x));
            });
        }

        for i in 0..data.len() {
            println!("{:?}", rx.recv());
        }
    }
}

fn main() {
  let runner = Runner{
    client: Arc::new(Client{nyan: 123})
  };
  runner.run(vec![1,2,3]);
}
