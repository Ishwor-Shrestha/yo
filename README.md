# YO : Make flutter development easier

### Prerequisite

- Linux or Mac OS
- Bash

 `Note : For the Windows users, you can run the script in git bash.`

### Installation

#### 1. Download the script
```shell script
curl -o ~/Downloads/yo https://raw.githubusercontent.com/Ishwor-Shrestha/yo/master/yo
```
Make sure you replace `~/Downloads/` with your directory.

#### 2. Append the following line at the end in your `.bashrc` or whatever is equivalent to `.bashrc`
```shell script
source ~/Downloads/yo  
```

#### 3. Activate the script
```shell script
 source ~/.bashrc  
```

### Usage

#### Initialize a project

Run the following command at the root of your project.
```shell script
yo init
```

#### Set package path
You'll need to set the package path (Path to the directory where your flutter packages reside).

```shell script
yo set -p <path_to_package(s)>
```

#### Get flutter dependencies `flutter pub get`
After you've set your package path, you can then run the following commands.

Get dependencies of all your packages.
```shell script
yo get
```

You can also get the dependencies of a single package.
```shell script
yo get <package_directory_name>
```

There may be a case when you have a simple project with the package content at the root of your project. In such cases, just set the root of the project as your package path. And you can use either of these commands.
```shell script
yo get
```

Use this to get dependencies at the current directory.
```shell script
yo get /c
```

Or this when you're deep inside your project and don't want the hassle to navigate all the way to the top. This will run `yo get` at the root of the package directory.
```shell script
yo get /r
```

#### Run your tests `flutter test`
Run the tests in all your packages.
```shell script
yo test
```

Run the tests in a specific package.
```shell script
yo test <package_directory_name>
```

Run tests in the current package.
```shell script
yo test /c
```

Run tests in the root of the package directory.
```shell script
yo test /r
```

#### Run your tests coverage `flutter test --coverage`
Run the tests coverage in a specific package.
```shell script
yo test <package_directory_name> --coverage
```

Run tests coverage in the current package.
```shell script
yo test /c --coverage
```

Run tests coverage in the root of the package directory.
```shell script
yo test /r --coverage
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

Clean the current package.
```shell script
yo clean /c
```

Clean the root of the package directory.
```shell script
yo clean /r
```

#### Upgrade your flutter packages `flutter pub upgrade`
Upgrade all your packages.
```shell script
yo upgrade
```

Upgrade a specific package.
```shell script
yo upgrade <package_directory_name>
```

Upgrade the current package.
```shell script
yo upgrade /c
```

Upgrade the root of the package directory.
```shell script
yo upgrade /r
```

#### Run `build_runner watch` to build and start watching
- Learn about [build_runner](https://pub.dev/packages/build_runner).

Use this command to run the `build_runner`.

```shell script
yo watch <package_directory_name>
```

Run `build_runner` in the current directory.
```shell script
yo watch /c
```

Run `build_runner` in the root of the package directory.
```shell script
yo watch /r
```

#### Run `build_runner build` to build
- Learn about [build_runner](https://pub.dev/packages/build_runner).

Use this command to run the `build_runner`.

```shell script
yo build <package_directory_name>
```

Run `build_runner` in the current directory.
```shell script
yo build /c
```

Run `build_runner` in the root of the package directory.
```shell script
yo build /r
```

#### Check if your flutter packages have outdated dependencies `flutter pub outdated`
Check in all your packages.
```shell script
yo outdated
```

Check in a specific package.
```shell script
yo outdated <package_directory_name>
```

Check in the current package.
```shell script
yo outdated /c
```

Check in the root of the package directory.
```shell script
yo outdated /r
```

#### Yo also supports adb commands
To start the adb server
```shell script
yo start-server
```

To kill the adb server
```shell script
yo kill-server
```

To restart the adb server
```shell script
yo restart-server
```

To start the adb server listening on TCP on PORT 5555
```shell script
yo tcpip
```

To connect to the device with IP
```shell script
yo connect <ip_address>
```

#### Start Mockoon
- Learn about [Mockoon](https://mockoon.com/).

In order to start mockoon, use the following command. Make sure you've installed mockoon first.
```shell script
yo mock
```

#### Run your script
`yo` can also run your own bash scripts.

Just add the path to your scripts.

```shell script
yo set -s <path_to_scripts>
```

Then you'll be able to run your script like
```shell script
yo run <script_name> <arguments(s)>
```
Arguments are optional. So, pass them when your script requires them to be passed.

#### Get help
You can see all the help info for `yo`'s subcommands.
```shell script
yo help
```

