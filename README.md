# runity

## runity explained

### What is it?

runity is a rust to unity interface library. Currently, it doesn't reimplement every unity function and quirk, but it is a goal. It aims to act as an alterative scripting language than c# in unity, and can be used for everything from modding to regular scripting.

### How does it work?

runity works by implementing unity datatypes, functions and more into an interface callable by rust. The script written in rust is then compiled into a dynamic library, which is loaded into a unity application at runtime. This allows for rust to be used for modding and scripting support, or to be used as the whole backbone of your game.

## How do I use it?

Please look at the examples to see a simple example of usage. Your library must be a `["cdylib"]` type, and you should copy the resulting library from the build folder to a `plugins` folder in unity. This means that when using this, you shouldn't make a binary but a library. The examples folder also includes a monobehaviour script which will be required to use the resulting DLL in unity. To use it, simply make a folder in `Assets/Plugins/` and put your plugin in there, then attach the monobehaviour to the object you want the script to run on. Then, enter the name of the DLL (eg, `"runity.dll"`) and it should work!
