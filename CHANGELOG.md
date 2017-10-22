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
