# Configuration reference

Each TOML file describes one mod and its optional variants.
Use the [README example](../README.md#quick-start) as a starting point.
The [usage guide](./usage.md) explains how to generate the metadata.

## Metadata

An identifier is the name that CKAN uses for a package.
RKAN requires at least two characters. The first character must be a letter
or digit. The remaining characters can contain letters, digits, underscores,
and hyphens. Letters and digits must be ASCII characters.

| Field            | Required | Meaning or default                                                           |
| ---------------- | -------- | ---------------------------------------------------------------------------- |
| `identifier`     | Yes      | Base CKAN identifier, such as `MyCoolMod`.                                   |
| `name`           | Yes      | Display name.                                                                |
| `repo`           | Yes      | GitHub repository in `owner/repo` form.                                      |
| `authors`        | Yes      | Array of author names.                                                       |
| `tags`           | Yes      | Array of tags supported by RKAN.                                             |
| `license`        | Yes      | License value copied into CKAN metadata.                                     |
| `abstract`       | No       | Description. If omitted, RKAN uses the GitHub repository description.        |
| `ksp_version`    | No       | Compatible KSP version. Defaults to `"1.12"`.                                |
| `asset_match`    | No       | Pattern for selecting a release asset. See below.                            |
| `provides`       | No       | Array of additional package names that this mod supplies. Defaults to empty. |
| `[resources]`    | Yes      | Resource links. The table can be empty.                                      |
| `[[install]]`    | No       | Installation paths. Defaults to no explicit paths.                           |
| `[dependencies]` | No       | Required packages. Defaults to empty.                                        |
| `[recommends]`   | No       | Recommended packages. Defaults to empty.                                     |
| `[conflicts]`    | No       | Incompatible packages. Defaults to empty.                                    |
| `[[variants]]`   | No       | Alternative packages for the mod. Defaults to empty.                         |

Omit `abstract` to use the repository description.
An empty string stays empty. RKAN copies `license` without enforcing allowed
license values.

RKAN accepts these tags:

```text
parts, physics, plugin, app, config, library, flags, agency, suits,
control, convenience, information, editor, planet-pack, graphics,
sound, resources, science, tech-tree, career, combat, comms, buildings,
crewed, uncrewed, stock-inventory, first-person
```

## Release assets

A release asset is a file attached to a GitHub release.
RKAN selects the first asset whose filename matches `asset_match`.
Without this field, it selects the first asset in the release.
The selected file must be a ZIP archive.

The pattern is a regular expression, a rule for matching text.
Use TOML literal strings to preserve backslashes:

```toml
asset_match = '^MyCoolMod-.*\.zip$'
```

The `^` and `$` anchors require the whole filename to match.
The `\.` matches a literal period.
Variants use their own patterns instead of the top-level `asset_match`.

## Resource links

Keep `[resources]` in every configuration, even when you use the defaults.
Omit individual fields to use their default URLs.
An empty string overrides the default with an empty value.

| Field        | Default                                    |
| ------------ | ------------------------------------------ |
| `homepage`   | `https://github.com/<owner>/<repo>`        |
| `repository` | `https://github.com/<owner>/<repo>`        |
| `bugtracker` | `https://github.com/<owner>/<repo>/issues` |
| `manual`     | `https://github.com/<owner>/<repo>/wiki`   |

A value starting with `/` is relative to the GitHub repository URL.
For example, this configuration uses the repository wiki and a custom homepage:

```toml
[resources]
homepage = "https://example.com/my-mod"
manual = "/wiki"
```

## Installation paths

Each `[[install]]` entry maps a path inside the ZIP to an installation destination.
Both `file` and `install_to` are required within each entry.
Use a separate entry for each file or directory to install:

```toml
[[install]]
file = "GameData/MyCoolMod"
install_to = "GameData"
```

RKAN makes sure that each source path exists after extraction.
Without `[[install]]`, RKAN expects an entry at the ZIP root with the generated
identifier as its name. For a variant, that name includes the variant suffix.

RKAN accepts these destinations:

```text
GameData, Missions, Ships, Ships/SPH, Ships/VAB, Ships/@thumbs/VAB,
Ships/Script, Tutorial, Scenarios, GameRoot
```

## Dependencies and conflicts

Use CKAN identifiers as keys in the dependency tables.
The `dependencies` and `recommends` tables accept a version string or an expanded
entry with `version` and optional `help_text`:

```toml
[dependencies]
ModuleManager = "*"
Kopernicus = ">=release-1.12.1-242"

[dependencies.MyTexturePack]
version = "*"
help_text = "Choose a texture resolution."

[recommends]
Deferred = "*"

[conflicts]
MyCoolMod-Legacy = "*"
```

The expanded entry passes `help_text` to CKAN as `choice_help_text`.
Both `dependencies` and `recommends` use the following version forms:

| Form                               | Generated requirement                                   |
| ---------------------------------- | ------------------------------------------------------- |
| `"*"`                              | Any version.                                            |
| `"1.2.3"`, `"=1.2.3"`, `"==1.2.3"` | Exact version.                                          |
| `">=1.2.3"`                        | Minimum version, including `1.2.3`.                     |
| `"<=2.0.0"`                        | Maximum version, including `2.0.0`.                     |
| `">=1.2.3,<=2.0.0"`                | Minimum and maximum versions, including both endpoints. |

RKAN currently treats `>` like `>=` and `<` like `<=`.
It does not interpret caret or tilde ranges.
The `conflicts` table accepts strings, but RKAN currently ignores their version
requirements. Use `"*"` for conflicts.

## Variants

A variant represents an alternative package, such as a texture resolution.
Each variant requires `identifier`, `name`, and `asset_match`.
Append entries like these to the base mod configuration:

```toml
[[variants]]
identifier = "4k"
name = "4K Textures"
asset_match = '^MyCoolMod-4K\.zip$'

[[variants]]
identifier = "8k"
name = "8K Textures"
asset_match = '^MyCoolMod-8K\.zip$'
```

For a base identifier of `MyCoolMod`, RKAN generates `MyCoolMod-4k` and
`MyCoolMod-8k`. Their display names append the variant name in parentheses.
Each variant provides the base identifier and any additional `provides` entries.
Each variant also conflicts with its siblings and the configured conflicts.

Variants share the base metadata, dependencies, recommendations, and installation
paths. RKAN generates only the variant packages when variants exist.
It does not generate an additional base package.

See [Sol-Core.toml](../tests/e2e/Sol-Core.toml) and
[Sol-UranusSystem.toml](../tests/e2e/Sol-UranusSystem.toml) for configurations used
by the repository demo.
