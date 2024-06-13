#!/usr/bin/env node
const process = require('node:process')
const cli = require('../bindings/index.js')

cli.runCli(['rswind', ...process.argv.slice(2)])
