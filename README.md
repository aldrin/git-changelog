[![Build Status](https://travis-ci.org/aldrin/git-changelog.svg?branch=master)](https://travis-ci.org/aldrin/git-changelog)
[![Build status](https://ci.appveyor.com/api/projects/status/ixcfop3nhjmx3s5v/branch/master?svg=true)](https://ci.appveyor.com/project/aldrin/git-changelog/branch/master)
[![Crates.io](https://img.shields.io/crates/v/git-changelog.svg)](https://crates.io/crates/git-changelog)
[![GitHub release](https://img.shields.io/github/release/aldrin/git-changelog.svg)](https://github.com/aldrin/git-changelog/releases)
[![codecov](https://codecov.io/gh/aldrin/git-changelog/branch/master/graph/badge.svg)](https://codecov.io/gh/aldrin/git-changelog)

`git-changelog` is a tool to automate repository [changelog] generation.

A commit [like this](tests/common/sample-commit.message) generates an output [like
this](tests/common/sample.md).

## Motivation

Commit messages must always be meaningful and with a little extra effort we can automate the chore
of generating meaningful change logs for our users. As I finish up work on a change to a repository,
I like to pause a while, consider what the change means to the end-user and reorganize the message a
bit. If you follow the (easy) conventions described below and tag lines in your commit message
appropriately, this tool will help you generate an *accurate* and *presentable* change log.

A little time spent at commit, when the context and impact of the change is fresh in mind, saves a
lot of time at release milestones.

## Installation

```bash
> cargo install git-changelog
```

If you use a Mac with [Homebrew], you may prefer the following:

```bash
> brew tap aldrin/tap
> brew install git-changelog
```

Both options build the executable from sources.
If you're looking for just the executables, they're attached to the [releases].

## Usage

Just write your commits as you [normally] do. When it looks like a particular commit includes a
change that the "user" may be interested in, tag its lines appropriately.

Concretely, instead of writing this:

```
Add support for filtering responses

UI gets a bit cluttered when the response contains too many
items. Added a simple filtering scheme to reduce the result
set to a more relevant subset. Clients using v1.2 need to
upgrade to accomodate the new request parameter.
```

Write this:

```
Add support for filtering responses

- feature: UI gets a bit cluttered when the response
  contains too many items. Added a simple filtering scheme
  to reduce the result set to a more relevant subset.

- break: Clients using v1.2 need to upgrade to accommodate
  the new request parameter.
```

The two commit messages are almost the same but the latter tags *user visible* changes a bit more
diligently. Eventually, this diligence helps the tool to identify lines, aggregate similar things
(e.g. breaking changes) across commits, order them, and give you a report that you can share, as is,
with users. Or, you can use the output as the starting draft, make editorial changes and then share
it with users. Either way, it saves you some time.

Of course, you don't need to tag **every** commit (`git commit -m` is perfectly fine, where you
think it is). You just need to tag the changes you want your users to know about. The quality of the
tool output depends on the quality of *your* input.

## Generate reports

When `git-changelog` is on the path, `git changelog` works just like `git log` and takes similar
arguments. Concretely, it takes a [commit range] and looks for all commits in that range and uses
the tags it finds in their messages to generate a report. Simple. :)

The default revision range picks all commits made since the *last* tag. For example, to generate the
changelog for `v0.2.0` of the tool, I used the following:

```bash
> git changelog
```

The command picks all commits since [v0.1.1] (the current *last* tag) and generates a change report
that I pasted verbatim to the [CHANGELOG.md].

## Customization

Each project is different and you may want to customize the tags and output to suit your
requirements. You can do that by adding a [.changelog.yml] file to your repository root. See the
[default configuration file](changelog.yml) for a starting example.

If you like to tweak the output, you can specify a [Handlebars] template to control the rendered
report format (using `template` key in your .changelog.yml). The [default template](changelog.hbs)
generates a Markdown document that renders well on Github.

[normally]:https://chris.beams.io/posts/git-commit/
[changelog]: http://keepachangelog.com/
[commit range]: https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#_commit_ranges
[Handlebars]: http://handlebarsjs.com/
[Homebrew]: https://brew.sh/
[CHANGELOG.md]: CHANGELOG.md
[v0.1.1]: https://github.com/aldrin/git-changelog/tree/v0.1.1
[.changelog.yml]: .changelog.yml
[releases]:https://github.com/aldrin/git-changelog/releases
