# Fate has decided my AC is slightly better

CoverCaption: The app lives
CoverImage: fate-at-work.png
Created: October 19, 2022 2:03 AM
Tags: Project

Hello again, some exciting news has graced us all. 

It’s been a little bit since I’ve posted about projects. Splitting my time in Toronto to be with my partner means certain projects (like the AC) a little more difficult to work on, but don’t worry new opportunities have arisen in it’s place (more on that later).

In returning to Seattle after spending all of September in Toronto, there is an unprecedented bout of heat and forest fires. Combining the poor ventilation in my apartment with getting headaches every time the air quality index is above 160, the AC is finding a lot of use this fall. And in using it more often, I thought to give the wifi setup another go around. And this time… it worked? This is a huge sigh of relief for the project!

# Thank you whoever fixed my AC’s app

No firmware was updated on my AC, which means all of the fixes must be app side. Shout outs to the Hisense engineer who fixed the bugs related to the random disconnects! You app is still half baked and truly awful but at least you’ve now delivered the features I expected.

Google Home functionality is still just as bad (a single on and off switch), but the presence of a very feature-rich app has lead to some new ideas. Instead of directly interfacing with the physical machine, digital means are back on the table! Yay no risk of injuring myself on high voltage applicanes nor over-complicated systems! We now just need a way to directly interface with the app since progress appears to be [slow](https://github.com/deiger/AirCon/issues/160) on the API front.

# Emulating Android (with little user intervention)

Anyone who has done Android development has most likely come across the simple options for getting an emulator running locally. Android Studio makes it really simple to get things moving and I honestly prefer it over installing to a physical device. Can we do that but for running this awful smart home app and then figure out how to mock presses on it?

## The Two Contenders

Ideally I would want this to be run in a kube cluster, just like any other service on the server. So what’s our options look like for running Android inside a docker container? The answer is two different methods.

### Dock Droid (KVM)

[Dock droid](https://github.com/sickcodes/dock-droid) appears to utilize your systems virtualization modules to spin up QEMU inside of a docker container. It seems really feature-rich, good documentation, and ticks the boxes of being easy to clean up and easily exposing adb (main thing that’s neeed).

### ReDroid (Kernel Sharing?)

[Redroid](https://github.com/remote-android/redroid-doc) is to be the other popular option, with many docker images marking themselves deprecated and linking to this project. It appears to utilize a more barebones approach, sharing more of the underlying system kernel instead of visualizing it. This requires some additional kernel modules to be installed (the Android binder IPC and Ashmem modules) and needs containers to run as privileged. From a performance perspective this sounds fantastic, reducing a lot of the overhead attributable to the QEMU solution. However, in practice, this setup is incredibly finicky. Even with their instructions per popular distro, the errors are obscure and the issues posted unhelpful. Not to mention requiring the additional packages installed on the hypervisor and requirement to run privileged (you can probably mount the specific device files but that adds onto the finicky nature of this solution). Time will tell which solution is chosen but I’m leaning more towards Dock Droid at the moment.

## Messing with the App

Once we have the app running and can connect to adb, we now have to figure out how to read data out of the app and write our requests temperature settings into the app.

Beginning with reading, we need to be able to read the AC’s status at some regular polling interval. If we’re lucky, maybe there’s some logcat messages we can parse that give us state information. But more likely, we’re going to have to read what’s on the screen. Assuming a static screen layout and known parameters, it shouldn’t take much to be able to get a pretty good computer vision solution to this problem. Ask adb to take a screenshot of the app, run it through our CV system, and spit out a JSON payload of the state.

In terms of writing, this is where the input requirements begin and some more fun logic has to be figured out. The input commands we can send via adb shell are all single touch functions, and we have the options to swipe and tap. This is perfect for the smart home app as all actions can be accomplished via tapping and all locations are static. Therefore, all writing operations can be reduced to a list of tapping actions to do in sequence. What are some facets to this action sequence?

## Known Constraints

All actions will have to start and stop at the main menu screen as there is no cross-settings page navigation. Additionally, there is an unknown load time attributed to most actions, including viewing pages, since communicating with your AC is a *foreground* operation. Finally, and quite the doozy, **actions are not gu**a**ranteed to succeed because the app sucks**. Sometimes you will attempt an action and the app will revert back to it’s previous state, no message sent to the AC.

Breaking these down, constraint one means there’s not much room for action-sequence optimizations, you will always have to go back to the start. This has the nice benefit that write actions can be unsorted if sent together, just throw it into a queue and let the system slowly write the data. Constraint two and three can be solved in similar methods, more CV! C2 boiled down to expecting to be presented with a certain page before performing further action. The system should be able to check for that page, and if it can’t find it back out and retry the action in a little bit. C3 can be solved almost the exact same way except instead of just looking for a certain page, also look for the *expected change* in that page, demonstrating the state is written. Given we already will have the logic for this as part of the reading data process, adding this is trivial.

# Systems Go? What’s Up?

So this all seems nicely in place! When am I going to work on it?

Good question! I am currently juggling five-ish projects including this one so progress is jumbled as I’m trying to enjoy my time with them while also not burning out. We have:

- This project, AC Smart System with what will probably become lots of tedious interfacing with the app via adb.
- This website’s development, with new features rolling in to actually support this blog better and look more feature complete.
- My home lab, Luma, which has recently faced data loss on my part and I’m mulling over my options to better prep myself for increased storage and more resilient storage (that will probably become a blog post).
- IPv8, a programming multiplayer game where you have to write solutions to puzzles to unlock certain game functionalities, all while juggling a resource management game. This project is currently on the back-burners due to scope-creep and overlap with…
- Butter Tart, which is a public relations tool my partner gave me the idea for that I think is really promising. It shares a similar microservice architecture to IPv8, so combining that with my day job also turning into a lot of cloud service-style architectures means I’m at risk of burning out if I’m not careful. I’m expecting to give this project my focus until it is either complete (probably another month) or proven unnecessary by some other product (unfortunately a possibility at the time of writing).

Of these projects, IPv8 and Butter Tart are the ones I’m most excited about. Butter Tart actually has a user base awaiting any details on a release for them to try and IPv8 I believe has the ability to be something special. While the AC it nearing a final solution, it’s also beginning that off-season time that was mentioned in the previous article. I probably won’t end up thinking about using it until spring rolls around next year. So, the new year feels like the perfect time to face this work, and hey, maybe that’ll give the AirCon maintainers some time to figure things out and make my life even easier.

Next time, depending on how the next week or so goes, expect either a chat about home labs or Butter Tart. Until next time.

[https://open.spotify.com/track/4T5ZsCfzhTVmE6lHM9c3gb?si=73566d7b5fba4641](https://open.spotify.com/track/4T5ZsCfzhTVmE6lHM9c3gb?si=73566d7b5fba4641)