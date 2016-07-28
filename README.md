Cards UI (WIP)
========

A card game UI library written in Rust.

TODO
----
* Switch back to mainline conrod (depends on https://github.com/PistonDevelopers/conrod/issues/725)
* Stacked deck
* Card picker
* Layout (rows/columns)


Design
------
For the layout, here is my current vision:
Matrix of Stacks
Where a Stack is a collection of Cards + appearance(expanded or not, flipped or not)
Each Card can be face up or face down and has a Value and a Suit

Cards can then be moved between stacks (how exactly is to be investigated)

Credits
-------
* UI framework: Conrod from the great PistonDevelopers
* Simple cards abstraction: cards-rs from th4t
* Card deck: assets from Monika Ratan
