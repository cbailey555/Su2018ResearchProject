#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash)]
pub struct ElectionStatus {
    pub cstatus: bool,
    pub rstatus: bool,
}

impl ElectionStatus {
    pub fn new() -> Self {
        ElectionStatus {
            cstatus: true,
            rstatus: true,
        }
    }
    pub fn get_cstatus(&self) -> bool {
        self.cstatus
    }

    pub fn get_rstatus(&self) -> bool {
        self.rstatus
    }

    pub fn set_cstatus(&mut self, b: bool) {
        self.cstatus = b
    }

    pub fn set_rstatus(&mut self, b: bool) {
        self.rstatus = b
    }

}