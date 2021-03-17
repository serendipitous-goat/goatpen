<div align="center">

![GitHub tag (latest SemVer)](https://img.shields.io/github/tag/LemmyNet/lemmy.svg)
[![Build Status](https://travis-ci.org/LemmyNet/lemmy.svg?branch=main)](https://travis-ci.org/LemmyNet/lemmy)
[![GitHub issues](https://img.shields.io/github/issues-raw/LemmyNet/lemmy.svg)](https://github.com/LemmyNet/lemmy/issues)
[![Docker Pulls](https://img.shields.io/docker/pulls/dessalines/lemmy.svg)](https://cloud.docker.com/repository/docker/dessalines/lemmy/)
[![Translation status](http://weblate.yerbamate.ml/widgets/lemmy/-/lemmy/svg-badge.svg)](http://weblate.yerbamate.ml/engage/lemmy/)
[![License](https://img.shields.io/github/license/LemmyNet/lemmy.svg)](LICENSE)
![GitHub stars](https://img.shields.io/github/stars/LemmyNet/lemmy?style=social)
</div>

<p align="center">

## About The Project

Front Page|Post
---|---
![main screen](https://raw.githubusercontent.com/LemmyNet/lemmy/main/docs/img/main_screen.png)|![chat screen](https://raw.githubusercontent.com/LemmyNet/lemmy/main/docs/img/chat_screen.png)

GoatPen is forked from [Lemmy](https://github.com/LemmyNet/lemmy), a site similar to [Reddit](https://reddit.com), [Lobste.rs](https://lobste.rs), or [Hacker News](https://news.ycombinator.com/): you subscribe to forums you're interested in, post links and discussions, then vote, and comment on them. GoatPen intends to keep compatibility with Lemmy's federation interfaces as they develop, if practical (even though we're apparently banned ;-) ).

Each GoatPen server can set its own moderation policy; appointing site-wide admins, and community moderators. The key differentiator from Lemmy is that the developers will respect your God given right to free speech, follow your own conscience and freely choose to associate or not with whom you please.

*Note: Federation is still in active development and the WebSocket, as well as, HTTP API are currently unstable*

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs/)
- [Diesel](http://diesel.rs/)
- [Inferno](https://infernojs.org)
- [Typescript](https://www.typescriptlang.org/)

## Features

- Open source, [AGPL License](/LICENSE).
- Self hostable, easy to deploy.
  - Comes with [Docker](https://lemmy.ml/docs/administration_install_docker.html) and [Ansible](https://lemmy.ml/docs/administration_install_ansible.html).
- Clean, mobile-friendly interface.
  - Only a minimum of a username and password is required to sign up!
  - User avatar support.
  - Live-updating Comment threads.
  - Full vote scores `(+/-)` like old Reddit.
  - Themes, including light, dark, and solarized.
  - Emojis with autocomplete support. Start typing `:`
  - User tagging using `@`, Community tagging using `!`.
  - Integrated image uploading in both posts and comments.
  - A post can consist of a title and any combination of self text, a URL, or nothing else.
  - Notifications, on comment replies and when you're tagged.
    - Notifications can be sent via email.
    - Private messaging support.
  - i18n / internationalization support.
  - RSS / Atom feeds for `All`, `Subscribed`, `Inbox`, `User`, and `Community`.
- Cross-posting support.
  - A *similar post search* when creating new posts. Great for question / answer communities.
- Moderation abilities.
  - Public Moderation Logs.
  - Can sticky posts to the top of communities.
  - Both site admins, and community moderators, who can appoint other moderators.
  - Can lock, remove, and restore posts and comments.
  - Can ban and unban users from communities and the site.
  - Can transfer site and communities to others.
- Can fully erase your data, replacing all posts and comments.
- NSFW post / community support.
- OEmbed support via Iframely.
- High performance.
  - Server is written in rust.
  - Front end is `~80kB` gzipped.
  - Supports arm64 / Raspberry Pi.

## Installation

- [Docker](https://lemmy.ml/docs/administration_install_docker.html)
- [Ansible](https://lemmy.ml/docs/administration_install_ansible.html)
