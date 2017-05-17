# fallout

🚧 _Work in Progress_ 🚧

Warning infomation collector for `xcodebuild`.

[![Build Status](https://travis-ci.org/Codezerker/fallout.svg?branch=master)](https://travis-ci.org/Codezerker/fallout)

## Usage

```shell
$ fallout [xcodebuild log file]
```

## Sample output

The output will be a `.json` file with content like this:

```
[
  {
    "message": "<module-includes>:1:1: warning: umbrella header for module 'WarningKit' does not include header '/Users/eyeplum/Projects/fallout/samples/BuildWarningSampler/WarningKit/WNGView.h'",
    "hint": {
      "source": "#import \"Headers/WarningKit.h\"",
      "indicator": "^"
    }
  },
  ...
]
```
