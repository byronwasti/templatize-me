# Templatize-Me

A super simple and basic templating renderer. It is just a wrapper around the Tera crate to use as a CLI tool.

The most basic way to run is to set up a `context.toml` and a `template` file (Tera templating docs: https://tera.netlify.app/docs/) and then run the following which will output to stdout:
```
$ templatize-me
```

You can also customize the context or template with:
```
$ templatize-me -c config.json -t someFileTemplate.html
```

Currently this supports JSON or TOML.
