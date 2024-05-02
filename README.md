 # Final Project CS 181G (Game Engine Programming)
<u>Authors<u>: Ayelet Kleinerman & Joe Posthauer

<u>Professor<u>: Joseph Osborn

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
This project uses the frenderer crate extensevily, (https://github.com/JoeOsborn/frenderer)
It is based on tile maps and sheet regions to represent all objects, maps, and players.

## Games Screenshots
### Adventure
![Adventure Screenshot](https://github.com/joeposthauer/CS181GFinal/blob/main/adventure/Tron.png?raw=true)
### Goldminer
![Goldminer Screenshot](https://github.com/joeposthauer/CS181GFinal/blob/main/goldminer/Goldminer.png?raw=true)
### Snake
![Snake Screenshot](https://github.com/joeposthauer/CS181GFinal/blob/main/snake/Snake.png?raw=true)

## Future Steps
This project is not final, and more work is required to get it to be done.
There should be more changes with the engine, moving more shared structs and functions from the main.rs files to it.
Some functionality need to be changed to make the engine more versetile (such as changing EntityTypes themeselves to be in main.rs and not in engine)
Snake may be changed to work on tile size instead of pixels so the snake would not be able to hit only part of the apple (which currently do not count), it may also be changed to have a score, additional levels and complexity (adding obsticles), have graphics of snake head and body.
Tron should have different graphic which will be closer visually to the original Tron game, it should also have Game Over screen and who won, and additional features that can help players when picked up such as increase/decrease speed.
Goldminer needs the most work - increase size of objects, finish functionality to drag objects, add score counter, add different values to different objects, add a goal score for the level, add more levels, add a timer for each level, add obstacles, add diffrent object the player can buy to help them in the game.
 
## Acknowledgments
We thank professor Osborn for his mentorship throughout the class and while creating these games.
We thank each other for the cooperation, knowledge, and support thoughout the creation of this project.
