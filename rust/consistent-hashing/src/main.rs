use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{
    collections::BTreeMap,
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
    time::Duration,
};
use tokio::{sync::Mutex, time::sleep};

#[derive(Debug)]
struct Server {
    id: u32,
    active: Arc<Mutex<bool>>,
}

impl Server {
    fn new(id: u32) -> Self {
        let active = Arc::new(Mutex::new(true));
        Self { id, active }
    }

    async fn is_active(&self) -> bool {
        *self.active.lock().await
    }

    async fn take_down(&self, secs: u64) {
        {
            let mut active = self.active.lock().await;
            *active = false;
            println!("Server {} is down", self.id);
        }

        sleep(Duration::from_secs(secs)).await;

        {
            let mut active = self.active.lock().await;
            *active = true;
            println!("Server {} is back up", self.id);
        }
    }
}

struct ConsistentHashing {
    ring: BTreeMap<u64, Arc<Server>>,
}

impl ConsistentHashing {
    fn new(servers: Vec<Arc<Server>>) -> Self {
        let mut ring = BTreeMap::new();

        for server in servers {
            for i in 0..3 {
                let key = calculate_hash(format!("{i}{}", server.id));
                ring.insert(key, server.clone());
            }
        }

        Self { ring }
    }

    async fn get_server_for_request(&self, request: &str) -> Option<Arc<Server>> {
        let range = calculate_hash(request);
        let servers = self.ring.range(range..).chain(self.ring.range(..range));

        for (_, server) in servers {
            if server.is_active().await {
                return Some(server.clone());
            }
        }

        None
    }
}

fn calculate_hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

async fn take_down_servers(servers: Vec<Arc<Server>>) {
    let mut rnd = StdRng::from_entropy();

    loop {
        let idx = rnd.gen_range(0..servers.len());
        let server = servers[idx].clone();

        tokio::spawn(async move { server.take_down(4).await });

        sleep(Duration::from_secs(4 - 1)).await;
    }
}

#[tokio::main]
async fn main() {
    let servers = vec![
        Arc::new(Server::new(1)),
        Arc::new(Server::new(2)),
        Arc::new(Server::new(3)),
    ];

    tokio::spawn(take_down_servers(servers.clone()));

    let consistent_hashing = Arc::new(ConsistentHashing::new(servers.clone()));

    let mut requests = vec![];
    for i in 0..100 {
        requests.push(format!("Request {i}"));
    }

    for request in requests {
        let consistent_hashing_clone = Arc::clone(&consistent_hashing);

        tokio::spawn(async move {
            match consistent_hashing_clone
                .get_server_for_request(&request)
                .await
            {
                Some(server) => println!("{request} handle by server {}", server.id),
                None => panic!("No server available for {request}"),
            }
        });

        sleep(Duration::from_secs(1)).await;
    }
}
