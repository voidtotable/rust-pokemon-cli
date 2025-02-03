---
name: {{ npc.name }}
description:
level: {{ npc.level }}
health: {{ npc.health }}
armor: {{ npc.armor }}
damage: {{ npc.damage }}
movement: 
interaction: 
loot: 
motive: 
abilities: 
moves:
---
# {{ npc.name }}

## Description

Description

## Stats
|                   |                     |
| ----------------- | ------------------- |
| **Level:**        | {{ npc.level }}     |
| **Health:**       | {{ npc.health }}    |
| **Armor:**        | {{ npc.armor }}     |
| **Damage:**       | {{ npc.damage }}    |
| **Movement:**     |     |
| **Interaction:**  |     |
| **Loot:**         |     |
| **Motive:**       |     |

## Abilities
{% for ability in abilities %}
- **{{ ability.name }}** - {{ ability.description }}{% endfor %}


## Moves
{% for move in moves %}
- **{{ move.name }}** - {{ move.description }}{% endfor %}
