use crate::services::IService;
use crate::snowflake::Snowflake;

pub struct IdService {
    worker: Snowflake
}

impl IService for IdService {
    
}

impl IdService{
    pub fn new(worker_id: u64)-> Self{
        IdService{
            worker: Snowflake::new(worker_id).unwrap(),
        }
    }


    pub fn id(&self) -> String{
        self.worker.next_id().to_string()
    }

    pub fn ids(&self, numbers: u32) -> Vec<String> {
        (0..numbers).map(|_|  self.id()).collect()
    }

    pub fn parse(&self, id: &String) -> (u64, u64, u64){
        let _id = id.parse::<u64>().unwrap();
        self.worker.parse_id(_id)
    }
}