use std::{borrow::BorrowMut, rc::Rc};

use crate::model::{data_model::primatives::{location::Location, types::{die_roll::DieRollType, enumeration::EnumerationType, equation::Equation, modifier::ModifierType, Type}, values::{meta::MetaInst, Value}, wiki::WikiPage}, types::CharacterId};

use super::{character::Character, game::{Game, GameMasterData}, location::LocationRef, playset::Playset, ruleset::Ruleset, setting::Setting, types::{DieRollTypeRef, EnumerationTypeRef, EquationRef, ModifierTypeRef, TypeRef}, values::{MetaInstRef, ValueRef}, wiki::WikiPageRef, IndexRef, IndexStorage, Query, QueryError, RefTarget};

#[derive(Debug, PartialEq, Clone)]
pub struct IntermediateView {
    ruleset: Option<Rc<Ruleset>>,
    setting: Option<Rc<Setting>>,
    characters: Vec<Rc<Character>>,
}

impl IntermediateView {
    pub fn from_ruleset(r: Ruleset) -> IntermediateView {
        IntermediateView { ruleset: Some(Rc::new(r)), setting: None, characters: vec![] }
    }

    pub fn set_ruleset(&mut self, r: Ruleset) {
        self.ruleset = Some(Rc::new(r));
    }

    pub fn set_setting(&mut self, s: Setting) {
        self.setting = Some(Rc::new(s));
    }

    pub fn set_characters(&mut self, c: Vec<Rc<Character>>) {
        self.characters = c;
    }

    pub fn get_ruleset(&self) -> Option<&Ruleset> {
        self.ruleset.as_ref().map(|s| s.as_ref())
    }

    pub fn get_setting(&self) -> Option<&Setting> {
        self.setting.as_ref().map(|s| s.as_ref())
    }

    pub fn get_character(&self, id: &CharacterId) -> Option<&Character> {
        for c in self.characters.iter() {
            if c.id.eq(id) {
                return Some(c);
            }
        }
        None
    }

    pub fn get_playset<'a>(&'a self) -> Option<Playset<'a>> {
        if self.ruleset.is_none() && self.setting.is_none() { 
            return None; 
        }
        Some(Playset::new(self.get_ruleset(), self.get_setting()))
    }

    pub fn get_ruleset_rc(&self) -> Option<Rc<Ruleset>> {
        self.ruleset.clone()
    }

    pub fn get_setting_rc(&self) -> Option<Rc<Setting>> {
        self.setting.clone()
    }

    pub fn get_character_rc(&self, id: &CharacterId) -> Option<Rc<Character>> {
        for c in self.characters.iter() {
            if c.id.eq(id) {
                return Some(c.clone());
            }
        }
        None
    }
}

// ---------------- Ref Implementations ---------------------------

impl IndexStorage<WikiPage, WikiPageRef> for IntermediateView {
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
                Err(r.to_target_dne_error())
            },
            RefTarget::GameplayData => Err(r.to_target_dne_error()),
            RefTarget::GamemasterData => todo!(),
            RefTarget::Character(_) => todo!(),
        }
    }
}

impl IndexStorage<Location, LocationRef> for IntermediateView {
    fn get(&self, r: &LocationRef) -> Query<&Location> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(r.to_target_dne_error()),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

// ---------- Values -------------

impl IndexStorage<Value, ValueRef> for IntermediateView {
    fn get(&self, r: &ValueRef) -> Query<&Value> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(r.to_target_dne_error()),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<MetaInst, MetaInstRef> for IntermediateView {
    fn get<'a>(&'a self, r: &MetaInstRef) -> Query<&'a MetaInst> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(r.to_target_dne_error()),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

// ----------- Types -------------

impl IndexStorage<Type, TypeRef> for IntermediateView {
    fn get(&self, r: &TypeRef) -> Query<&Type> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(r.to_target_dne_error()),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<EnumerationType, EnumerationTypeRef> for IntermediateView {
    fn get<'a>(&'a self, r: &EnumerationTypeRef) -> Query<&'a EnumerationType> {
        todo!()
    }
}

impl IndexStorage<DieRollType, DieRollTypeRef> for IntermediateView {
    fn get<'a>(&'a self, r: &DieRollTypeRef) -> Query<&'a DieRollType> {
        todo!()
    }
}

impl IndexStorage<Equation, EquationRef> for IntermediateView {
    fn get<'a>(&'a self, r: &EquationRef) -> Query<&'a Equation> {
        match r.get_target() {
            RefTarget::Playset => todo!(),
            RefTarget::Character(_) => todo!(),
            RefTarget::GameplayData => Err(r.to_target_dne_error()),
            RefTarget::GamemasterData => todo!(),
        }
    }
}

impl IndexStorage<ModifierType, ModifierTypeRef> for IntermediateView {
    fn get<'a>(&'a self, r: &ModifierTypeRef) -> Query<&'a ModifierType> {
        todo!()
    }
}