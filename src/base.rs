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
