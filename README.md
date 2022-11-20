<p align="center">
    <img src="https://user-images.githubusercontent.com/23109089/171970710-a72e7506-9e55-4f1c-8089-bd331cd68aa4.png#gh-dark-mode-only" alt="WK's dark logo" />
    <img src="https://user-images.githubusercontent.com/23109089/171970769-e07ffa29-5a19-4893-9610-8c1a08e8972e.png#gh-light-mode-only" alt="WK's light logo" />
</p>


<p align="center">
WK is a CLI tool to create, manager and access workspaces.
</p>

## How to install

First, clone the repository:

```sh
git clone https://github.com/henrybarreto/wk.git
```

Go to the repository folder:

```sh
cd wk
```

Then, install the wk through the script
    
```sh
./scripts/install.sh
```

## How to use

**Save a workspace**

```sh
wk --save go ~/Documents/Projects/Go
```

**Go to a saved workspace**

```sh
wk go
```
> It is worth to say that `wk` does not change the directory itself. Currently, it just “split out” a `cd` command with the path to the workspace informed, being necessary to use a shell script to get that output, execute the `cd` command and open the shell into the directory.

**Show saved workspaces**
```sh
wk --list
```
> not implemented yet

**Show usage**
```sh
wk --help
```

### How to configure
When you save a workspace, the file `~/.wk.ron` is created/populated with that data, but you can also create/edit this file manually.

```ron
(workspaces:[(name:"wk",path:"<PATH TO WK>")])
```
