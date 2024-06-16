pub mod character_creator;
pub mod character_viewer;
pub mod dashboard;
pub mod home;
pub mod page_not_found;
pub mod ruleset_creator;
pub mod setting_editor;
pub mod user;

// TODO: Define a GM screen which looks like a wiki display
// that will allow the GM to jump between links and edit the wiki live.
// The GM should be able to edit the display of the wiki in a Markdown-esque syntax.
// EX:
// ## Description
// > [!infobox] /* Defines that the following lines with > are part of a side-panel info-box*/
// > # General Zaas Thaeran /* A header */
// > ![[General Zaas.png|cover hsmall]]  /* An image with the path to the image then the customization */
// > ###### Stats /* Header */
// > | Stat | Value | /* Defines a table: Defines the headers */
// > | ---- | ---- | /* Defines the number of columns in the table */
// > | **Magic Might** | 25 (Mentem) |  /* Defines a row of the table */ TODO: Define a syntax for displaying a character's data in a table through Markdown syntax
// > | **Size** | 0 | 
// > | **Soak** | n/a | 
// > ###### Characteristics
// > | Type | Value |
// > | ---- | ---- |
// > | **Intelligence** | +1 |
// > | **Perception** | +1 |
// > | **Presence** | +3 |
// > | **Communication** | +2 |
// > | **Strength** | 0 |
// > | **Stamina** | 0 |
// > | **Dexterity** | 0 |
// > | **Quickness** | +2 |

// Displaying character's info in a table?
// [!display] characteristics

// Display an individual value?
// [!display] Intelligence
// [!display] Magic Theory
// [!display] Casting Score

// Inline a description from elsewhere
// For example, in-line the description of an attack / spell for a generic creature attack
// **Kinesis:** [!inline] Kinesis
// Displays:
// Kinesis (bolded) (5 points, init 0, Terram): The creature can interact and use objects at a range ...

mod sheets;