# Simpleaudioswitcher

Simple program to simplify the use of pactl or wpctl (wireplumber). Currently can be run in the terminal or wrapping in a script and 
calling a terminal with it's outputs.

Currently tested with:

`pactl 15.0`

`wireplumber 0.4.8`

### Arguments
`-c --cli` 

Which cli to use, supported values are: `pac` or `wp`

## Usage

A sample script looks like:

```
#!/bin/bash
~/.scripts/simpleaudioswitcher -c wp
```

And then I have a gnome shortcut like:

`foot -- $nameofmyscript`

`$nameofmyscript` being the file wrapping the call to the executable.

Currently, it's very barebones, not much error handling or even proper project structure since I'm not well versed in rust.

Also, currently it only deals with outputs, no mic switches (can only be done with pactl, I dont know a way to do it with wireplumber)

### Why Rust at all

Funny story, the first time I wrote this stuff was with kotlin (something I enjoy writing), but distributing might not be that straight forward with java stuff. Other thing that could bother people is resource usage,
with kotlin I couldn't make it run with less than 5mbs of heap which sounds ridiculous (but whatever, memory is cheap),
so I tried my hand at a Rust hack, and it's pretty lightweight and functional (resource wise). 