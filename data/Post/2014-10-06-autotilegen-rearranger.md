---
title: AutoTileGen Rearranger
slug: autotilegen-rearranger
summary: I somewhat recently came across AutoTileGen by Pixelatto—a tool for rapidly creating “blob pattern” tilesets from just three input images. It seems like a pretty good tool, at least for a quick rough-in of tiles. It just has one major flaw—the tilesheet that it outputs is all mangled
author: kenton
published: 2014-10-06 21:37:02
category: programming
tags: [Python]
image: /assets/images/autotilegen-rearranger/Tileset.png.r.png
---

I somewhat recently came across [AutoTileGen](http://autotilegen.com/) by [Pixelatto](http://pixelatto.com/)&mdash;a tool for rapidly creating ["blob pattern"](http://www.squidi.net/three/entry.php?id=166) tilesets from just three input images. It seems like a pretty good tool, at least for a quick rough-in of tiles. It just has one major flaw&mdash;the tilesheet that it outputs is all mangled:

<!-- PELICAN_END_SUMMARY -->

<figure>
    <img src="/assets/images/autotilegen-rearranger/Tileset.png">
    <figcaption>How am I supposed to use this?</figcaption>
</figure>

So I created a simple tool in Python to covert the above output in something a little more friendly:

<figure>
    <img src="/assets/images/autotilegen-rearranger/Tileset.png.r.png">
    <figcaption>Much better!</figcaption>
</figure>

You can get it (and the source) on [GitHub](https://github.com/FuzzyWuzzie/AutoTileGenRearranger).