# RPG Helper Architecture
This file is used to extensively describe the architectural design of the project in high detail for use in implementation. A combination of text and diagrams (created using [ASCIIFlow](https://asciiflow.com/)) are used to describe the architecture. This document will not explain the reasoning behind design decisions or the implementation behind the API (at least in detail) as both may change. However, the core API and interaction between layers of the API should remain relatively stable.

The architecture for this project has several API layers that build ontop of one another, each layer introducing a further level of complexity and abstraction. Every layer interacts only with the layers below it in the API, thus the Data layer is the lowest layer that only interacts with itself. The RPG layer interacts with itself and the Data layer, and so on.

*The Parse layer interacts with all other layers and thus is adjacent from the hierarchy*

```
┌────────┐          
│Database┼─┐        
├────────┤ │        
│Display ├─┤        
├────────┤ │┌──────┐
│Wiki    ├─┼►Parse │
├────────┤ │└──────┘
│RPG     ├─┤        
├────────┤ │        
│Data    ├─┘        
└────────┘          
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

### `Location`s and `Map`s


### `InputAction`s
Used to communicate user input / desired input requirements and responses.


## Wiki Layer

## Display Layer

## Database Layer

## Parse Layer