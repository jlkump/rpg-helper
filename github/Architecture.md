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

The `Database` trait is defined in `src/database.rs` and abstracts away the actual implementation of storage in the database.
Within the `src/database/imp` module are all the different implementations of the databases. Currently, this only contains
the Rust Sled database, which is a integrated database (meaning all the data of the sever is stored locally on the same server).

Any `Entity` stored in the Database must implement the `DatabaseEntity<B>` and `DatabaseMutator<D, B>` traits, which ensure that
entities know how to insert themselves in the database. This essentially gives the Entities the ability to perform
procedures before or after the database operations performed on them.

As such, the interface methods defined on the `Database` trait should only be called by the implmentations of `DatabaseMutator`
to avoid situations where the database is modified incorrectly.

### User
The `User` type holds the data we care about for the user, which is organized into Secure, Private, and Public data.

**Secure Data**
This is data that is only display in the authorized user settings panel. It is not accessed anywhere else. 
The user can only change their email and password directly.
- Username
- Email
- Password: Stored as hashed and salted string using Bycrpt algorithm
- Admin: A bool marking whether this user has admin privileges to access an admin panel
- Verified: A bool marking whether this user has verified their email
- Donation Amount: A uint64 tracking the amount of money donated in cents by the user (comes from the external Kofi API)
- Monthly Donor: Whether this user is an active monthly donator

**Private Data**
- Storage Used: The number of bytes this user has used of their storage in the database.
- Friends: The IDs of the other users this user is friends with
- Blocked: The IDs of the other users this user has blocked (meaning no friend requests or game invites)
- Owned Games / Rulesets / Settings / Characters: The list by ID of the Games, Rulesets, Settings, and Characters this user has ownership of.
- Accessed Games / Rulesets / Settings / Characters: The list by ID of the Games, Rulesets, Settings, and Characters this user has the ability to view or view and edit.

**Public Data**
All the profile data of the user displayed publically to other users.
- Profile Name
- Profile Catchphrase: The subtitle text of the user underneath their profile name. (Optional)
- Profile Biography: An optional profile biography for display on the user's profile page.
- Profile Image: The image used to represent the user. Defaults to [Default Profile](/assets/profile-defaults/Profile.png)
- Profile Banner: The image used as the background of the user's profile page. Defaults to [Default Banner](/assets/profile-defaults/Banner.png)

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

### Container
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

### Store
The `Store` type encapsulates all the data of the actual achitectural system.
These are the `Store`s that exist:
- Event Store (The timeline)
- Location Store
- Map Store
- Type Store
- Value Store
- Wiki Store

The `Store`s are quite complex and require extensive detail for each individual store. As such, the bottom of this document will go over each individual store in the detail required. 

## Using the Library
Thinking about how we want the CLI, Client, and Server to use the architecture defined in the library module, we want the root access to all actions to come from the `DataHandle` implementation defined in `src/core.rs`. The `DataHandle` will abstract away access to the data underlying the model. In particular, we don't want a user of the library to have to care about manipulating the `Entity` type. Rather, the `DataHandle` will return builders and the data of the model without the Entity types needed.

For an example, the web-client will have a Data Handle for a logged-in user. This Data Handle will abstract away the Network API calls to the backend server, allowing the client to care just about the data model of the Rulesets, Settings, Games, and Characters and the data stored within them. The web-client code should never have to care about accessing EntityIDs or parsing Entity data. Instead, the code interacts with the data model through the DataHandle to perform the desired operations.

```
      ┌─────────────────┐                            
      │Abstraction Layer│                            
      └────────┬┬───────┘                            
┌───────────┐  ││    ┌───────────┐  ┌───────────────┐
│Data Handle┼──┼┼───►│Network API┼─►│Server Database│
└───────────┘  ││    └───────────┘  └───────────────┘        
```

This core role of the `DataHandle` type means it will be the most extensive trait with details specific for each Container and Store that exists. This will make the implementation quite large, but as this interface type will provide essentially all the interaction with the data model for the project, this is expected and designed.

An added benefit of creating this abstraction layer with the `DataHandle` type is that we can create different interaction types with the data model. For example, the CLI program can have the ability to access the remote server data base through the network API or it can just access a local database. In either case, all that needs to change is the `DataHandle` being used, making the system very modular in design.

```
      ┌─────────────────┐                            
      │Abstraction Layer│                            
      └────────┬┬───────┘                            
┌───────────┐  ││    ┌───────────┐  ┌───────────────┐
│Data Handle┼──┼┼─┬─►│Network API┼─►│Server Database│
└───────────┘  ││ │  └───────────┘  └───────────────┘
               ││ │  ┌──────────────┐                
               ││ └─►│Local Database│                
               ││    └──────────────┘                
```

## Stores in Detail
Something important to note for all stores is that each have a permission system on a scale of `NoAccess`, `ReadAccess`, and `ReadWriteAccess`. This is different from the permission system for getting the data of important stuff, like user private data or modifying entities. This is instead used to keep things private from the players in a game as the Game Master.

### Event Store
The event store will store all the events on the timeline of events for a game.

What is required for an event?
- A time-stamp for the event based on the in-setting defined calendar
      - This then requires that a ruleset define a date type
      - A date type requires only that there be a way to evaluate it into a int value to compare and order
- An id to identify it uniquely
- The modification(s) it has on character(s)

A base event type is required to be defined for a ruleset in addition to a base date type. The event type for Ars Magica might be defined as the following:

```
Event:
{
      Date,
      ID,
      Effect,
}
```

Activities can then be defined in a Ruleset as pre-sets to events. Activities can be tied to locations or can be globably available. They can also be restricted based on the Values of a character.

I.E, the `DataHandle` takes in a `Character` and returns the available list of `Activities` for that `Character`.

By using a activity, an event is created from that activity to have affect on the designated character.

Once an event is added to the timeline, the character it affects is updated accordingly. This is the ONLY way to modify a character past character creation (aside from debugging tools). Likewise, if an event is edited or removed, the character it affects is updated accordingly. For events that are edited or removed, the character's values may have been referenced by other characters and used in their own events (for example, think of the Teaching score in Ars Magica, which gives another player character's Exp based on the abilities of original player character's communication score). Thus, a dependency graph is used and tracked in each event to know what `Value`s from a `Character` used the value and how it changed. This is used to propigate the change to older events up the chain.

The available activities for a `Character` is determined by their location field. The field may be empty / unknown in which case the available locations default to just the global locations.

**Data Handle interface for Event Store**
This is a general outline for the interface methods on a `DataHandle` which is performs the functionality described above. This outline does not define all the complexity of the actual implementation (such as error type returns or all the required parameters) but is instead to give the idea of the interface to be designed.


```rust
// This assumes a given Game context for a DataHandle
// Thus, how to handle a Game context is yet to be decided.
// I think it should probably be through a `pub fn set_game_context(&mut self, GameID)`

pub fn get_available_activities(&self, c: &Character) -> Vec<Activtiy>; // Get the activities available for a character to perform

pub fn get_all_activities(&self) -> Vec<Activity>; // Get all the types a given player can see. A player's view context is defined by the DataHandle

pub fn get_timeline(&self) -> Timeline; // Timeline. Timeline should be filterable to show certain player events or global events.

pub fn get_current_date(&self) -> Date; // The current date that the players are at in the timeline.

pub fn get_date_limit(&self) -> Date; // The date up to which the player can create events for.

pub fn set_display_date(&self, d: Date); // Sets the date for the player locally for them to see their character at different points in the timeline.

pub fn get_display_date(&self) -> Date;

pub fn insert_event(&self, event: EventBuilder);

pub fn update_event(&self, event_id: EventID, event: EventBuilder);

pub fn remove_event(&self, event_id: EventID);
```

### Location Store
A location can be defined by the user. The required fields for a location are the following:
- Name
- Optional Wiki Note link (The displayed description)
- List of available Activities

Locations come in a Heirarchy defined as the following:
- Region
- Sector
- Locale
- Landmark

A Landmark can be contained by a Locale can be contained by a Sector can be contained by a Region. A player character's location is defined at any level of specificity along this scale.

### Map Store
A map can be added to any Ruleset, Setting, or Game. A map is simply a 2D image with a name and ID. It also can contain a list of pins and a list of regions. Regions are defined by a set of points defining a polygon on the map in which a location or locations is defined. A pin is a single point on the map that defines a location or locations.

### Type Store
Types define what types exist to be used to define Values in a character. They define the required fields and the ways in which values can be computed to a numeric or boolean value for execution and display.

### Value Store
Values exist only for Characters and are used for evaluation in equations and display computation.

### Wiki Store
A Wiki store contains Wiki Notes, which are simply Markdown documents displayed in a nice wiki style format to the client. Wiki Notes can link to other Wiki Notes (like in Obsidian) or can display the Values of a Character.