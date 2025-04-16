# RPG Helper Architecture
This file is used to extensively describe the architectural design of the project in high detail for use in implementation. A combination of text and diagrams (created using [ASCIIFlow](https://asciiflow.com/)) are used to describe the architecture.

## Database and Data Model
At the core of the architectural design is the Database design and the Data Model.

To make things abstract and easy to manage on the Database side, all the data we care about is stored as the all-encapsulating type of `Entity`. An `Entity` is accessed in the `Database` by an `EntityID`, which facilitates all CRUD operations. All entities share the same ID space (currently using the UUID system).
```
┌────────┐             ┌──────┐    
│Database◄──EntityID───┼Entity│    
└────────┘             └──────┘    
                   ┌─Entity─Types─┐
                   │ - User       │
                   │ - Container  │
                   │ - Store      │
                   │ - DBRecord   │
                   └──────────────┘
```

Entities can be the following:
- `User` type, which tracks all data about users of the project.
- `Container`, a type that "owns" or references `Store`s in the database
- `Store`, a type that holds the actual data that a user cares about for games, like types, values, locations, etc.
- `DatabaseRecord`, a singleton data struct that holds data we track for the database itself

*TODO: Define a permissions system design for the database*

The `User` type holds the data we care about for the user, which is currently:

```
                 ┌─────────────┐              
       ┌─────────►  User Data  ◄───────┐      
       │         └──────▲──────┘       │      
┌──────┼───────┐ ┌──────┼──────┐ ┌─────┼─────┐
│Secure Data   │ │Private Data │ │Public Data│
│--------------│ │-------------│ │-----------│
│Username      │ │StorageUsed  │ │ProfileData│
│Email         │ │Friends      │ └───────────┘
│Password      │ │Blocked      │              
│Admin         │ │Owned:       │              
│Verified      │ │- Games      │              
│DonationAmount│ │- Rulesets   │              
│MonthlyDonor  │ │- Settings   │              
└──────────────┘ │- Characters │              
                 └─────────────┘              
```


The `Container` type is used to facilitate the global addressability of the `Reference` type.
```
                                       ┌────────────────┐                    
┌────────────────┐ ┌────────────────┐  │Game            │  ┌────────────────┐
│Ruleset         │ │Setting         ◄─┐│----------------│  │Character       │
│----------------│ │----------------│ ││Owned:          │  │----------------│
│Owned:          │ │Owned:          │ ││- Type Store    │  │Owned:          │
│- Type Store    │ │- Type Store    │ ││- Wiki Store    │  │- Value Store   │
│- Wiki Store    │ │- Wiki Store    │ ││- Location Store│  │                │
│- Location Store│ │- Location Store│ ││- Map Store     │  │                │
│- Map Store     │ │- Map Store     │ ││- Timeline      │  │Reference:      │
└───────▲────────┘ │Reference:      │ ││Reference:      │  │- Required      │
        ├──────────│- Parent Ruleset│ └┼- Setting       │  │  Ruleset or    │
        │          └────────────────┘┌─┼- Ruleset       │  │  Setting       │
        └────────────────────────────┘┌┼- Characters    ◄──┼- Game          │
                                      │└────────────────┘  └──▲─────────────┘
                                      └───────────────────────┘              
```

The `Store` type ...


The client-server model for `Reference` resolution
```
                        ┌───────────┐                        
           Client       │Network API│     Server             
           ──────       └─────┬─────┘     ──────             
                              │                              
┌─────────┐ ┌─────────────┐   │  ┌────────────┐  ┌──────────┐
│Reference┼─► Data Handle ┼───┼──►API Handler ┼──► Database │
└─────────┘ │-------------│   │  │------------│  └──────────┘
            │  Cache      │   │  │ Validation │              
            └─────────────┘   │  │ Permission │              
                              │  │ DBReference│              
                              │  └────────────┘              
```