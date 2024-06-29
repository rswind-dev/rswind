import type { OrderingKey } from './ordering'
import type { Property } from './property'

export type * from './ordering'
export type * from './property'

export interface GeneratorConfig {
  /**
   * The glob pattern to match input files
   *
   * @default ['./**\/*.{html,js,jsx,mjs,cjs,ts,tsx,mts,cts,vue,svelte,mdx}']
   */
  content?: string[]
  /**
   * How to handle `dark:` variant, can be `media` or `selector`
   */
  darkMode?: string
  theme?: UserTheme

  // rswind specific config
  /**
   * User defined static utilities e.g. `flex`
   */
  features?: Features
  staticUtilities?: Record<string, Record<string, string> | [string, Record<string, string>]>
  /**
   * User defined dynamic utilities, e.g. `bg-blue-500`
   */
  utilities?: UtilityBuilder[]
  [k: string]: unknown
}

export interface Features {
  /**
   * Use a lexer to parse candidate, default to `true` if set to `false`, the parser will use regex to parse candidate
   */
  strict_mode: boolean
  [k: string]: unknown
}

export type Rule = Record<string, string | Record<string, string>>

type ExcludeThemeKey = Exclude<ThemeKey, 'colors' | 'keyframes'>

export type UserThemeBase = {
  colors?: Record<string, Record<string, string> | string>
  keyframes?: Record<string, Rule>
} & { [K in ExcludeThemeKey]?: Record<string, string> }

export interface UserTheme extends UserThemeBase {
  extend?: UserThemeBase
}

export interface UtilityBuilder {
  /**
   * The type validator for the utility, only used at `arbitrary values`
   *
   * e.g. `length-percentage` for `width`
   */
  type?: string | null
  additionalCss?: Rule | null
  /**
   * The css handler for the utility, e.g. `background-color: $1`
   */
  css?: Record<string, string> | null
  /**
   * The key of the utility， e.g. `bg`
   */
  key: string
  /**
   * The modifier for the utility, e.g. `bg-blue-500/50 <-`
   */
  modifier?: ValueDefinition | null
  /**
   * The ordering key for the utility
   */
  orderingKey?: OrderingKey | null
  /**
   * Whether the utility supports fraction values, e.g. `w-1/2`
   */
  supportsFraction?: boolean
  /**
   * Whether the utility supports negative values
   */
  supportsNegative?: boolean
  /**
   * The theme key for the utility, will read from `theme` by this key later, e.g. `colors`
   */
  theme?: ThemeKey | null
  /**
   * The wrapper selector for the utility
   */
  selector?: string | null
}

export interface ValueDefinition {
  type?: Property | DataType | null
  theme?: ThemeKey | null
}

export type ThemeKey =
    // Responsiveness
  | 'screens'
  | 'supports'
  | 'data'
    // Reusable base configs
  | 'colors'
  | 'spacing'
    // Components
  | 'container'
    // Utilities
  | 'inset'
  | 'zIndex'
  | 'order'
  | 'gridColumn'
  | 'gridColumnStart'
  | 'gridColumnEnd'
  | 'gridRow'
  | 'gridRowStart'
  | 'gridRowEnd'
  | 'margin'
  | 'aspectRatio'
  | 'height'
  | 'maxHeight'
  | 'minHeight'
  | 'width'
  | 'maxWidth'
  | 'minWidth'
  | 'flex'
  | 'flexShrink'
  | 'flexGrow'
  | 'flexBasis'
  | 'borderSpacing'
  | 'transformOrigin'
  | 'translate'
  | 'rotate'
  | 'skew'
  | 'scale'
  | 'animation'
  | 'keyframes'
  | 'cursor'
  | 'scrollMargin'
  | 'scrollPadding'
  | 'listStyleType'
  | 'columns'
  | 'gridAutoColumns'
  | 'gridAutoRows'
  | 'gridTemplateColumns'
  | 'gridTemplateRows'
  | 'gap'
  | 'space'
  | 'divideWidth'
  | 'divideColor'
  | 'divideOpacity'
  | 'borderRadius'
  | 'borderWidth'
  | 'borderColor'
  | 'borderOpacity'
  | 'backgroundColor'
  | 'backgroundOpacity'
  | 'backgroundImage'
  | 'gradientColorStops'
  | 'backgroundSize'
  | 'backgroundPosition'
  | 'fill'
  | 'stroke'
  | 'strokeWidth'
  | 'objectPosition'
  | 'padding'
  | 'textIndent'
  | 'fontFamily'
  | 'fontSize'
  | 'fontWeight'
  | 'lineHeight'
  | 'letterSpacing'
  | 'textColor'
  | 'textOpacity'
  | 'textDecorationColor'
  | 'textDecorationThickness'
  | 'textUnderlineOffset'
  | 'placeholderColor'
  | 'placeholderOpacity'
  | 'caretColor'
  | 'accentColor'
  | 'opacity'
  | 'boxShadow'
  | 'boxShadowColor'
  | 'outlineWidth'
  | 'outlineOffset'
  | 'outlineColor'
  | 'ringWidth'
  | 'ringColor'
  | 'ringOpacity'
  | 'ringOffsetWidth'
  | 'ringOffsetColor'
  | 'blur'
  | 'brightness'
  | 'contrast'
  | 'dropShadow'
  | 'grayscale'
  | 'hueRotate'
  | 'invert'
  | 'saturate'
  | 'sepia'
  | 'backdropBlur'
  | 'backdropBrightness'
  | 'backdropContrast'
  | 'backdropGrayscale'
  | 'backdropHueRotate'
  | 'backdropInvert'
  | 'backdropOpacity'
  | 'backdropSaturate'
  | 'backdropSepia'
  | 'transitionProperty'
  | 'transitionTimingFunction'
  | 'transitionDelay'
  | 'transitionDuration'
  | 'willChange'
  | 'content'

export type DataType =
  | 'color'
  | 'length'
  | 'length-percentage'
  | 'percentage'
  | 'number'
  | 'ident'
  | 'image'
  | 'time'
  | 'angle'
  | 'any'
