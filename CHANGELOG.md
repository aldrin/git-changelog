## v0.3.0 (2018-02-01)

- [Restructure inputs and outputs](https://github.com/aldrin/git-changelog/pull/12)

#### Features
    
- Markdown output is tidier as includes revision range and timestamps in
  the generated report. The report can also be produced to JSON to
  support dynamic renderings (if required).

#### Breaking Changes
    
- The input to the tool is rearranged and now separates project
  conventions from output preferences under separate YAML nodes. This
  implies that external configuration files would require some
  tweaks. See [.changelog.yml](.changelog.yml) and
  [src/assets/changelog.yml](src/assets/changelog.yml) for the new
  format.

## 0.2.1 (2017-12-23)

#### Summary

- Release Windows Binaries ([#7](https://github.com/aldrin/git-changelog/pull/7))

##### Features

- Windows binaries are now available with releases.

## 0.2.0 (2017-12-10)

#### Summary

- Simplify configuration and group by tag titles ([#2](https://github.com/aldrin/git-changelog/pull/2))
- Improve configuration file lookup and support post-processors ([#3](https://github.com/aldrin/git-changelog/pull/3))

##### Features

- Simplify configuration by omitting tags, defaults, etc. All we
    need now are simple keyword->title mapping and keywords without
    titles are skipped from the report. The aggregation is now done
    on title (instead of tag keyword) which allows us to group
    multiple tags into one report section. For example, with tags
    `break => Breaking Change` and `braek => Breaking Change` we
    can accommodate simple typos.
  

- The tool now automatically uses a configuration file
    `.changelog.yml` in the current directory. If a file with that
    name doesn't exist in the current directory, it looks up the
    directory tree in the parent directories as long as it reaches
    the root. This simplifies project specific configuration by
    checking in a `.changelog.yml` in the repo root.
  

- The configuration can now specifiy post-processors
    that look for specific markers like (e.g. `JIRA-12345`) and
    replaces them with suitable replacements. This makes adding
    links like [JIRA-1234](http://jira.company.com/view/JIRA-1234)
    easy. In general, a post-processor is a `(lookup,replacement)`
    tuple where a `lookup` is a regex with named capture groups
    like `JIRA-(?P<ticket>\\d+)`) and
    [replacement](https://doc.rust-lang.org/regex/regex/index.html#example-replacement-with-named-capture-groups)
    is a simple string that refers to the named capture groups like
    `[JIRA-$ticket](http://jira.company.com/view/JIRA-$ticket)`


##### Breaking Changes

- Configuration format has changed and old configuration
    files will need some tweaks.

## 0.1.0 (2017-11-24)

### Summary

-  Demonstrate tagging conventions for commit messages

#### Features

- **Categorized Changes:** Lines that begin with a `- <tag>:` can be used to categorize
  changes. Everything that follow a the prefix is tagged with the chosen category. For example, this
  text describes the tool feature that categorizes changes. Among others, `feature` is an
  out-of-the-box tag. You can define tags that make sense for your project.

- **Scoped Changes**: For larger projects, you can introduce another level of organization using
  *scopes* (e.g. `API`, `Documentation`, `Benchmarks`, etc.) To specify a scope, use the `-
  tag(scope):` prefix as done by the next item. Lines that don't specify a `scope` like this or the
  previous item fall into the `default` scope. As with tags, you can define scopes that make sense
  for your project (or not use them at all).
