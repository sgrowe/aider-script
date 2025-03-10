# aider-script

aider-script is a command-line tool that streamlines using [aider][] for common tasks, by allowing you to use reusable prompts with templating.

- Define reusable prompt templates in Markdown
- Use variables in your prompt templates
- Specify in your template which files should automatically be loaded into context when running the prompt
- Commit the prompt templates to your repo to share and refine them over time

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

Templates use [tera][] templating syntax. You can use variables from template arguments in your markdown.

## Frontmatter Configuration

The template's frontmatter can specify:

- `args`: Required template arguments
- `read`: Files to be passed as read-only to aider
- `edit`: Files to be edited by aider

These are all optional.

## Installation

Make sure you've installed and set up [aider][] first.

Install using [Cargo][cargo-install]:

```bash
cargo install aider-script
```

## Contributing

Contributions are welcome! Please submit pull requests or open issues on the GitHub repository.

[aider]: https://github.com/Aider-AI/aider
[tera]: https://keats.github.io/tera/
[cargo-install]: https://doc.rust-lang.org/cargo/getting-started/installation.html
