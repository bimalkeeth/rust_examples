pub struct Games;
pub struct Enemy;
pub struct Hero;

pub trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("enemy loaded")
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("hero loaded")
    }
}

impl Games {
    pub fn load<T>(&self, entity: T) where T: Loadable {
        entity.init();
    }
}


