# A Blog Site Wishlist

CoverCaption: My ArgoCD deployment for this website.
CoverImage: argo-site.jpg
Created: June 7, 2022 10:44 PM
Tags: Project, Tech

# We had success!

Last week the blog went live! Obviously not much fanfare behind it, but it was nice to see all of my choices around the publishing pipeline went on without a hitch. I wanted to take a minute to talk about that Infrastructure, and also form a little bit of a wishlist for myself and you, the reader, about the scope of what the blog should do!

# The System

The system I designed for this publishing pipeline straddles the line of novel and overly complex. Here’s what publishing one of these articles looks like:

To begin, I am writing this and all future posts in Notion. I’ve tried a ton of different writing apps and this was the one that stuck. It’s organization is really nice and it has proper cross platform support and dark mode. Once I’m done writing, I can export all of my text and tags into something standardized, in this case a markdown file.

The next step is what do I do with that markdown? Given markdown has rich content support with images, tags to search by, embeds, etc. there needs to be a processing step between what Notion spits out and what is ready to be used by the web frontend. In a past blogging system I wrote for a friend (hi June!), I handled this as a Webpack hook step, where it would convert all of their blogging files (in that case, TOMLs) into JSON and handle all of the indexing and photo optimizations to make it web ready. For this service, I wanted the deployment of posts to be severed from the site itself, so I decided to write some tooling for it with the goal to deploy a specific service backend just to serve blog posts.

## Blog Processing (project name `chatterbox`)

The processing tool, `chatterbox`, is written in rust and takes a directory of manually placed blog posts and photos and converts them into something more structured and usable. A big part of this is stripping out all of the fields and title added by notion, since that’s not part of the body of the post. Here’s what the first post’s header looks like:

```markdown
# Hello, again

CoverCaption: My view of the Seattle skyline tonight
CoverImage: skyline.png
Created: June 1, 2022 10:45 PM
Tags: Rant

# Here we go again
```

The file will always begin with a `h1` header for the title, but there’s no guarantee I start with one when I begin the actual content. But, working off of the assumption that Notion will always have those tags (which they will, since the notebook is structured to) I can basically just note the fields down as I find them and once all are account for I can interpret the rest as the body. This is all done with a beefy match tree as I iterate through all of the found markdown elements, that code can be found [here](https://github.com/ChristopherJMiller/chatterbox/blob/a3ef1703c18f9ee5ae41e6bdf242ccb2dd1880c2/src/post.rs#L41).

At the time of writing this, that’s all it really does. In an attempt to not immediately burn myself out working on this, the parser builds a `Post` object with the fields pulled out, places them into minified JSON files in an output directory, and builds a little index JSON to act as summary of all of the available posts. Soon this will be expanded more (we’ll talk future features later).

## Serving the JSON

Now that we can build all of our JSON, we need to deploy it to be served. Since the current site is deployed in a Kubernetes cluster, this needs to be as well. The [Dockerfile](https://github.com/ChristopherJMiller/chatterbox/blob/main/Dockerfile) for this is a two-stepper: Build the JSON then give it to nginx to serve. I also have a nginx config prepped for my specific cluster restrictions, which requires non-root containers. Add on some [github actions](https://github.com/ChristopherJMiller/chatterbox/blob/main/.github/workflows/docker-publish.yml) to build and publish to an image registry and it’s ready for the cluster!

The Kubernetes side, to keep things brief, is setup to deploy the image and expose it under `/blogapi`,  for example you can visit that indexing JSON [here](https://chrismiller.xyz/blogapi/index.json).

# Now What?

Well we have a blog, but it’s a little boring right now. Images aren’t supported yet, there’s no real way to know when a post is published unless you check, and as of writing this all of those fields we just spent time parsing out aren’t even used. Let’s figure out some need to haves and some nice to haves moving forward.

## Need to have: Images!

Images are easily the next on my list. Hell, I have cover photos ready to go for this and last blog and I can’t even show them yet since they’re not supported! Also, captions for all images used would be nice, it provides an easy way to additionally supply accessibly alt text right away.

## Nice to have: Reading Time

Aren’t those time to read labels so fancy on blogs? Turns out it assumes you read at around 200 words per minute and just makes a guess at it. Is it useful? Kinda, it’s not very accurate but it gives a good scale to your article.

## Need to have: RSS Feed

`chatterbox` really needs to generate an rss feed alongside everything else. I’ve really been getting into maintaining a blogging RSS feed lately, and it’s so convenient having a central service ping you when you have something new to read.

## Nice to have: Visitor Analytics

I currently have no idea who is actually reading this. The answer is probably no one at the time of writing, but I would love to be able to keep track of who’s visiting, where they’re coming from, etc. Learning about what open source solutions for analytics are out there will probably be a project in itself, and maybe something we can explore together later.

# Closing Thoughts

I’m happy with getting this blog off of the ground in a way that has it still feeling fresh. Features will be incremental, but if anything that gives future things to talk about. And since we aren’t tied to any specific blogging platform, the sky is the limit with what we can do!

As a closing song, I want to share a track of Purity Ring’s latest album. It was on my radar this week and I was listening to it a ton while working on getting the blog up and running. Talk to you next week!

[https://open.spotify.com/track/1mMAmnslV5tCn1ldEghNpi?si=hRfbQ6i3SYe93fiIuv4CiQ](https://open.spotify.com/track/1mMAmnslV5tCn1ldEghNpi?si=hRfbQ6i3SYe93fiIuv4CiQ)