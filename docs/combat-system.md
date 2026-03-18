# Menu-Based Combat System

## Overview

Combat is triggered by encounters on the overworld. The game transitions to a dedicated battle screen where the player and enemies take turns selecting actions from menus.

## State Machine

```mermaid
stateDiagram-v2
    [*] --> PlayerTurn
    PlayerTurn --> PlayerActionMenu: show menu
    PlayerActionMenu --> MoveSelect: Attack
    PlayerActionMenu --> ItemSelect: Item
    PlayerActionMenu --> RunAttempt: Run
    MoveSelect --> EnemyTurn: move chosen
    ItemSelect --> EnemyTurn: item used
    RunAttempt --> EnemyTurn: failed
    RunAttempt --> BattleEnd: escaped
    EnemyTurn --> EnemyAction: AI picks action
    EnemyAction --> PlayerTurn: player alive
    EnemyAction --> BattleEnd: player defeated
    EnemyTurn --> BattleEnd: enemy defeated
    BattleEnd --> [*]
```

## Screen Layout

```
+------------------------------------------+
|                                          |
|   Enemy Name        HP [========--]      |
|   Enemy Sprite                           |
|                                          |
|                                          |
|                                          |
|   Player Name       HP [==========]      |
|                     MP [======----]      |
|                                          |
+------------------------------------------+
|  > Attack     Item                       |
|    Defend     Run                        |
+------------------------------------------+
```

## Core Components

```mermaid
classDiagram
    class BattleScreen {
        state: BattleState
        player: Combatant
        enemy: Combatant
        menu: Menu
        message_queue: Vec~String~
        update(input, elapsed)
        produce_frame()
    }

    class Combatant {
        name: String
        hp: u32
        max_hp: u32
        attack: u32
        defense: u32
        speed: u32
        moves: Vec~Move~
        is_alive() bool
    }

    class Move {
        name: String
        power: u32
        move_type: MoveType
    }

    class MoveType {
        <<enumeration>>
        Physical
        Special
    }

    class BattleState {
        <<enumeration>>
        PlayerTurn
        MoveSelect
        ItemSelect
        EnemyTurn
        Animating
        Message
        BattleEnd
    }

    class Menu {
        options: Vec~String~
        cursor: usize
        move_up()
        move_down()
        select() usize
    }

    BattleScreen --> Combatant
    BattleScreen --> BattleState
    BattleScreen --> Menu
    Combatant --> Move
    Move --> MoveType
```

## Turn Flow

```mermaid
sequenceDiagram
    participant P as Player
    participant B as BattleScreen
    participant E as Enemy AI

    B->>P: Show action menu
    P->>B: Select "Attack"
    B->>P: Show move list
    P->>B: Select move
    B->>B: Calculate damage
    B->>B: Queue message "Player used Slash!"
    B->>B: Queue message "Enemy took 12 damage!"
    B->>B: Display messages (wait for input)
    alt Enemy alive
        B->>E: Request action
        E->>B: Choose move
        B->>B: Calculate damage
        B->>B: Queue message "Enemy used Bite!"
        B->>B: Display messages
        B->>P: Back to action menu
    else Enemy defeated
        B->>B: Queue message "Enemy fainted!"
        B->>B: End battle, return to overworld
    end
```

## Damage Formula

```
damage = ((2 * power * (attacker.attack / defender.defense)) / 5) + 2
```

A simple formula inspired by Pokemon gen 1. Can be extended later with type effectiveness, critical hits, and random variance.

## Integration with Existing Architecture

```mermaid
flowchart LR
    OW[Overworld Screen] -->|encounter| BS[BattleScreen::new]
    BS -->|battle end| OW
    BS -.->|implements| Screen[Screen trait]
    OW -.->|implements| Screen
```

`BattleScreen` implements the existing `Screen` trait. The overworld triggers an encounter and swaps the active screen to `BattleScreen`. When combat ends, it swaps back. The `run_game` loop needs no changes — it already drives whatever `Box<dyn Screen>` is active.

## Message Queue

Rather than showing all combat results instantly, messages are queued and displayed one at a time. The player presses a key to advance to the next message. This controls pacing and makes combat readable.

States like `BattleState::Message` consume from the queue, showing one message per frame, and advance to the next state (enemy turn, player turn, or battle end) when the queue is empty.
