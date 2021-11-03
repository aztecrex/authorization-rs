use authorization_core::effect::Effect;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Principal<Id> {
    Single(Id),
    Compound(Vec<Id>),
}

pub trait Query {
    type PId;
    type Err;
    type Resource;
    type Action;

    fn authorize(
        &self,
        principal: &Principal<Self::PId>,
        query: dyn Iterator<Item = (Self::Resource, Self::Action)>,
    ) -> Result<Vec<((Self::Resource, Self::Action), Effect)>, Self::Err>;
}
