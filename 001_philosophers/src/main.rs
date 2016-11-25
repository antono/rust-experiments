use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();

        thread::sleep(Duration::from_millis(150));

        let _right = table.forks[self.right].lock().unwrap();

        println!("{} начала есть", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} закончила есть", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table {
        forks: vec![Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(()), Mutex::new(())],
    });

    let philosophers = vec![Philosopher::new("Джудит Батлер", 0, 1),
                            Philosopher::new("Рая Дунаевская", 1, 2),
                            Philosopher::new("Зарубина Наталья", 2, 3),
                            Philosopher::new("Эмма Гольдман", 3, 4),
                            Philosopher::new("Анна Шмидт", 0, 4)];

    let handles: Vec<_> = philosophers.into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
