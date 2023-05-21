
pub struct Audio(pub String);
pub struct Video(pub String);

pub trait Playable {
    fn play(&self);
    fn pause(){
        println!("Paused");
    }
}

impl Playable for Audio{
    fn play(&self) {
        println!("now playing {}",self.0)
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("now playing {}",self.0)
    }
}

