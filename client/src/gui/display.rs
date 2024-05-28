mod atoms;
mod molecules;
mod organisms;
pub mod pages;

// Every atom, molecule, and organism will have a data context which is either a rule-set or a setting restriction.
// A rule-set restriction is less strict than a setting restriction, as multiple settings could exist for the same rule-set.
enum Restriction {
    RuleSet(String), // The ID of the rule-set
    Setting(String), // The ID of the setting
}

trait ContextRestriction {
    fn get_restriction() -> Option<Restriction>;
}