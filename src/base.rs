use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct Row {
    pub contig: String,
    pub bin: String,
    pub binner: String
}
impl Row{
    pub fn test(&self)-> String{
        format!("{},{},{}\n", self.binner, self.contig, self.bin)
    }

}
#[derive(Debug, Deserialize, Serialize)]
pub struct folders {
    pub name: String,
    pub bins: Vec<Bin>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Binner {
    pub name: String,
    pub bins: Vec<Bin>
}

#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Bin {
    pub name: String,
    pub binner: String,
    pub checksum: String,
    pub contigs: Vec<String>,
}

impl Bin {
    pub fn id(&self) -> String {
        format!("{}/{}",self.binner, self.name)
    }
}
