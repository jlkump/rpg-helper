# RPG Helper Architecture
This file is used to extensively describe the architectural design of the project in high detail for use in implementation. A combination of text and diagrams (created using [ASCIIFlow](https://asciiflow.com/)) are used to describe the architecture. This document will not explain the reasoning behind design decisions or the implementation behind the API (at least in detail) as both may change. However, the core API and interaction between layers of the API should remain relatively stable.

The architecture for this project has several API layers that build ontop of one another, each layer introducing a further level of complexity and abstraction. Every layer interacts only with the layers below it in the API, thus the Data layer is the lowest layer that only interacts with itself. The RPG layer interacts with itself and the Data layer, and so on.

*The Parse layer interacts with all other layers and thus is adjacent from the hierarchy*

```
┌──────────┐          
│Database  ├─┐        
├──────────┤ │        
│Display   ├─┤        
├──────────┤ │        
│Permission├─┤        
├──────────┼ │┌──────┐
│Wiki      ├─┼►Parse │
├──────────┤ │└──────┘
│RPG       ├─┤        
├──────────┤ │        
│Data      ├─┘        
└──────────┘          
```

## Data Layer
The data layer operates on the following core abstractions of
- `Tag`: A string of alphanumeric characters separated by '.'s. A tag can not begin with a number. 
- `Attribute`: A floating point number and tag pair.
- `Equation`: Outputs a number based on the values contained within an input context.
- `Conditional`: Outputs a boolean based on the values contained within an input context.
- `Modifier`: Targets an `Attribute` or `Equation` to add to the end resultant value
- `Context`: A context is the core data struct that interaction with this API layer, holding all tags, attributes, equations, conditionals, and modifiers in a single struct. A context can be "layered" atop another context, combining values of both (and in the case of conflicts, taking the values of the "top" context in the layer).
- `Effect`: Changes the tags, attributes, equations, conditionals, or modifiers in a context

Each of the above values can also have a "Template," and intermediate value which is meant to be filled in later. Templates do not provide the functionality of their fully built counterparts.

The Data layer is the most abstract of all layers and only provides functionality in the way it is built upon. In essence, it is essentially a glorified HashMap with the ability to compute values based on other values in that HashMap.

`Tag`s are used at all layers of the API hierachy, while `Context`s are used primarily in the next layer, the RPG Layer.

## RPG Layer
The RPG Layer is what provides most of the core functionality this project sets out to achieve. Namely, the ability to make a `Ruleset` for a game, make a `Character` based on a `Ruleset`, and then track changes (`Event`s) to `Character`s in a `Timeline`. Further, a `Timeline` allows for the ability to view a `Character` at different `Date`s in the timeline and have their `Character`'s values adjusted accordingly.

Beyond this core functionality, the RPG Layer provides further abstractions to track data about a `Character` for a game, which are described in the following sections.

### `Abilities`
Abilities can be thought of as changes to the `Character`'s Context (both when the Ability is owned and while it is "Active") as well as containing their own Sub-Context containing data to compute about itself. It also allows for user input to change the state of the ability and thus the player as well.

An example of an Ability would be Spells in Ars Magica, as they each have their own Magnitude, Range, Target, Duration, etc. Another example would be Virtues and Flaws from Ars Magica. (Some Virtues and Flaws are purely aesthetic and thus only contain text. Text is not handled in the RPG Layer and is instead handled in the Wiki Layer above this one).

### `Item`s and `Inventory`
`Item`s are similar to abilities, however they are represent some resource or crafted item. Resources can be used up for `Event`s and crafted items can come from the product of `Event`s.

Items are still a work in progress, as I am not 100% sure what their requirements will entail. They are mainly meant to serve as a thing to be tracked that can be exchanged between players, used for events, activated to produce effects similar to abilities, equiped to modify the character, and perhaps other effects.

### `Location`s and `Map`s
`Location`s are defined by a `Ruleset` (or a `Setting` / `Game`) and are used to provide additional `Event` options to players. For example, a player may be able to perform Laboratory activities only in their Laboratory. Additionally, a location provides some contextual information to be used for the `Character`'s context and the context of an `Event`. For example, the `Aura` of a location. (It is important to provide a default location as well as the ability for the user or game master to create "Temporary" locations from a template when a simple temporary location is needed for combat or special one-off event locations).

### `InputAction`s
Used to communicate user input / desired input requirements and responses.

### Dice
The dice struct allows for the definition of a type of dice (Ex: Stress Die), which has a number of sides and some number of side modifiers. This is used to define things such as exploding dice or simple die rolls. The system facilitates both user inputed rolls as well as computed calculated rolls.

## Wiki Layer
The wiki layer provides text data associated with everything in the RPG Layer, as well as the ability to make Wiki `Page`s, which act very similarly to the Obsidian note taking app's notes. Pages can link to each other, as well as display values of a Character or NPC in real-time based on the current active date of a game. For Ars Magica, this is particularly useful for defining NPCs who age over time as well as for creating Towns that change over time.

## Display Layer
The display layer provides the ability for a user to define how they want the layout of a character sheet, GM sheet, or any other game data should look. This also includes creating custom themes as well as doing some slight modification on the style of containers (border thickness, font, and border rounding).

## Permission Layer
The permission layer is what provides the idea of a User and how we determine what a User can see. An idea of a permission wraps the values in the RPG and Wiki layer by either a Player-level permission check or a User-level permission check. It is important that these remain separate checks, as there could be severe permission problems for User permission escalation otherwise.

The permission layer also wraps values in the Display layer, but only by a User level permission.

## Database Layer
The database layer is how everything is actually stored to a database. This is provided as an abstract API that is meant to be implemented on the server-side of things. It also provides a way for the client to interface with the database (maybe).

## Parse Layer
A farily simple layer. This layer is used to serialize the data of each layer to some output file type. Currently, JSON is the only file type planned. (This allows users the ability to download their characters and have them for safekeeping. Since this is a public, open source project, they can then use that saved character data on their own variations of the program).