use crate::model::data_model::primatives::{location::Location, permissions::CharacterId, types::{enumeration::EnumerationType, equation::Equation, Type}, values::{meta::MetaInst, Value}, wiki::WikiPage};

use super::{character::Character, game::{Game, GameMasterData}, location::LocationRef, playset::Playset, ruleset::Ruleset, setting::Setting, types::{EnumerationTypeRef, EquationRef, TypeRef}, values::{MetaInstRef, ValueRef}, wiki::WikiPageRef, IndexRef, IndexStorage, Query, QueryError, RefTarget};

#[derive(Debug, PartialEq, Clone)]
pub struct IntermediateView<'a> {
    ruleset: Option<Ruleset<'a>>,
    setting: Option<Setting<'a>>,
    characters: Vec<Character<'a>>,
    game_master_data: Option<GameMasterData<'a>>,
}

impl<'g> IntermediateView<'g> {
    pub fn get_ruleset(&self) -> Option<&Ruleset> {
        self.ruleset.as_ref()
    }

    pub fn get_setting(&self) -> Option<&Setting> {
        self.setting.as_ref()
    }

    pub fn get_character(&self, id: &CharacterId) -> Option<&Character> {
        for c in self.characters.iter() {
            if c.id.eq(id) {
                return Some(c);
            }
        }
        None
    }

    pub fn get_mut_ruleset(&mut self) -> Option<&mut Ruleset<'g>> {
        self.ruleset.as_mut()
    }

    pub fn get_mut_setting(&mut self) -> Option<&mut Setting<'g>> {
        self.setting.as_mut()
    }

    pub fn get_mut_character(&mut self, id: &CharacterId) -> Option<&mut Character<'g>> {
        for c in self.characters.iter_mut() {
            if c.id.eq(id) {
                return Some(c);
            }
        }
        None
    }

    pub fn get_playset<'a>(&'a self) -> Option<Playset<'a, 'g>> {
        if self.ruleset.is_none() && self.setting.is_none() { 
            return None; 
        }
        Some(Playset::new(self.ruleset.as_ref(), self.setting.as_ref()))
    }
}

// ---------------- Ref Implementations ---------------------------

impl IndexStorage<WikiPage, WikiPageRef> for IntermediateView<'_> {
    fn get(&self, r: &WikiPageRef) -> Query<&WikiPage> {
        match r.get_target() {
            RefTarget::Playset => {
                if let Some(setting) = &self.setting {
                    let res = setting.get(r);
                    if res.as_ref().is_err_and(|e| e.is_dne_err()) {
                        if let Some(ruleset) = &self.setting {
                            return ruleset.get(r)
                        }
                    }
                    return res;
                }
                Err(QueryError::TargetDoesNotExist(r.get_target()))
            },
            RefTarget::GameplayData => Err(QueryError::TargetDoesNotExist(r.get_target())),
            RefTarget::GamemasterData => todo!(),
            RefTarget::Character(_) => todo!(),
        }
    }
}

impl IndexStorage<Location, LocationRef> for IntermediateView<'_> {
    fn get(&self, r: &LocationRef) -> Query<&Location> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(QueryError::TargetDoesNotExist(r.get_target())),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

// ---------- Values -------------

impl IndexStorage<Value, ValueRef> for IntermediateView<'_> {
    fn get(&self, r: &ValueRef) -> Query<&Value> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(QueryError::TargetDoesNotExist(r.get_target())),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<MetaInst, MetaInstRef> for IntermediateView<'_> {
    fn get<'a>(&'a self, r: &MetaInstRef) -> Query<&'a MetaInst> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => todo!(),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

// ----------- Types -------------

impl IndexStorage<Type, TypeRef> for IntermediateView<'_> {
    fn get(&self, r: &TypeRef) -> Query<&Type> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(QueryError::TargetDoesNotExist(r.get_target())),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<EnumerationType, EnumerationTypeRef> for IntermediateView<'_> {
    fn get<'a>(&'a self, r: &EnumerationTypeRef) -> Query<&'a EnumerationType> {
        todo!()
    }
}

impl IndexStorage<Equation, EquationRef> for IntermediateView<'_> {
    fn get<'a>(&'a self, r: &EquationRef) -> Query<&'a Equation> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(QueryError::TargetDoesNotExist(r.get_target())),
            RefTarget::GamemasterData => todo!(),
        }
    }
}