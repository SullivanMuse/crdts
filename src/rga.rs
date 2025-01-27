use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Id {
    clock: usize,
    client: usize,
}

#[derive(Debug)]
pub struct Rga<T> {
    items: BTreeMap<Id, T>,
}

impl<T> Rga<T> {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn push(&mut self, id: Id, value: T) {
        self.items.insert(id, value);
    }

    pub fn to_vec(&self) -> Vec<(Id, T)>
    where
        T: Clone,
    {
        self.items
            .iter()
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    struct Client {
        id: usize,
        count: usize,
    }

    impl Client {
        fn new(id: usize) -> Self {
            Self { id, count: 0 }
        }

        fn update<T>(&mut self, rga: &Rga<T>) {
            if let Some((key, _)) = rga.items.last_key_value() {
                self.count = key.clock + 1;
            }
        }

        fn get_count(&mut self) -> usize {
            let out = self.count;
            self.count += 1;
            out
        }

        fn id(&mut self) -> Id {
            Id {
                clock: self.get_count(),
                client: self.id,
            }
        }
    }

    #[test]
    fn test1() {
        let mut rga = Rga::new();

        let mut a = Client::new(0);
        let mut b = Client::new(1);

        let id = a.id();
        let value = 1234;

        rga.push(id, value);

        let id = a.id();
        rga.push(id, value);

        dbg!(&rga);

        dbg!(&a);
        dbg!(&b);
        a.update(&rga);
        b.update(&rga);
        dbg!(&a);
        dbg!(&b);

        let id = b.id();
        rga.push(id, 2);
        let id = b.id();
        rga.push(id, 3);

        b.update(&rga);
        dbg!(&b);

        let id = a.id();
        rga.push(id, 1);

        dbg!(&rga);
    }
}
