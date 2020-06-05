# TIMESTAR

Small Rust app that let user run a pomodoro clock in order to keep focus on tasks


## Requirements

Since this app uses a notification library that has [dbus-rs](https://github.com/diwic/dbus-rs) as it's own dependency some libraries are required

- Development
    - openSusSE
        - dbus-1-devel
        - pkgconf-pkg-config


## Run
Timestar can be launched alone or with a repetitionargument

##### Launching alone

```bash
timestar
```
> This case is the default one, when timestar cycle is finished will ask if user wants to repeat it

##### Launching with repetition argument

```bash
timestar 5
```
> This case number 5 represents the number of whole timestar cycles that will be launched,  
> it means that 'finish question' it won't be launched until last cycle

> This case is useful for other unix users like OsX or for those users who uses notification systems  
> that doesn't handle questions, like **dunst**
