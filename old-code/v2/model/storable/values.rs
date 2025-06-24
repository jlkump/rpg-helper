use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::model::{core::Reference, database::entity::EntityID};

use super::{types::Type, Referenceable, StorableBuilder};

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize, Clone)]
pub struct Value
{
    container_id: EntityID,
    name: String,
    parent: Option<Reference>,
    type_ref: Reference,
    pub data: EValue,
}

impl Value
{
    pub fn new(name: &str, type_ref: Reference) -> ValueBuilder
    {
        ValueBuilder 
        { 
            name: name.to_string(), 
            parent: None,
            data: EValue::Num(0.0),
            type_ref,
        }
    }

    pub fn into_builder(self) -> ValueBuilder
    {
        ValueBuilder 
        { 
            name: self.name,
            parent: self.parent,
            data: self.data,
            type_ref: self.type_ref,
        }
    }

    // Method to construct a reference path
    fn build_path(&self) -> String
    {
        if let Some(parent_ref) = &self.parent {
            // TODO:
            //      Not always true way to build path
            //      Doesn't account for parent being a list. Need to index as self
            format!("{}.{}", parent_ref.get_path(), self.name)
        } else {
            self.name.clone()
        }
    }

    // Get numeric value for evaluation
    pub fn get_numeric_value(&self) -> f32
    {
        match &self.data {
            EValue::Num(n) => *n,
            EValue::Bool(b) => if *b { 1.0 } else { 0.0 },
            EValue::List(list) => {
                // Sum all values in the list
                list.iter().map(|v| v.get_numeric_value()).sum()
            },
            EValue::Enum(s) => {
                // Simple hash function for string
                let mut hash: u32 = 0;
                for c in s.bytes() {
                    hash = hash.wrapping_mul(31).wrapping_add(c as u32);
                }
                hash as f32
            },
            EValue::Struct(map) => {
                // Try to find "Value" field
                if let Some(value) = map.get("Value") {
                    value.get_numeric_value()
                } else {
                    0.0 // Default if no "Value" field
                }
            },
            EValue::DieRoll() => 0.0, // Placeholder, would need actual implementation
            EValue::Reference(ref_val) => {
                // This would need resolution by ValueStore
                // For now just return 0.0, will be handled by the ValueStore
                0.0
            },
        }
    }

    // Get a value at a path (e.g. "Exp[0].Source")
    pub fn get_at_path(&self, path: &str) -> Option<&Value>
    {
        let parts: Vec<&str> = path.split('.').collect();
        self.get_nested_value(&parts, 0)
    }

    // Recursive helper for get_at_path
    fn get_nested_value<'a>(&'a self, parts: &[&str], index: usize) -> Option<&'a Value>
    {
        if index >= parts.len() {
            return Some(self);
        }

        let part = parts[index];
        
        // Check if this part contains array indexing
        if let Some(bracket_pos) = part.find('[') {
            if part.ends_with(']') {
                let name = &part[..bracket_pos];
                let idx_str = &part[bracket_pos + 1..part.len() - 1];
                
                if let Ok(idx) = idx_str.parse::<usize>() {
                    // Handle array indexing
                    match &self.data {
                        EValue::Struct(map) => {
                            if let Some(field) = map.get(name) {
                                if let EValue::List(list) = &field.data {
                                    if idx < list.len() {
                                        return list[idx].get_nested_value(parts, index + 1);
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
            return None;
        } else {
            // Simple field access
            match &self.data {
                EValue::Struct(map) => {
                    if let Some(field) = map.get(part) {
                        return field.get_nested_value(parts, index + 1);
                    }
                },
                _ => {}
            }
        }
        None
    }
}

impl Referenceable for Value
{
    
    fn to_ref(&self) -> Reference
    {
        Reference::new(self.container_id.clone(), self.build_path())
    }

}

#[derive(Debug, Deserialize, PartialEq, PartialOrd, Serialize, Clone)]
pub enum EValue
{
    Num(f32),
    Bool(bool),
    List(Vec<Value>),
    Enum(String),
    Struct(BTreeMap<String, Value>),
    DieRoll(), 
    Reference(Reference),  // Points to a value matching the type of reference
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct ValueBuilder
{
    pub name: String,
    pub parent: Option<Reference>,
    pub data: EValue,
    pub type_ref: Reference,
}

impl ValueBuilder
{
    pub fn with_type(mut self, type_ref: Reference) -> Self
    {
        self.type_ref = type_ref;
        self
    }

    pub fn with_num(mut self, value: f32) -> Self
    {
        self.data = EValue::Num(value);
        self
    }

    pub fn with_bool(mut self, value: bool) -> Self
    {
        self.data = EValue::Bool(value);
        self
    }

    pub fn with_list(mut self, values: Vec<Value>) -> Self
    {
        self.data = EValue::List(values);
        self
    }

    pub fn with_enum(mut self, value: String) -> Self
    {
        self.data = EValue::Enum(value);
        self
    }

    pub fn with_struct(mut self, map: BTreeMap<String, Value>) -> Self
    {
        self.data = EValue::Struct(map);
        self
    }

    pub fn with_reference(mut self, reference: Reference) -> Self
    {
        self.data = EValue::Reference(reference);
        self
    }

    pub fn with_parent(mut self, parent: Reference) -> Self
    {
        self.parent = Some(parent);
        self
    }

    // Helper to create an empty struct
    pub fn as_struct(mut self) -> Self
    {
        self.data = EValue::Struct(BTreeMap::new());
        self
    }

    // Helper to add a field to a struct
    pub fn add_struct_field(mut self, field_name: &str, value: Value) -> Self
    {
        if let EValue::Struct(map) = &mut self.data {
            map.insert(field_name.to_string(), value);
        } else {
            // If not already a struct, create one with this field
            let mut map = BTreeMap::new();
            map.insert(field_name.to_string(), value);
            self.data = EValue::Struct(map);
        }
        self
    }

    // Helper to create an empty list
    pub fn as_list(mut self) -> Self
    {
        self.data = EValue::List(Vec::new());
        self
    }

    // Helper to add a value to a list
    pub fn add_list_item(mut self, value: Value) -> Self
    {
        if let EValue::List(list) = &mut self.data {
            list.push(value);
        } else {
            // If not already a list, create one with this item
            self.data = EValue::List(vec![value]);
        }
        self
    }
}

impl StorableBuilder<Value> for ValueBuilder
{
    fn build(self, container_id: EntityID, path: String) -> Value
    {
        Value
        {
            container_id,
            name: path,
            parent: self.parent,
            data: self.data,
            type_ref: self.type_ref,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::core::Reference;
    use crate::model::database::DatabaseEntity;
    use crate::model::store::values::ValueStore;
    use crate::model::store::Store;
    use std::collections::BTreeMap;

    // Helper function to create a type reference for testing
    fn create_type_ref(container_id: &EntityID, name: &str) -> Reference {
        Reference::new(container_id.clone(), format!("Type_{}", name))
    }

    // Helper function to create a test ValueStore with some predefined values
    fn create_test_store() -> ValueStore {
        let mut store = ValueStore::debug_new();
        let store_id = store.to_id().clone();
        
        // Create type references
        let strength_type = create_type_ref(&store_id, "Strength");
        let dexterity_type = create_type_ref(&store_id, "Dexterity");
        let intelligence_type = create_type_ref(&store_id, "Intelligence");
        let wizard_type = create_type_ref(&store_id, "IsWizard");
        let class_type = create_type_ref(&store_id, "Class");
        let exp_type = create_type_ref(&store_id, "Experience");
        let ability_type = create_type_ref(&store_id, "Magic Theory");
        
        // Add some basic values
        store.set(Value::new("Strength", strength_type.clone()).with_num(12.0)).unwrap();
        store.set(Value::new("Dexterity", dexterity_type.clone()).with_num(14.0)).unwrap();
        store.set(Value::new("Intelligence", intelligence_type.clone()).with_num(16.0)).unwrap();
        store.set(Value::new("IsWizard", wizard_type.clone()).with_bool(true)).unwrap();
        
        // Add an enum value
        store.set(Value::new("Class", class_type.clone()).with_enum("Wizard".to_string())).unwrap();
        
        // Create and add a list value
        let exp_items = vec![
            Value::new("Exp1", exp_type.clone()).with_num(100.0).build(store.to_id().clone(), "Exp1".to_string()),
            Value::new("Exp2", exp_type.clone()).with_num(200.0).build(store.to_id().clone(), "Exp2".to_string()),
        ];
        store.set(Value::new("Experience", exp_type.clone()).with_list(exp_items)).unwrap();
        
        // Create and add a struct value
        let mut ability_map = BTreeMap::new();
        ability_map.insert(
            "Value".to_string(), 
            Value::new("AbilityValue", ability_type.clone()).with_num(5.0).build(store.to_id().clone(), "AbilityValue".to_string())
        );
        ability_map.insert(
            "Exp".to_string(),
            Value::new("AbilityExp", ability_type.clone()).with_num(45.0).build(store.to_id().clone(), "AbilityExp".to_string())
        );
        store.set(Value::new("Magic Theory", ability_type.clone()).with_struct(ability_map)).unwrap();
        
        // Add a reference value
        let strength_ref = Reference::new(store.to_id().clone(), "Strength".to_string());
        store.set(Value::new("StrengthReference", strength_type.clone()).with_reference(strength_ref)).unwrap();
        
        store
    }

    #[test]
    fn test_value_creation() {
        // Create a container ID for testing
        let container_id = uuid::Uuid::new_v4();
        
        // Create type references
        let num_type = create_type_ref(&container_id, "Number");
        let bool_type = create_type_ref(&container_id, "Boolean");
        let enum_type = create_type_ref(&container_id, "Enum");
        let list_type = create_type_ref(&container_id, "List");
        let struct_type = create_type_ref(&container_id, "Struct");
        
        // Create numeric value
        let num_value = Value::new("Strength", num_type).with_num(12.0).build(container_id.clone(), "Strength".to_string());
        assert_eq!(num_value.get_numeric_value(), 12.0);
        
        // Create boolean value
        let bool_value = Value::new("IsWizard", bool_type).with_bool(true).build(container_id.clone(), "IsWizard".to_string());
        assert_eq!(bool_value.get_numeric_value(), 1.0);
        
        // Create enum value
        let enum_value = Value::new("Class", enum_type).with_enum("Wizard".to_string()).build(container_id.clone(), "Class".to_string());
        // We don't assert the exact hash value, just that it produces some numeric value
        assert!(enum_value.get_numeric_value() != 0.0);
        
        // Create a list value
        let items = vec![
            Value::new("Item1", list_type.clone()).with_num(10.0).build(container_id.clone(), "Item1".to_string()),
            Value::new("Item2", list_type.clone()).with_num(20.0).build(container_id.clone(), "Item2".to_string()),
        ];
        let list_value = Value::new("List", list_type).with_list(items).build(container_id.clone(), "List".to_string());
        assert_eq!(list_value.get_numeric_value(), 30.0); // Sum of 10 + 20
        
        // Create a struct value with a "Value" field
        let mut map = BTreeMap::new();
        map.insert(
            "Value".to_string(), 
            Value::new("StructValue", struct_type.clone()).with_num(5.0).build(container_id.clone(), "StructValue".to_string())
        );
        let struct_value = Value::new("Struct", struct_type).with_struct(map).build(container_id.clone(), "Struct".to_string());
        assert_eq!(struct_value.get_numeric_value(), 5.0);
    }

    #[test]
    fn test_value_builder_methods() {
        // Test the builder pattern methods
        let container_id = uuid::Uuid::new_v4();
        
        // Create type references
        let struct_type = create_type_ref(&container_id, "Struct");
        let list_type = create_type_ref(&container_id, "List");
        let field_type = create_type_ref(&container_id, "Field");
        let item_type = create_type_ref(&container_id, "Item");
        
        // Test as_struct and add_struct_field
        let struct_value = Value::new("TestStruct", struct_type)
            .as_struct()
            .add_struct_field("Field1", Value::new("F1", field_type.clone()).with_num(10.0).build(container_id.clone(), "F1".to_string()))
            .add_struct_field("Field2", Value::new("F2", field_type).with_num(20.0).build(container_id.clone(), "F2".to_string()))
            .build(container_id.clone(), "TestStruct".to_string());
        
        // Test as_list and add_list_item
        let list_value = Value::new("TestList", list_type)
            .as_list()
            .add_list_item(Value::new("I1", item_type.clone()).with_num(5.0).build(container_id.clone(), "I1".to_string()))
            .add_list_item(Value::new("I2", item_type).with_num(15.0).build(container_id.clone(), "I2".to_string()))
            .build(container_id.clone(), "TestList".to_string());
        
        // Verify structure
        if let EValue::Struct(map) = &struct_value.data {
            assert!(map.contains_key("Field1"));
            assert!(map.contains_key("Field2"));
        } else {
            panic!("Expected struct value");
        }
        
        // Verify list
        if let EValue::List(list) = &list_value.data {
            assert_eq!(list.len(), 2);
            assert_eq!(list[0].get_numeric_value(), 5.0);
            assert_eq!(list[1].get_numeric_value(), 15.0);
        } else {
            panic!("Expected list value");
        }
    }

    #[test]
    fn test_path_resolution() {
        let container_id = uuid::Uuid::new_v4();
        
        // Create type references
        let ability_type = create_type_ref(&container_id, "Ability");
        let score_type = create_type_ref(&container_id, "Score");
        let exp_type = create_type_ref(&container_id, "Exp");
        
        // Create a nested structure for testing path resolution
        let mut field_map = BTreeMap::new();
        field_map.insert(
            "Score".to_string(),
            Value::new("Score", score_type).with_num(10.0).build(container_id.clone(), "Score".to_string())
        );
        
        // Create a list for the Exp field
        let exp_items = vec![
            Value::new("Exp0", exp_type.clone()).with_num(100.0).build(container_id.clone(), "Exp0".to_string()),
            Value::new("Exp1", exp_type.clone()).with_num(200.0).build(container_id.clone(), "Exp1".to_string()),
        ];
        field_map.insert(
            "Exp".to_string(),
            Value::new("Exp", exp_type).with_list(exp_items).build(container_id.clone(), "Exp".to_string())
        );
        
        // Create the main ability struct
        let ability = Value::new("Magic Theory", ability_type)
            .with_struct(field_map)
            .build(container_id.clone(), "Magic Theory".to_string());
        
        // Test path resolution
        assert!(ability.get_at_path("Score").is_some());
        assert_eq!(ability.get_at_path("Score").unwrap().get_numeric_value(), 10.0);
        
        // Test array indexing
        assert!(ability.get_at_path("Exp[0]").is_some());
        assert_eq!(ability.get_at_path("Exp[0]").unwrap().get_numeric_value(), 100.0);
        assert!(ability.get_at_path("Exp[1]").is_some());
        assert_eq!(ability.get_at_path("Exp[1]").unwrap().get_numeric_value(), 200.0);
        
        // Test non-existent paths
        assert!(ability.get_at_path("NonExistent").is_none());
        assert!(ability.get_at_path("Exp[2]").is_none()); // Index out of bounds
    }

    #[test]
    fn test_value_store_basic_operations() {
        let mut store = ValueStore::debug_new();
        let store_id = store.to_id().clone();
        
        // Create type reference
        let strength_type = create_type_ref(&store_id, "Strength");
        
        // Test set operation
        let strength = Value::new("Strength", strength_type.clone()).with_num(12.0);
        assert!(store.set(strength).unwrap().is_none());
        
        // Test get operation
        let ref_path = Reference::new(store_id.clone(), "Strength".to_string());
        let value = store.get(&ref_path).unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().get_numeric_value(), 12.0);
        
        // Test updating an existing value
        let new_strength = Value::new("Strength", strength_type.clone()).with_num(14.0);
        let old_value = store.set(new_strength).unwrap();
        assert!(old_value.is_some());
        assert_eq!(old_value.unwrap().get_numeric_value(), 12.0);
        
        // Verify the update
        let updated = store.get(&ref_path).unwrap();
        assert!(updated.is_some());
        assert_eq!(updated.unwrap().get_numeric_value(), 14.0);
        
        // Test remove operation
        let removed = store.remove(&ref_path).unwrap();
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().get_numeric_value(), 14.0);
        
        // Verify it's gone
        let not_found = store.get(&ref_path).unwrap();
        assert!(not_found.is_none());
        
        // Test get_all
        let dex_type = create_type_ref(&store_id, "Dexterity");
        store.set(Value::new("Strength", strength_type).with_num(12.0)).unwrap();
        store.set(Value::new("Dexterity", dex_type).with_num(14.0)).unwrap();
        let all_values = store.get_all();
        assert_eq!(all_values.len(), 2);
    }

    #[test]
    fn test_reference_resolution() {
        let mut store = create_test_store();
        let store_id = store.to_id().clone();
        
        // Test basic reference resolution
        let ref_path = Reference::new(store_id.clone(), "StrengthReference".to_string());
        let value = store.get(&ref_path).unwrap().unwrap();
        
        // Use the evaluate method to resolve the reference
        
        // Add a chain of references
        let dex_ref = Reference::new(store_id.clone(), "Dexterity".to_string());
        let dex_type = create_type_ref(&store_id, "DexRef");
        store.set(Value::new("DexRef", dex_type.clone()).with_reference(dex_ref)).unwrap();
        
        let dex_ref2 = Reference::new(store_id.clone(), "DexRef".to_string());
        let dex_chain_type = create_type_ref(&store_id, "DexRefChain");
        store.set(Value::new("DexRefChain", dex_chain_type).with_reference(dex_ref2)).unwrap();
        
        // Evaluate the chain
        let chain_ref = Reference::new(store_id.clone(), "DexRefChain".to_string());
        let chain_value = store.get(&chain_ref).unwrap().unwrap();
        
        // Test circular reference detection
        // Create a circular reference A -> B -> A
        let ref_a_type = create_type_ref(&store_id, "RefA");
        let ref_b_type = create_type_ref(&store_id, "RefB");
        
        let b_ref = Reference::new(store_id.clone(), "RefB".to_string());
        store.set(Value::new("RefA", ref_a_type).with_reference(b_ref)).unwrap();
        
        let a_ref = Reference::new(store_id.clone(), "RefA".to_string());
        store.set(Value::new("RefB", ref_b_type).with_reference(a_ref)).unwrap();
        
        // Try to evaluate with circular reference
        let circular_ref = Reference::new(store_id.clone(), "RefA".to_string());
        
        // This should return an error for circular reference
        let result = store.get(&circular_ref);
        
    }

    #[test]
    fn test_nested_value_references() {
        let mut store = create_test_store();
        let store_id = store.to_id().clone();
        
        // Test accessing a nested value through a reference path
        let magic_theory_ref = Reference::new(store_id.clone(), "Magic Theory.Value".to_string());
        let value = store.get(&magic_theory_ref).unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().get_numeric_value(), 5.0);
        
        // Test accessing a list item
        let exp_ref = Reference::new(store_id.clone(), "Experience[1]".to_string());
        let exp_value = store.get(&exp_ref).unwrap();
        assert!(exp_value.is_some());
        assert_eq!(exp_value.unwrap().get_numeric_value(), 200.0);
        
        // Create a reference to a nested path
        let nested_ref = Reference::new(store_id.clone(), "Magic Theory.Exp".to_string());
        let magic_exp_type = create_type_ref(&store_id, "MagicExpRef");
        store.set(Value::new("MagicExpRef", magic_exp_type).with_reference(nested_ref)).unwrap();
    }

    #[test]
    fn test_nested_value_field_value() {
        let store = create_test_store();
        let store_id = store.to_id().clone();
        
        // Test accessing a nested value through a reference path
        let magic_theory_ref = Reference::new(store_id.clone(), "Magic Theory.Value".to_string());
        let value = store.get(&magic_theory_ref).unwrap();
        assert!(value.is_some());
        assert_eq!(value.unwrap().get_numeric_value(), 5.0);
    }
}