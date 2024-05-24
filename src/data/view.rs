use self::data_view::DataView;

pub(super) mod data_view;
pub(super) mod instance_view;

pub enum ActiveView<'a> {
    Character(&'a DataView<'a>),
    GameMaster(&'a DataView<'a>)
}