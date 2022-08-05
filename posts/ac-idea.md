# Over-engineering a solution to a bad portable air conditioner.

CoverCaption: My air conditioner
CoverImage: ac.jpg
Created: August 4, 2022 10:48 PM
Tags: Project, Rant

The largest downside of moving to the Pacific Northwest is not the egregious housing prices, nor the incessant number of tech bros, but rather the seeming lack of air conditioning in all rentals.

As of writing this we have just exited the second heat wave of the summer, reaching roughly 90 degrees Fahrenheit. At those temps fans aren’t cutting it anymore. You’re hot, your entire place is hot, and you can’t escape it. Luckily to save us renting plebeians against the might of mother nature and our landlord’s lack of care, we have individual air conditioning units. These are your run of the mill window mounted air conditions and single or double hose portable air conditioners. However, in places like my apartment complex they restrict you to only using the portable variety on some loose reasoning of either “safety” or “neighborhood aesthetic”.

> Ok so you buy a portable air conditioner and you’re good to go for the summer, why are you bitching about it in a blog post?

The issue is most cheaper portable air conditioners suck. And when I say cheaper I even mean the $700 dollar one I purchased. It’ll cool down the room (usually) but it’ll do so in the most inelegant way possible.

# My AC Woes

My portable AC is a Hisense 10,000 BTU DOE Dual Hose Portable AC. I currently have to run it in a single hose configuration because my apartment complex the weirdest thing possible and chose all incompatible window types and then mounted a pvc tube into the wall for portable ac exhausts. Efficiency plummets but it still gets the job done. My main woes are with two things, the onboard thermostat and it’s “wifi capabilities”.

## How about we stick the thermostat next to the hot part.

I have no idea where exactly the thermostat is within the unit but the spot is wrong. Usually how this goes is if the temperature to cool to is set below 75 degrees, the unit will never turn off and will usually freeze my bedroom (the room it’s sitting in and recording the temperature of) to around 68 by the morning. Above 75, and it will constantly cycle between on and off. Runs for 30 minutes, shuts off the compressor, turns off the fan, does nothing for 5 minutes, turns everything back on, rinse and repeat.

So upon seeing how poorly this thing controlled itself, my immediate idea was to solve it myself with some good ol’ smart home tech. This portable air condition has “wifi capabilities” that should mean I can control it from something like google home, attach it to my home assistant, and have the ecobee smart thermostat in my room manage the cooling schedule. Easy!

## The most rushed smart home app I’ve ever seen

I feel like Hisense barely got away on a technicality for “wifi capable” being a feature of the air conditioner. The associated app “SmartLife”, and it’s connection to Google Home, only allows you to turn the air condition on and off. That’s it. And it does both of these things poorly, as it’ll commonly disconnect until I power cycle it. How did they think this was good enough to sell is probably some combination of “we’ll add features later” and an underfunded software department. But regardless, the app is basically useless, I can’t even pull off any tricks with cranking the temperature all the way down and let my smart thermostat turn it on since it disconnects itself all the time.

So if I want my smart home integration, I’m going to do it myself.

# Making my own smart portable AC

I want to keep the actions of the smart portable AC basic, it should be able to:

Turn on and Off

Set the Temperature on it

Set Fan Speed

Set Fan, Cool, or Dry Mode

It needs to have a way to connect to my home assistant instance, which I can either do via wifi or zigbee. Something like an ESP32 would be perfect for this, and I can assist it from some custom pcb fab work as needed. Right now, the main hurdle is determining how I’m going to send it data and ensure the AC is synced with what home assistant expects.

## Idea One, IR Blasting

What this AC lacks in app features it makes up for in it’s remote. It’s a bi-direction IR remote that contains every possible feature (including some not built into the AC’s top panel). This is promising since every time you send a command, the AC will blast out it’s own response, which updates a little display on the remote with everything we would need.

The problem with this is I’m really bad at reverse engineering it’s signal. It’s running at a weird frequency for serial communication (2kHz-ish, but it varies) and doesn’t appear to line up any of the standard digital communication protocols. It’s probably using something more standard for these sorts of remotes, but I haven’t been able to find anything. I may need to perform a tear down on the remote to find some chip to look up.

## Idea Two, Wire to the Front Panel

What I consider the most blunt method is just doing a tear down on the top portion of the portable AC and wire in to the button terminals. Then, a breakout board can perform the button connections to change the inputs, and additional wires can monitor voltage across the indicator LEDs to determine what state it is in.

This method has a ton of downsides. It could possibly break my only source of cooling during the summer, it could injure me since it’s a high voltage appliance, and it would most likely be too annoying to wire into the entire front panel, leading to a cut in features.

## Idea One + Two, Wire into the Front Panel of the IR Remote

If I can’t figure out the what the IR remote is sending, might as well just use the IR remote. I can buy replacements for pretty cheap on ebay (which still irks me that this must be a common remote protocol wise), wire in to the buttons and screen, and use that for all my communication!

This is what I’m currently leaning towards unless I can figure out idea one. And this leads me to something additional that I want to do, which is a fun way to model interfacing with the air conditioner.

# Graphs on the Mind.

I think the air conditioner’s state could be modeled pretty well as a finite state machine, and getting from one AC state to another is a matter of traversing the FSM. Given the limited number of states, a breadth first search could be used to find the most efficient route, which equates to the least number of remote inputs needed to sync the AC’s state. This would run on the microcontroller, and interfacing with home assistant could be as simple as calling a state transition between the current state and the one retrieved.

Is this an excuse to write a novel FSM paradigm in Rust? Yes, 100%. I’m sure there’s already some out there but I’m curious about writing my own solution for it and comparing after the fact. Plus, blog content!

# Things to do:

Ok, so with this all in mind, let’s sit down and write a to-do list for this project. This will probably turn into the list of upcoming blog posts:

Write a FSM library

Interface it with Home Assistant, put it on a microcontroller.

Figure out the IR protocol or get another remote to rip apart.

Mount it nicely somehow, and done!

Phew, this should be a coool project. Excited to look into the library work soon. Until next time!

[https://open.spotify.com/track/213RFtIruWs7V6pAAvArRV?si=vFeSlzWYTsy3VyBJqqIk1g](https://open.spotify.com/track/213RFtIruWs7V6pAAvArRV?si=vFeSlzWYTsy3VyBJqqIk1g)
