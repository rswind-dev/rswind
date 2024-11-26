# Configuration

## Configuration File

The configuration file is a `rswind.config.{json,toml,yaml,yml}` file that is placed in the root of your project. It can be used to configure the JIT engine and other options.

Here is an example of a configuration file:

::: code-group

```json [rswind.config.json]
{
  "contents": ["./src/**/*.{js,jsx,ts,tsx,vue}"],
  "darkMode": "class",
  "theme": {
    "extend": {
      "colors": {
        "primary": "#3490dc",
        "secondary": "#ffed4a",
        "danger": "#e3342f"
      }
    },
    "custom": {
      "...": "$spacing",
      "large": "32px"
    }
  }
}
```
```toml [rswind.config.toml]
contents = [ "./src/**/*.{js,jsx,ts,tsx,vue}" ]
darkMode = "class"

[theme.extend.colors]
primary = "#3490dc"
secondary = "#ffed4a"
danger = "#e3342f"

[theme.custom]
"..." = "$spacing"
large = "32px"
```

```yaml [rswind.config.yaml]
contents:
  - ./src/**/*.{js,jsx,ts,tsx,vue}
darkMode: class
theme:
  extend:
    colors:
      primary: '#3490dc'
      secondary: '#ffed4a'
      danger: '#e3342f'
  custom:
    '...': $spacing
    large: 32px
```

:::
