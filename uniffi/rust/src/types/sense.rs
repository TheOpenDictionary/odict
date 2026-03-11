use internal::ToEnumWrapper;
use super::enums::EnumWrapper;
use super::definition::Definition;
use super::group::Group;
use super::translation::Translation;
use super::form::Form;

#[derive(uniffi::Enum, Debug, Clone)]
pub enum DefinitionOrGroup {
    Definition(Definition),
    Group(Group),
}

#[derive(uniffi::Record, Debug, Clone)]
pub struct Sense {
    pub pos: EnumWrapper,
    pub lemma: Option<String>,
    pub definitions: Vec<DefinitionOrGroup>,
    pub tags: Vec<String>,
    pub translations: Vec<Translation>,
    pub forms: Vec<Form>,
}

impl From<odict::schema::Sense> for Sense {
    fn from(sense: odict::schema::Sense) -> Self {
        Sense {
            pos: sense.pos.to_enum_wrapper().into(),
            lemma: sense.lemma.map(|entry_ref| entry_ref.to_string()),
            definitions: sense
                .definitions
                .into_iter()
                .map(|def_type| match def_type {
                    odict::schema::DefinitionType::Definition(def) => {
                        DefinitionOrGroup::Definition(def.into())
                    }
                    odict::schema::DefinitionType::Group(group) => {
                        DefinitionOrGroup::Group(group.into())
                    }
                })
                .collect(),
            tags: sense.tags,
            translations: sense.translations.into_iter().map(|t| t.into()).collect(),
            forms: sense.forms.into_iter().map(|f| f.into()).collect(),
        }
    }
}
