# Low-Level Jam 0001

<!-- TOC -->

- [Low-Level Jam 0001](#low-level-jam-0001)
	- [Introduction](#introduction)
	- [What are we building](#what-are-we-building)
	- [Theme](#theme)
	- [When will it be](#when-will-it-be)
	- [How to Play](#how-to-play)
		- [How to Submit an Entry](#how-to-submit-an-entry)
	- [What can you use](#what-can-you-use)
	- [Judging](#judging)
	- [Who can play](#who-can-play)
		- [Team size](#team-size)
		- [Where to organize](#where-to-organize)
	- [Announcements and Organization Communications](#announcements-and-organization-communications)
	- [Presentations](#presentations)
	- [Licensing](#licensing)

<!-- /TOC -->

## Introduction
Welcome to the first Low-Level Jam, an emulator, virtual machine and tooling coding session taking place over the span of a week. In the spirit of game jams, [lang jams](https://github.com/langjam/langjam) and other hackathons the intent is for you, or your team, to create a new machine that fits the theme of the jam.

If you are interested in participating or finding a team for the jam, join [the Discord Server](https://discord.gg/59tmtaQe3X).

## What are we building
The goal of this project is to get code running somewhere it normally wouldn't be. This could mean simulating hardware, like a console or circuit emulator. It could mean a virtual console, like the likes of corewar or pico-8. It could mean building hardware to simulate an important tool, like a rom emulator for a piece of old hardware. Or it could mean building an entirely new architecture of your own design to share with the other players.

We aim to not limit creativity and instead encourage players to experiment on something they normally wouldn't think to try.

## Theme
The theme of this jam will be 

Small, but mighty.

## When will it be
The event will take place starting Friday, June 17, 2022 at 17:00 GMT running for 1 week until 17:00 GMT on Friday, June 24, 2022.

A kickoff will be planned for this time to announce the theme and the official start. 

## How to Play
The jam will take place over the span of a week with each team spending up to 48 hours (2 days worth of time) per player on the project. This time will include planning, implementation, testing, documentation and presentation preparation and can be broken up to accommodate the needs of each individual.

### How to Submit an Entry
Entries are submitted by opening a PR to the jam repo containing a project named directory that follows the [submission template](./submissions/template/README.md). I.e. if a project is called `racecar` a pr would add a directory called `racecar` to the `submissions` directory containing _at least_ the following:

Submissions should link to, or contain all material for a team's project, including but not limited to:

- All source code.
- All presentation materials
  - Documentation
  - Slides
  - Links to demos
- Any working examples
- Instructions on how to setup and run the project from either:
  - A fresh `debian:bullseye`, `ubuntu:21.04` or `archlinux:base` container
  - A fresh `Ubuntu 21.04` install.

Source code for submissions should be made available through a git tag created prior to the end of the jam, or by including it directly in the teams submission directory. If a team chooses to provide a tag to an external repo, it must only link publicly to either GitHub or GitLab for ease of accessibility. We encourage submissions to include as much of the following supplementary materials as possible.

These are the bare-minimum requirements and teams are encouraged to provide as many easy ways for new users to try out their submission.

## What can you use
You can use any languages, libraries or tools in building the project as long as it can easily be setup and run on one of the following.

- A fresh `debian:bullseye`, `ubuntu:21.04` or `archlinux:base` container
- A fresh `Ubuntu 21.04` install.

## Judging
At the end of the jam teams will be provided an opportunity to demo their work. These demos can include a written demo with examples (blogpost-style), a pre-made video, or hands-on demos in the group chat.

Teams will have the option to arrange for the presenters to demo their tool ahead of time if they are uncomfortable with or unable to demo it themselves.

All pre-made submissions should be made available to the organizers prior to the demo sessions for a preview.

After all teams have showcased, participants will be allowed to vote for their favorite submission along with a few additional categories (most creative, most bizarre, etc..).

After all voting has been counted winners of each category, and overall favorite will be announced.

All _participants_ will get one vote in each category.

Categories will be announced at time of judging.

## Who can play
Anyone, there are no restrictions on age or location. As such submissions, documentation and interaction with other players should be kept as inclusive as possible. 

### Team size
There are no restrictions on team sizes.

### Where to organize
All event chat, will be kindly hosted by the [the LLJS Discord server](https://discord.gg/59tmtaQe3X). This will be where players can both find teams and chat about their progress throughout the event. All participants are expected to remain inclusive and age-appropriate at the risk of removal from both the event and chat.

## Announcements and Organization Communications
All event announcements will be conducted via the `#jam-announcements` channel on the [the LLJS Discord server](https://discord.gg/59tmtaQe3X). Any communication related to event announcements, scheduling, themes or other miscellaneous information will be announced or echoed officially through this channel.

## Presentations
Presentations will be announced closer to the event via the announcement channel and will vary based on the number of participants that will need to be accommodated. Most likely it will be one of in order of precedence:

- Discord
- YouTube Live
- Google Meet
- Zoom

## Licensing
Entering teams retain all rights to their submission and no restrictions on the licenses are imposed as long as an entry's chosen license respects the license of all projects used in the entry, does not restrict any other submissions and allows the jam access to the to the project for showcasing.