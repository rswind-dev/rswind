import { createHtmlGenerator } from './api'

createHtmlGenerator().then(generator => generator.watch())
