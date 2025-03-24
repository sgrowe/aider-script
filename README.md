# aider-script

aider-script is a CLI tool that streamlines using [aider][] for common tasks, by allowing you to use reusable prompts templates which support being passed arguments when run.

- Define reusable prompt templates in Markdown
- Use variables in your prompt templates
- Specify in your template which files should automatically be loaded into context when running the prompt
- Commit the prompt templates to your repo to share with your team and refine them over time

For example you might have a template which adds an API route to your backend server, and it could take the URL pattern as an argument. You might have another template to add unit tests for a React component given the component name, and another which generates mock data for a third-party API call given the object schema name.

**Table of Contents**

- [Installation](#installation)
- [Usage](#usage)
- [Template Syntax](#template-syntax)
  - [Case Conversion Filters](#case-conversion-filters)
- [Frontmatter Configuration](#frontmatter-configuration)

## Installation

Make sure you've installed and set up [aider][] first.

Install using [Cargo][cargo-install]:

```bash
cargo install aider-script
```

## Usage

### Basic Usage

```bash
aider-script <template-file> [template-arguments...]
```

### Example Template

Each prompt template is a markdown document with a frontmatter section (in YAML) which configures:

- `args`: the names of any arguments to be passed into the template
- `read`: files to be added to the LLM’s context as read-only
- `edit`: files to be edited by aider

All of the config options are optional.

```markdown
---
args:
  - QUERY_NAME # Arguments to be interpolated into the template below
read:
  - "src/schema.graphql" # Files to read into the LLM context
edit:
  - "src/fixtures/mocks.ts" # Files to be edited by aider
---

# Generate mock data for {{ QUERY_NAME }}

Inspect the type definition for {{ QUERY_NAME }} and create some...
```

### Running the Template

```bash
# Basic usage
aider-script example.md myFunctionName
```

Arguments should be passed to the CLI

### Preview the generated message

The `--preview-message` lets you see the message that will be passed to aider without actually running it (handy for double checking you’ve got the template right).

```bash
# Preview without running
$ aider-script --preview-message example.md ListMyFavouritesQuery

Generated message:
------------------

# Generate mock data for ListMyFavouritesQuery

...
```

## Template Syntax

Templates use [tera][] templating syntax (see the [example above](#example-template)). You can use variables from template arguments in your markdown.

### Case Conversion Filters

aider-script provides several case conversion filters to transform strings:

| Filter   | Example output |
| -------- | -------------- |
| `camel`  | `fooBarBaz`    |
| `pascal` | `FooBarBaz`    |
| `kebab`  | `foo-bar-baz`  |
| `snake`  | `foo_bar_baz`  |

Example usage:

```markdown
Create a file called `{{ name | kebab }}.tsx` that exports a component called `{{ name | pascal }}`.
```

Tera also provides several built-in casing filters:

- `upper`: Converts a string to UPPERCASE
- `lower`: Converts a string to lowercase
- `title`: Capitalizes each word inside a sentence
- `capitalize`: Returns the string with all its characters lowercased apart from the first char which is uppercased

See the [Tera documentation](https://keats.github.io/tera/docs/#built-ins) for more details.

## Frontmatter Configuration

The template's frontmatter can specify:

- `args`: Required template arguments
- `read`: Files to be passed as read-only to aider
- `edit`: Files to be edited by aider

These are all optional.

[aider]: https://github.com/Aider-AI/aider
[tera]: https://keats.github.io/tera/
[cargo-install]: https://doc.rust-lang.org/cargo/getting-started/installation.html
