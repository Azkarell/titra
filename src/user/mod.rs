
#[derive(Clone)]
pub struct UserData {
    pub name: String,
    pub street: String,
    pub citycode: String,

}

impl UserData {
    pub fn new(name: String, street: String, citycode: String) -> Self {
        Self { name, street, citycode }
    }
}

