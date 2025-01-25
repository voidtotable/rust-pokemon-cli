# rust-pokemon-cli

A CLI tool using the PokeApi used to generate tables and stats blocks for tabletop roleplaying games (TTRPGs). 

This project is initially focused on stat blocks for the [Cypher System](https://cypher-system.com/), and [Old School Renaisance (OSR)](https://en.wikipedia.org/wiki/Old_School_Renaissance) TTRPGS.

## Use Case and Features

I am running a homebrew hexcrawl TTRPG campaign loosely based on Pokemon using the Cypher System. I want to be able to easily generate random encounter tables based on the general geography of an area using existing Pokemon data. 

There are a plethora of online resources for Pokemon such as [Bulbapedia](https://bulbapedia.bulbagarden.net/wiki/Main_Page), but given a plethora of generations and overwhelming amount of information for each Pokemon, decision fatigue becomes a problem with the amount of choices you need to make when converting from a Pokemon wiki page to a simple statblock.

### Random Encounter Tables
My most common use case will be entering in one or more Pokemon types to the CLI, and then getting a table of Pokemon matching those types with some quick information on them such as their flavor text, abilities and moves.

### Statblocks
I'd like to be able to then use that table to generate "stat blocks" or detailed pages with more verbose information on the Pokemon listed in that table.

The Cypher System has a very simple NPC stats, so I can ignore a lot of the numeric stats and focus on the flavor text. I'll want an algorithm to help choose a small set of moves for each statblock as I only need 3-5 moves at most. I want some randomness in this process which will give me numerous variants on a single Pokemon.

### Quick Reference
I'd like the CLI to provide a quick reference for information such as type strengths/weaknesses, moves, etc. This could be very useful when running a game and I need to look something up, and will save a lot over going to a reference wiki.

### Output to Markdown
Markdown editors such as Obsidian are growing in popularity and is what I personally use for my session notes. I want to be able to either generate markdown files I can import into my editor of choice, or print markdown that I can copy/paste from the terminal into my editor. I'd also like to be able to support some extended markdown features if the editor is specified on the CLI.

### Example Output

| Name      | Description                                                                                                                                  | Types        | Abilities             | Moves                            |
|-----------|----------------------------------------------------------------------------------------------------------------------------------------------|--------------|-----------------------|----------------------------------|
| Bulbasaur | While it is young, it uses the nutrients that are stored in the seed on its back in order to grow.                                           | grass,poison | growl,tackle          | body slam,sleep powder,take down |
| Grookey   | When it uses its special stick to strike up a beat,the sound waves produced carry revitalizing energy to the plants and flowers in the area. | grass        | overgrow,grassy surge | scratch,slam,swords dance        |
|           |                                                                                                                                              |              |                       |                                  |


