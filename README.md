Cards UI (WIP)
========

A card game UI library written in Rust.

TODO
----
* Button with texture widget for cards (that I can click to flip/ drag and drop to move)
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
