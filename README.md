# Runity

## Runity explained

### What is it?

runity is a rust to unity interface library. Currently, it doesn't reimplement every unity function and quirk, but it is a goal. It aims to act as an alterative scripting language than c# in unity, and can be used for everything from modding to regular scripting.

### How does it work?

runity works by implementing unity datatypes, functions and more into an interface callable by rust. The script written in rust is then compiled into a dynamic library, which is loaded into a unity application at runtime. This allows for rust to be used for modding and scripting support, or to be used as the whole backbone of your game.

## How do I use it?

Please look at the ![examples](https://github.com/dimitribobkov/runity/tree/master/examples) to see a simple example of usage. 


Your library must be a `["cdylib"]` type. Please look at the example toml file for a base to start off.

![image](https://user-images.githubusercontent.com/30769396/116154577-bbe27e00-a6e0-11eb-8e8a-3f0b19771ba0.png)


You should build it, then copy the resulting library (on windows this is a `*.dll`) from the build folder to a `Assets/Plugins` folder in unity. The included scripts (found in ![cs_examples](https://github.com/dimitribobkov/runity/tree/master/cs_examples)) will automatically find the dlls in the folder. 

![image](https://user-images.githubusercontent.com/30769396/116154376-7f168700-a6e0-11eb-92fe-56658a3c259c.png)



To use the library in unity, take both scripts from ![cs_examples](https://github.com/dimitribobkov/runity/tree/master/cs_examples) and make sure your library is in `Assets/Plugins`. Then, attach `DLLPool.cs` to an empty gameobject. This will store the loaded libraries throughout its lifetime. 

![image](https://user-images.githubusercontent.com/30769396/116154443-92c1ed80-a6e0-11eb-83f2-44d3b54f7aee.png)



Then, create any gameobject and attach `runity.cs` to it. It takes in one parameter - the name of the library. If your library was called `runity.dll`, you would input `runity.dll`. Now click play, and your gameobject will be running through rust!

![image](https://user-images.githubusercontent.com/30769396/116154299-69a15d00-a6e0-11eb-867a-e82f868538c1.png)

