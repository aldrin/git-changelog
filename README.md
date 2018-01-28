[![Build Status](https://travis-ci.org/aldrin/git-changelog.svg?branch=master)](https://travis-ci.org/aldrin/git-changelog)
[![Build status](https://ci.appveyor.com/api/projects/status/ixcfop3nhjmx3s5v/branch/master?svg=true)](https://ci.appveyor.com/project/aldrin/git-changelog/branch/master)
[![Crates.io](https://img.shields.io/crates/v/git-changelog.svg)](https://crates.io/crates/git-changelog)
[![GitHub release](https://img.shields.io/github/release/aldrin/git-changelog.svg)](https://github.com/aldrin/git-changelog/releases)
[![codecov](https://codecov.io/gh/aldrin/git-changelog/branch/master/graph/badge.svg)](https://codecov.io/gh/aldrin/git-changelog)

`git-changelog` is a tool to automate repository [changelog] generation.

A commit [like this](src/assets/sample-commit.message) generates an output [like
this](src/assets/sample.md).

[![asciicast](https://asciinema.org/a/Jk8A5UEJGkhlalL4gl3HevC7e.png)](https://asciinema.org/a/Jk8A5UEJGkhlalL4gl3HevC7e)

## Motivation

Commit messages must always be meaningful and with a little extra effort we can automate the chore
of generating meaningful change logs for users. As I finish up work on a change, I like to pause,
consider what the change means to the end-user and reorganize the message a bit. If you follow the
(easy) conventions described below and tag lines in your commit message appropriately, this tool
will help you generate an *accurate* and *presentable* change log.

A little time spent, when the context and impact of the change is fresh in mind, saves a lot of time
at release milestones.

## Installation

The recommended way to install the tool is:

```bash
> cargo install git-changelog
```

This compiles the tool for your environment from the sources. 

If you just need the executables, they are attached to the [releases].

If you use a Mac with [Homebrew], you can get the latest release with the following:

```bash
> brew tap aldrin/tap
> brew install git-changelog
```

## Usage

Just write your commits as you normally [would]. When it looks like a particular commit includes a
change that the "user" may be interested in, tag its lines appropriately. Concretely, instead of
writing this:

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

You don't need to tag **every** commit (`git commit -m` is perfectly fine, where you think it is).
You just need to tag the changes you want your users to know about. 

> The quality of the tool output depends on the quality of *your* input.

## Generate reports

When `git-changelog` is on the path, `git changelog` works like `git log` and takes *similar*
arguments. It looks at all commits in the given [commit range] and uses the tags it finds in their
messages to generate a report. Simple. :)

## Customization

Each project is different and you may want to customize the tags and output to suit your
requirements. 

**Conventions**: You can define change categories and scopes tags and titles relevant to your
project. Add a [.changelog.yml] file to your repository root (or use the `--config` option).  See
the [default configuration file](src/assets/changelog.yml) for a starting example.

**Templates**: You can specify your own [Handlebars] template if the output doesn't work for
you. Add a `.changelog.hbs` to your repository root or use the `--template` command line option. See
the [default template](src/assets/changelog.hbs) for a starting example.

**Post Processors**: You can add line post-processors to tweak the output. I use these to simplify
adding links to bug-tracking systems. For example, the commit message can simply state the ticket
number:

```
Fixes: JIRA-1234
```

Then, with a post-processor like the following in the configuration

```yml
output:
  post_processors:
    - {lookup: "JIRA-(?P<id>\\d+)", replace: "[JIRA-$id](https://jira.company.com/view/JIRA-$id)"}
```

the tool will emit the following:

```
Fixes: [JIRA-1234](https://jira.company.com/view/JIRA-1234)
```

[would]:https://chris.beams.io/posts/git-commit/
[changelog]: http://keepachangelog.com/
[commit range]: https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#_commit_ranges
[Handlebars]: http://handlebarsjs.com/
[Homebrew]: https://brew.sh/
[CHANGELOG.md]: CHANGELOG.md
[v0.1.1]: https://github.com/aldrin/git-changelog/tree/v0.1.1
[.changelog.yml]: .changelog.yml
[releases]:https://github.com/aldrin/git-changelog/releases
