use super::pronunciation::Pronunciation;
use super::sense::Sense;

#[derive(uniffi::Record, Debug, Clone)]
pub struct Etymology {
    pub id: Option<String>,
    pub pronunciations: Vec<Pronunciation>,
    pub senses: Vec<Sense>,
    pub description: Option<String>,
}

impl From<odict::schema::Etymology> for Etymology {
    fn from(ety: odict::schema::Etymology) -> Self {
        Self {
            id: ety.id,
            pronunciations: ety.pronunciations.into_iter().map(Pronunciation::from).collect(),
            senses: ety.senses.into_iter().map(Sense::from).collect(),
            description: ety.description,
        }
    }
}
