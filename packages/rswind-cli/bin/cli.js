#!/usr/bin/env node
import { cliWithArgs } from './index'

cliWithArgs(["rswind", ...process.argv.slice(2)])