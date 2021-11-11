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
        query: &dyn Iterator<Item = (Self::Resource, Self::Action)>,
    ) -> Result<Vec<((Self::Resource, Self::Action), Effect)>, Self::Err>;
}

pub trait Authorization {
    type PId;
    type Err;
    type Resource;
    type Action;

    fn authorize<I>(&self, principal: &Principal<Self::PId>, permissions: I) -> ()
    where
        I: IntoIterator<Item = (Self::Resource, Self::Action)>;
}

impl Query for () {
    type PId = ();
    type Err = ();
    type Resource = ();
    type Action = ();

    fn authorize(
        &self,
        _principal: &Principal<Self::PId>,
        _query: &dyn Iterator<Item = (Self::Resource, Self::Action)>,
    ) -> Result<Vec<((Self::Resource, Self::Action), Effect)>, Self::Err> {
        // 1. fetch policies for each principal
        // 1.a fetch attached policies
        // 1.b fetch indirect policies (roles?)
        // 1c. fetch indirect policy template parameters and apply to templates (associated roles?)
        // 2. for each query item, compute for each principle
        // 3. combine strict for each principle
        // 4. zip with query items
        todo!();
    }
}
