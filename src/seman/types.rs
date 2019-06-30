
#[derive(Debug, PartialEq)]
pub enum R {
    RO,
    RW
}

#[derive(Debug, PartialEq)]
pub enum Tipo {
    TUnit,
    TNil,
    TInt(R),
    TString,
    TArray(Box<Tipo>, Box<()>),
    TRecord(Vec<Tipo>, Box<()>),
    TTipo(String)
}

/*
impl Eq for Tipo {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TRecord(_), TNil) => true,

        }
    }
}
*/