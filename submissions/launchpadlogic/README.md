# Launchpad Logic
## Team
* Max Starr

# The main idea 💡
Since the theme of the project is 'small but mighty' I looked for what I had lying around which qualified. 
I knew from past experience that an open source SDK had been made available for making custom firmware for the Novation Launchpad Pro and it's and area I'd been wanting to dip back into.
Originally the plan was to build a rudimentary calculator using the pad interface. 
Calculators are usually a good starter project. However I quickly scrapped that for the idea of instead implementing a boolean logic simulator.
The idea of building up logic circuits only through the pads seemed interesting but also approachable.
(It also avoids the issue of handling infix operators and nesting c:)
The general specification came together in my head while I got re-aquatinted with the SDK and build process (more on that later). 
The main requirements were
* Have several gates which can be placed anywhere on the grid
* Have a system generated clock
* Have user input buttons
* Each row outputs the value of the last pad to its row terminator
* All signals from the perimeter buttons can be routed anywhere on the grid

## A quick aside 📜
I will make a distinction between pads and buttons in this document. 
Pads are the white squares in the middle of the device.
Buttons are the circular bits around the border.
# Initial setup 🧱
It took a minute to re-locate [the SDK repo](https://github.com/dvhdr/launchpad-pro). 
Once I did I found that it hasn't seen any activity in 4 years.
Normally this would be a red flag, and honestly, it was, but I really didn't want to re-implement this all myself. 
My fears turned out to be unfounded as the code works flawlessly! 
The repo outlines in good detail the build process.
It neglects to mention that anyone using the SDK needs to install the git submodules. 
That tripped me up for a bit before I noticed the `.gitmodules` config file.
With that set up, running the docker container and `make` inside it seemed to work flawlessly!

# The firmware upload nightmare 👻
It is now relevant to discuss what exactly the Launchpad *is* for anyone who might not know. 
Simply, it is a MIDI controller: a device built to send MIDI messages back to a computer or other music hardware. 
MIDI: Musical Instrument Digital Interface, is a protocol from the early 80s used primarily by music devices to send information. 
It's normal uses are for sending note information: pitch, how hard ta key was hit, etc. 
There is, however, an extension to the MIDI spec called SysEx MIDI. 
SysEx MIDI: System Exclusive MIDI (refereed to as SysEx) is a device specific extension to the base MIDI standard. 
SysEx messages can be used to configure options about a piece of music hardware. 
The specifics of the messages are set by the manufacturer, there is no set standard.
In this case, SysEx is used to completely re-write the firmware of the Launchpad itself.
So where you might upload a `.hex` or a `.bin` file to other devices, here we send a `.syx`. 

The *nightmare* comes in actually *doing* the uploading. 
This is a standard which hadn't been updated since the 80s by the time this device came out. 
And even then, according to sources I've found, SysEx is mostly used by older hardware. 
Given this, the tools for sending SysEx are sparse at best. 
The one most commonly recommended, [MIDI-OX](http://midiox.com/), appears to not have been updated since Windows XP days.
Nevertheless I gave it a shot. The UI leaves much to be desired by modern standards but if the tool works, it works. 
It didn't work.
Whatever I tried I kept getting one error after another. 
The most confusing of which was the assertion that my computer didn't have enough memory to process the upload. 
I checked, it had 15GB free at the time.
Other times it wouldn't detect that the device was connected.
There were a few just out and out crashes too in the mix.
Overall, not a satisfactory experience to say the least.
So I searched for an alternative.

The *only* other option I could find was from the Windows Store of all places.
Simply titled "MIDI SysEx file transfer utility" it seemed to do all I needed and nothing more.

At first it *also* gave me no end of errors, these ones much less helpful, amounting to essential "Something went wrong."
However... after many restarts of the application, the Launchpad, and my whole PC, I managed to get the example firmware to upload. 
Needless to say, I was elated.

The nightmare wasn't quite over though. 
Any time I went to upload the latest build there was a chance it would fail and I'd have to go through the whole dark ritual of getting it to work again.
But I figured out the set of things to keep trying in random order until I could consistently get it going again in just a few minutes.

With the process more or less locked down, I set to work actually learning the SDK. 

# Testing the waters: Game of Life 🌿
Given the fact that I was staring at a light up grid it seemed natural to get aquatinted with the development process by implementing Conway's Game of Life.
The only documentation given for the SDK is the example code provided.
While not a lot, I actually found that to be enough to get through everything I wanted to do.
I began by figuring out which of the supplied callback functions I could safely ignore.
That turned out to be quite a few.
Since I don't actually care about handling MIDI messages I could leave only stubs of the callbacks which handle that.
The main three I concerned myself with were:
* The first time setup function `app_init`
* The main loop `app_timer_event`. The SDK author claims this runs every millisecond, I have no way to confirm this but it seems about right.
* The pad event handler `app_surface_event`

The example code already had an array for storing the states of all the pads and buttons which told me doing that was at least not totally insane.
Since there's no way to read the LED state of a given pad or button it seems the firmware has to maintain some representation of it. 
The README from the SDK repo also stated that dynamic memory allocation wasn't a good idea.
Great! I didn't want to deal with that anyway. c:

Once I began writing some helper functions for handling the GOL rules, I realized that having Github copilot enabled felt like cheating.
But that wasn't in the rules so I kept it. c:
Go figure that an AI trained on the contents of millions of public repos could whip out a GOL implementation in any language. 

With the logic down I repurposed the example code in the main event loop function to run the simulation every tick. 
I decided to only include the 8x8 pad grid as the playing field to keep things neat.
Since the pads and buttons are indexed in increasing sequential order starting from 0 in the bottom left I just had two `for` loops run over only the inner pads.
Strangely, though there are no buttons in the corners, they still take up a number in the sequence.
So the first button you can actually press is index 1 and the last is 98 despite 0 and 99 being valid indexes to address.

I was pretty sure the simulation was running at this point but, with no cells living to start, I couldn't be sure. 
So in the initial setup function I just set some random cells to be alive at the start to see what happened. 
Lo and behind, it worked! It wasn't terribly exciting as the pattern I'd chosen died immediately, but it worked!

I spent some time messing around with the initial patterns to make sure it followed all the expected rules and it sure seemed to!

I then realized that if I just set a cell to be alive in the pad event handler, I could create an interactive GOL on the pads!
The code for it was essentially one line to just toggle the pad state when it was pressed.
With that in, I could now 'play' GOL like I never had before.
It was mesmerizing to see the patterns evolve and be able to change them in real time.

But this wasn't my end goal, as fun as it was, so it was on to bigger things!

# The real work begins 🔨
## Step 1: Scaffold out ideas
I started the actual logic sim project by roughing out the struct I wanted to use to represent each pad button and pad.
I came up with three types of cell:
* Input: A cell that gets its value directly from another cell
* Gate: A cell that computes its output based on a logic gate type
* Empty: nothing c:

I also, just to have, enumerated the gates I wanted to start with.
* AND
* OR
* XOR
* NOT

I then put together the actual struct to represent the cells.
I wanted to just have one for all possible cell types, so not all the fields are always used. 
This keeps the size of the array of all the structs constant and, since memory never seemed to be an issue, this worked out fine.
The struct stores:
* The cell type
* The index of the cell it takes input from if it's an input cell
* The gate type if it's a gate cell
* The gates input indexes. These are always just the two cells behind the gate so this doesn't need to be stored exactly, but it is. 
* The output value of the cell

## Step 2: the clock ⏱
I started off the actual implementation with the clock.
The cell type is empty but has its output set explicitly in the code.
In the main app loop function, it has a variable which counts up to a set number of cycles. 
Once that happens, the clock state is flipped and the simulation is run. 

While I was writing out the update code, Copilot stepped in and filled in the implementations for all the gates.
I just ran with it since I didn't have any deep desire to write that code myself, but it was *wild* to see it spring that out of nowhere.

When I managed to get the firmware uploaded I saw what would usually be the 'record arm' button blinking! Progress!

## Step 3: Basic routing 🔀
My idea for how to handle routing signals was pretty simple: Hold down a signal provider button then press the pad where you want the signal to go.
It is, however, not perfect. 
For example, this is difficult to do if you only have one hand, not impossible as the device isn't that wide across, but difficult.
Also the buttons are stiff and tiring to hold for more than a couple seconds.
This becomes more of a problem later. 
However, the strategy works and is simple to implement. 

I store an index for whatever button is currently being held.
When the button is pressed, the index is set, when it's released, the index is reset to 0. Dead simple. 
This doesn't support holding multiple buttons, in fact doing so can lead to unexpected behavior, but that's not explicitly part of the UI so a user would *never* do that! c:

When a pad is pressed, it checks if there is a non-zero index in the held button variable.
If so, it checks the type of the cell at that index and sets itself accordingly.
For now, just to an input with the source index as the clock.

With that set, I could now get any pad to blink in time with the clock! Sweet! 

## Step 4: Row values 🔢
One of my initial requirements was that each row output the final value to the button which caps it. 
By final value I mean the value of the non-empty pad furthest to the right in that row. 

This required refactoring the main application function to use nested for loops instead of one single loop. 
Nesting them made it easier to keep track of when a row was done processing.
It was simple enough then to just directly set the value of the row-ending buttons similar to how the clock was set. 

I tested by linking up some pads to the clock and it all seemed to work out!

## Step 5: Clear cells ❌
If I can 'place' clock signals, I want to be able to remove them as well. 
Here I just check if the input index of a given cell equals the button that's currently held down.
If it does, the cell is set to empty and its value is cleared. 

This decision will haunt me the rest of this project.

## Step 6: Gates! 🚪 (I couldn't find a gate emoji. Sue me.)
This was the first real test of the logic as I saw it.
I started by deciding which gate will be assigned to which button. 
Fortunately, the top left of the launchpad features buttons which are normally used for navigation.
So with up, down, left, and right arrows, the placement was basically decided. 
In some mathematical notation systems, and is written as ∧ and or as ∨ so those were settled. 
Circuit diagrams draw an inverter as essentially a > with a bubble so putting it on the right arrow made sense. 
That just left < for xor, which makes as much sense as anything else. 

In the setup function I manually assign distinct colors to each of the buttons.
The LEDs will hold their assigned color until explicitly changed, so this operation only has to be done once. 

I then implemented essentially the same code as with the clock for setting pads to certain gates.
Here the code got more than a little messy.
It just runs a switch statement through all the possible gate button indexes and then manually sets all the pad values accordingly, including the color.
It's criminally verbose but it gets the job done and runs fast enough that I don't care to fix it.

With the cell configured, the code that Copilot wrote in step 2 was put to the test.
It worked flawlessly! 
I now had working and, or, xor, and not gates! 
Though I could only test them with the clock input.

## Step 7: Route row outputs 🔀
This was a quick step. 
It just involved allowing the buttons at the end of rows to also be routed the same as the clock.
It uses exactly the same code.
Works fine!

## Step 8: Realtime user input 👆
I decided to make each of the buttons on the bottom row a rotatable user input button.
Whenever they are pressed, whatever pads they are routed to will be set to 1, and 0 when they're released.

Initially the routing was simple enough, just the same as the row end buttons. 
However, I soon discovered that only running the simulation every half second made inputs feel terrible. 
It took, at most, 499 ms to update the tile. That's a lot of input lag!
This was quickly solved by moving the simulation code to where it is run every millisecond rather than in sync with the clock.

With user input solved, I could finally make my first interesting circuit, a full adder! 
Because of the interconnected nature of it, each gate had to be on its own row, but it did work! 
It's really fun to watch the pretty lights flash and update in real time. 

## Aside: Just playing around 🕹
At this point I had enough built to start messing around and having fun with the simulator.
After the full adder I tried a few more async logic circuits and they all worked great! 
I tried building some latches; SR worked great as did a D flip flop. 
It was here I also learned that using the bottom row of buttons for input wasn't the best idea.
As I said earlier, holding them down for more than a second is not comfortable at all. 😅

I wanted to then try building a binary counter. 
The design I found used a JK flip flop but, because it's built of all NAND gates, I couldn't fit it neatly into the grid since I had to build it with an AND and an inverter.
So...

## Step 9: NAND and NOR!
This was also pretty simple.
I chose colors to represent the new gates, put the buttons right next to the existing gates, and duplicated the existing gate code for them.
Dead simple, worked as well as the rest.

## Step 10: Pain 😭
Here is where I started to break down.
Async logic seemed to work *great* but I started having issues with synchronous logic.
My ultimate goal was to build a binary counter but no matter what I tried, every stage after the first always just showed the same value.
I tried a design using JK flip flops, D flop flops, none of it seemed to work properly. 
The flip flops worked *fine* in isolation, but something about combining them caused issues. 

I went down an *entire* rabbit hole trying to solve this. 
I took a tip from a friend in the LLJS Discord and tried a design where all the updates for a simulation 'tick' write into a temporary state.
That state is then copied to the 'working' state after all the gates are updated. 
While I'm sure this approach is better, it didn't seem to fix anything. 

I moved the simulation function call back in with the clock tick to try and slow things down and saw something interesting.
I had noticed before that when the latch was in a state, it there was something odd about the lights.
Slowing it down revealed the issue, the latch isn't actually maintaining a steady state, there is a certain amount of oscillation going on.
The feedback loop causes the gates to be constantly changing states every few cycles. 
I'm *pretty* sure this is why it isn't working as intended. 
However, I haven't dug deeply enough into *why* this is happening to get it working. 

A good half of my time on this project was spent trying to debug this issue and as of now, it's still not fixed. 
So, async logic works great, some synchronous logic, on the surface, works fine, but there are still some deeper bugs in the system.


## Future improvements
There is a *ton* of room to make the code itself more modular.
Storing the button colors instead of the gate input values for one would clean up a *lot* of logic. 

Feature wise, I never actually implemented a dedicated delete button. 
If I wanted to clear a cell, I either had to hold the same button that was used to set it or, as I more commonly did, hold one button to set all the cells then immediately unset them.
This is what I meant when I wrote that the decision on how to clear the cells would haunt me. 

Perhaps more importantly, there is still a lot of ambiguity in the UI.
If a cell is routed from an input and the input is off, there is no way to visually distinguish it from an empty cell. 
Similarly, if the input is on, there's no way to tell *which* input it is receiving. 
I haven't thought enough about these issues to propose solutions but I'm sure they could be found. 

# Conclusion
Overall this was a *wildly* fun project.
I had way more fun writing C than I ever thought I would after college.
(It turns out making LEDs blink is very satisfying)

I learned a lot about the basics of digital logic circuit simulator design.
Turns out the basics aren't as daunting as I thought!
But I also learned that, as always, the devil is in the (implementation) details.

I also learned I hate working on this platform just because the upload process is so damn painful. 
Fun times! 😁

I'd like to sincerely thank Nate Catelli for organizing this jam and for all the advice he gave.
It really got me to get my butt back in the saddle of working on projects outside of work.
Thanks to him, I have some of my passion for coding exploration back. 

I also need to thank Francis Stokes for building the amazing LLJS community which made this jam possible.
His videos inspire me to this day and I swear I've just osmosed better JS and TS practices from them.

# Invaluable resources I used on this journey
[The readme and example code of the SDK](https://github.com/dvhdr/launchpad-pro)
[This article on understanding what SysEx actually is](https://blog.landr.com/midi-sysex/)
[The MIDI SysEx Transfer utility. Thanks Pete Brown!](https://apps.microsoft.com/store/detail/midi-sysex-transfer-utility/9PFD4DDWGKTN?hl=en-us&gl=US)