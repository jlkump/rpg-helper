use rpg_helper::model::database::Database;

use crate::data::{EditorType, ProgramData};



mod types;
mod typestore;

pub fn execute_editor<D: Database>(editor_type: EditorType, parts: Vec<&str>, data: &mut ProgramData<D>) -> Result<String, String>
{
    match editor_type
    {
        EditorType::TypeStore(id) => typestore::execute(id, parts, data),
        EditorType::Type(id, _) => types::execute(id, parts, data),
    }
}
