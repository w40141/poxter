use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Default)]
pub struct Id(Ulid);

impl Id {
    pub fn new() -> Self {
        Self(Ulid::new())
    }

    pub fn get(&self) -> &Ulid {
        &self.0
    }
}

impl From<Ulid> for Id {
    fn from(v: Ulid) -> Self {
        Id(v)
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};

    use super::*;

    #[test]
    fn id_test() {
        {
            let id = Id::new();
            let id_2 = id.clone();
            assert_eq!(id, id_2);
        }
        {
            let old_id = Id::new();
            let ten_millis = time::Duration::from_millis(10);
            thread::sleep(ten_millis);
            let new_id = Id::new();
            assert!(old_id < new_id);
        }
    }
}
