# YO : Make flutter development easier

    Usage: yo <COMMAND>

    Commands:
      init    Initialize yo
      config  Open config
      scan    Scan flutter project directory
      get     Get flutter dependecies
      build   Run build runner for flutter projects
      test    Run flutter tests
      clean   Clean flutter project
      help    Print this message or the help of the given subcommand(s)

    Options:
      -h, --help  Print help

### Installation

#### 1. Download the binary

```shell script
curl -o <path_to_dir>/yo https://raw.githubusercontent.com/Ishwor-Shrestha/yo/master/binaries/<platform>/yo
```

Make sure you replace `<path_to_dir>` with your directory and `<platform>` with your respective platform in the binaries
directory.

#### 2. Add the binary to your path

### Usage

#### Initialize a project

Run the following command at the root of your project.

```shell script
yo init
```

#### Open the config file for editing

```shell script
yo config
```

Check the config [guide](https://github.com/Ishwor-Shrestha/yo/config.md) for more information.

#### Run the `scan` command to scan your flutter project

```shell script
yo scan
```

This scans your flutter project directory to map out all possible packages. In a normal project this means that only a
single package will exist. But if your package is a `monorepo` then a list is maintained. It is recommend to run this
command once in a while to keep the mapping updated.

#### Get flutter dependencies `flutter pub get`

Get dependencies of all your packages.

```shell script
yo get
```

You can also get the dependencies of a single package.

```shell script
yo get <package_directory_name>
```

Here, package directory name is the name of the directory `pubspec.yaml` resides in.

#### Run your tests `flutter test`

Run the tests in all your packages.

```shell script
yo test
```

Run the tests in a specific package.

```shell script
yo test <package_directory_name>
```

#### Clean your flutter packages `flutter clean`

Clean all your packages.

```shell script
yo clean
```

Clean a specific package.

```shell script
yo clean <package_directory_name>
```

#### Run `build_runner build` to build

*   Learn about [build\_runner](https://pub.dev/packages/build_runner).

Use this command to run the `build_runner`.

```shell script
yo build <package_directory_name>
```

#### Get help

You can see all the help info for `yo`'s subcommands.

```shell script
yo help
```
