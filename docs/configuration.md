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

You can choose the format that you prefer.

## Advanced Configuration

### `utilities`

The `utilities` field is an array of utility definitions. Each utility definition is an object with the following fields:
| Field | Type | Description |
| ----- | ---- | ----------- |
| `key` | `string` | The key of the utility, e.g. `bg` |
| `css` | `string` | The css handler for the utility, e.g. `background-color: $1` |
| `modifier` | `ValueDef` | The modifier for the utility, e.g. `bg-blue-500/50` |
| `theme_key` | `string` | The theme key for the utility, will read from `theme` by this key later, e.g. `colors` |
| `validator` | `DataType \| PropertyKey` | The type validator for the utility, for inferring value of `arbitrary values`, could either be a css data type or a property key, e.g. `percentage`, `font-size` |
| `negative` | `boolean` | Whether the utility supports negative values |
| `fraction` | `boolean` | Whether the utility supports fraction values, e.g. `w-1/2` |

e.g. A `rswind.config.json` defining a custom utility like [outline-color](https://developer.mozilla.org/en-US/docs/Web/CSS/outline-color):

```json [rswind.config.json]
{
  "utilities": [
    {
      "key": "outline-color",
      "css": {
        "outline-color": "$0:color"
      },
      "modifier": {
        "type": "percentage",
        "theme_key": "opacity"
      },
      "theme_key": "colors",
      "type": "color"
    }
  ]
}
```
