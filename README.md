# RPG Helper
## Overview
This a project meant to help organize the complexity of various tabletop RPGs into a wiki-like, note-taking format, similar to Obsidian, while also managing the complex changing stats of characters throughout gameplay. This project was initially built for the Ars Magica TTRPG, but in building for the complexity of that game, I found that the tool could be used in just about any TTRPG.

The project is built in Rust, using Yew for the front-end web-client and Actix for the backend server with Sled as the integrated database. The API iterface for the project allows swapping out these specific implementations in the future if needed. 

## Core Features
A user can login to their account to create *Rulesets* and *Settings* to define the particular rules of an RPG. *Settings* can be thought of as a layer on top of a *Ruleset* which overrides existing rules or adds new unique rules. A user can then create a *Game*, which derives from either a *Ruleset* or *Setting*, and can invite other users to the *Game*. 

A user in the *Game* is given permision to act as Game Master and each user in the *Game* can create *Characters* for playing in the *Game*. The Game Master will be able to view all user's *Characters* in a *Game* and will have additional tools for taking *Wiki* notes, creating NPC *Characters*, creating and managing *Locations* and *Maps*, and editing the game *Timeline*.

The most unique feature of this project is the *Timeline*, which manages all changes made to all *Characters* in a *Game*. Every change to a *Character* in a *Game* is tracked by *Events* on the *Timeline*. *Events* can vary in scale to a small xp boost to a series of additional character abilities. The primary feature of the *Timeline* is that a user can edit their older events and have their changes cascade down the *Timeline*.

*Locations* aim to restrict the available event types, or *Activities*, that a *Character* can perform on the *Timeline*. For example, in Ars Magica, a player can only enchant items while in their laboratory. Thus, while the player is in some different location, such as in the city of a distant nation, they could not perform the echantment activity. Likewise, some special *Activities* may only be available in specific *Locations*.

A *Map* is meant to simply display *Locations*. The same *Location* can exist on multiple different *Maps* (for the cases of maps of various detail and specificity).

The Game Master can create changes to the rules, add new *Locations*, and create new *Wiki* notes for a *Game*. At some point, the Game Master may wish to add some of the changes from the *Game* to the *Setting*. In this case, if the Game Master has permision to edit the *Setting* in question, they can simply apply those changes. Otherwise, they can create a copy for themselves that adds those new changes.

The *Wiki* of a *Ruleset*, *Setting*, and *Game* provide note-taking functionality similar to that of Obsidian, where the user writes down notes in the Markdown format with some extended functionality for displaying things aesthetically in a *Wiki* format. All notes are simply stored as Markdown and parsed to provide extended functionality, such as hyper-links between notes, image display, and stat display.

## Architecture

### Data Model

## Contribution
### Getting Started
If you would like to contribute to this project, clone this repo and download the Rust toolchain.

The project's front-end uses Trunk, a Rust package that runs the Yew web client locally. Other than that, the Cargo.toml file should handle all dependencies.

### Project Structure
`github` contains files specific just to GitHub and nothing important to the code of the project.

`old-client-code` contains the old client code that was used before the swap to a unified project structure. It remains as a reference for future implementation of the client code and will be removed once the web client is fully implemented.

`src` contains all the code of the project detailed in the tree below.

`src/bin`: Code for the specific binaries which are the implementations that use the library api defined in `src/lib.rs`
`src/model`: Code for the [Data Model](). Essentially all the library code to define the API used in the project.

### Coding Standards


### Roadmap
See the [Roadmap](./github/Roadmap.md) file to see a detailed outline of each planned feature.

See the [TODO](./TODO.md) file to see what is actively being worked on.

## License
[GNU General Public License v3](./LICENSE)

## Authors
Landon Kump - Programming, Design, and Art

![Example Wiki Image](./github/imgs/Creature%20Screen%20Wiki.png)