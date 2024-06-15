import type { GeneratorConfig } from './types'

export class Generator {
  generateWith(candidates: Array<[string, string]>): GenerateResult
  generate(): string
  generateString(input: string, kind?: 'html' | 'ecma' | 'unknown'): GenerateResult
  generateCandidate(input: Array<string>): GenerateResult
}

export function createGenerator(options?: GeneratorOptions | undefined | null): Generator

export interface GenerateResult {
  css: string
  kind: ResultKind
}

export interface GeneratorOptions {
  base?: string
  config?: string | false | GeneratorConfig
  watch?: boolean
  parallel?: boolean
}

export enum ResultKind {
  Cached = 0,
  Generated = 1,
}
