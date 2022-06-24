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
The cell type is input but it has an input index of 0 and doesn't actually get computed with the rest of the pads. 
