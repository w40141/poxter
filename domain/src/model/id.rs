use ulid::Ulid;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Id(Ulid);

impl Id {
    pub fn new() -> Self {
        Self(Ulid::new())
    }

    pub fn id(&self) -> &Ulid {
        &self.0
    }
}
