<p align="center">
    <img src="https://user-images.githubusercontent.com/23109089/171970710-a72e7506-9e55-4f1c-8089-bd331cd68aa4.png#gh-dark-mode-only" alt="WK's dark logo" />
    <img src="https://user-images.githubusercontent.com/23109089/171970769-e07ffa29-5a19-4893-9610-8c1a08e8972e.png#gh-light-mode-only" alt="WK's light logo" />
</p>


<p align="center">
Simplify your directory navigation with WK, the human go-to tool
</p>

WK is a command-line interface (CLI) tool that allows users to navigate to directories more easily by saving and managing "workspaces". A workspace consists of a name and a path and can be saved, removed, or listed by the user. The configuration of workspaces is stored in a file and can be loaded, modified, and saved by the WK application.

## How to install
> In order to install `wk`, you need the `cargo` installed.

First, clone the repository:

```sh
git clone https://github.com/henrybarreto/wk.git
```

Go to the repository folder:

```sh
cd wk
```

Then, install it:
    
```sh
make install
```

## How to use

**Save a workspace**

```sh
wk save home $(pwd)
```

**Go to a saved workspace**

```sh
wk go home
```

**Remove a saved workspace**

```sh
wk remove home
```

**Show saved workspaces**
```sh
wk list
```

**Show usage**
```sh
wk help
```

## How to configure
When you save a workspace, the file `~/.wk.ron` is created/populated with that data, but you can also create/edit this file manually.

```ron
(workspaces:[(name:"wk",path:"<PATH TO WK>")])
```
