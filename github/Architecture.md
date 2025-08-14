# RPG Helper Architecture
This file is used to extensively describe the architectural design of the project in high detail for use in implementation. A combination of text and diagrams (created using [ASCIIFlow](https://asciiflow.com/)) are used to describe the architecture.

The architecture for this project has layers that build ontop of one another, each layer introducing a further level of complexity and abstraction.

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

*The Parse layer interacts with all other layers and thus is adjacent from the hierarchy*

## Data Layer
The data layer operates on the following core abstractions of
- `Tag`: A string of alphanumeric characters separated by '.'s. A tag can not begin with a number. 
- `Attribute`: A floating point number and tag pair.
- `Equation`: Outputs a number based on the values contained within an input context.
- `Conditional`: Outputs a boolean based on the values contained within an input context.
- `Modifier`: Targets an `Attribute` or `Equation` to add to the end resultant value
- `Context`: A context is the core data struct that interaction with this API layer, holding all tags, attributes, equations, conditionals, and modifiers in a single struct. A context can be "layered" atop another context, combining values of both (and in the case of conflicts, taking the values of the "top" context in the layer).
- `Effect`: Changes the tags, attributes, equations, conditionals, or modifiers in a context

## RPG Layer

## Wiki Layer

## Display Layer

## Database Layer

## Parse Layer