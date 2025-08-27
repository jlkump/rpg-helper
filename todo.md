# Finish RPG Layer Functionality

## Character and events
- [x] Complete template tags
- [x] Complete template equations / conditionals
- [x] Create template ctx, which can contain a ctx as well as template values, tags, equations, conditonals, modifiers, etc.
            - The template ctx is not used for characters or rulesets, but for event schemas (as well as other schemas), to ease filling in values of an intermediate context.
- [x] Create template attributes
- [x] Create template modifiers
- [ ] Template effects? Nah, effects can be built from templated values where needed.
            - Actually maybe. It might be useful to have a clearly defined template effect data type
- [ ] Complete a simple Date and Date Schema creator
    - This will ensure the design for basic schemas works.
    - [x] Style chosen for input
- [ ] Create events from event schema

## Rulesets
- [ ] Define possible event schemas
- [ ] Define Date format
- [ ] Define event interval
- [ ] Define character templates
    - [ ] What values exist on the character
- [ ] Ruleset context
    - [ ] Default equations (can be derived from template)
    - [ ] 

## Create basic Ruleset Editor
- [ ] Editor reads active ruleset settings
- [ ] Editor allows changing date-format, name (which is meta-data), event interval, and event schemas

## Create basic Character editor
- [ ] Display raw data of ctx (doesn't need to be neatly formatted yet)
- [ ] Display timeline events neatly (will be missing wiki display data, so just display raw tag data)
