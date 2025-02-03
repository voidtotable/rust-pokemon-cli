---
name: {{ npc.name }}
description: {% for flavor in flavor %}{{ flavor }}{% endfor %}
level: {{ npc.level }}
health: {{ npc.health }}
armor: {{ npc.armor }}
damage: {{ npc.damage }}
movement: {{ npc.movement|display_some }}
interaction: {{ npc.interaction|display_some }}
loot: {{ npc.loot|display_some }}
motive: {{ npc.motive|display_some }}
types: {% for type in types %}
- {{ type }}{% endfor %}
abilities: {% for ability in abilities %}
- {{ ability.name }}{% endfor %}
moves: {% for move in moves %}
- {{ move.name }}{% endfor %}
---
# {{ npc.name }}

## Description

{% for flavor in flavor %}{{ flavor }}{% endfor %}

## Stats
|                   |                     |
| ----------------- | ------------------- |
| **Level:**        | {{ npc.level }}     |
| **Health:**       | {{ npc.health }}    |
| **Armor:**        | {{ npc.armor }}     |
| **Damage:**       | {{ npc.damage }}    |
| **Movement:**     | {{ npc.movement|display_some }}    |
| **Interaction:**  | {{ npc.interaction|display_some }}    |
| **Loot:**         | {{ npc.loot|display_some }}    |
| **Motive:**       | {{ npc.motive|display_some }}    |

## Types
{% for type in types %}
- {{ type }}{% endfor %}

## Abilities
{% for ability in abilities %}
- **{{ ability.name }}** - {{ ability.description }}{% endfor %}

## Moves
{% for move in moves %}
- **{{ move.name }}** - {{ move.description }}{% endfor %}
