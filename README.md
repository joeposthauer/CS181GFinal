# Final Project CS 181G (Game Engine Programming)
Authors: Ayelet Kleinerman & Joe Posthauer
Professor: Joseph Osborn

## General Description
This project is final project for the class. It includes 3 games and a game engine behind them.
The three games are a versions of Snake, Goldminer, and Tron (called adventure) in the 
Provide a brief description of what your project does and its purpose.

## Project File Description and Flow
### Engine
The engine is aimed to be a modular game engine designed to support various types of games, providing essential functionalities like grid and level management.
#### Key Files
- grid.rs: Handles the grid system of the games
- level.rs: Manages the loading and parsing of levels
- lib.rs: Core library file including basic structs, enums, and implementations such as Vec2, Dir, Rect, EntityType and others.

### Adventure
Adventure is actually the Tron game. This game is a two players game, where the goal is to get the other player to crash into you.
#### Key Files
- level.txt: Level configuration file
- tilesheet.png: Tilesheet used for the game environment
- main.rs: the main functionality of the Tron game, which includes the implementation of the functions for new game intialization, render and simulate, as well the players movement, collision detection, and lengthening of track behind player.

### Goldminer
A game where players collect objects, where different objects have different values, using a craw that extendens from the top, which a goal of reaching some amount of value to reach the next level.

#### Key Files
- Goldminer_tilesheet1.png: Tilesheet for the environment
- level.txt: Level configuration file
- main.rs: the main functionality of the Goldminer game, which includes implementations of the fuctions for new game intialization, render and simulate, as well the craw rotation, extentions, and retraction; collision detection; object gathering; scores counting; and a timer.
 
### Snake
A classic snake game with custom textures.

#### Key Files
- level.txt: Level configuration file
- tilesheet.png: Tilesheet used for the game environment
- main.rs: the main functionality of the Snake game, which includes the implementation of the functions for new game intialization, render and simulate, as well the snake movement, collision detection, creation of apples, lengthening of the snake.

## Modules
this project uses the frenderer crate extensevily, (https://github.com/JoeOsborn/frenderer)

## Games Screenshots
### Adventure
https://github.com/github/joeposthauer/CS181GFinal/Adventure/Tron.png
### Goldminer
https://github.com/github/joeposthauer/CS181GFinal/Goldminer/Goldminer.png
### Snake
https://github.com/github/joeposthauer/CS181GFinal/Snake/Snake.png

## Future Steps




 

