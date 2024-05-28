// The game context will provide the information shared to all players of the game.
// Most importantly, this will be what types there are and how they are displayed,
// what sheets there are and how the sheets are displayed.

// For example, in Ars Magica, there is a Magus Character Display, which contains 
// the character details, the timeline, the laboratory sheet, etc

// The player can determine the order of these sheets, which is held in the character context

// Another game might not need the timeline sheet, so it will not be provided, but may want another
// sheet and might define a custom sheet for the player to have access to. Custom sheets will
// be more limited in functionality than built-in sheets like the timeline or inventory, but
// are instead built out of the components of existing pieces, like a table, an item display, etc.