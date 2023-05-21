pub trait Vehicle {
    fn get_price(&self)->u64;
}

pub trait Car:Vehicle{
    fn model(&self)->String;
}

pub struct TeslaRoadStar{
    model:String,
    release_date:u16,
}

impl TeslaRoadStar{
    pub fn new(mode:&str,rel_date:u16)->Self{
        return Self{model:mode.to_string(),release_date:rel_date}
    }
}


impl Vehicle for TeslaRoadStar {
    fn get_price(&self) -> u64 {
        return 2000
    }
}

impl Car for TeslaRoadStar{
    fn model(&self) -> String {
        self.model.to_string()
    }
}