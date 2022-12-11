<p align="center">
    <img src="https://user-images.githubusercontent.com/23109089/171970710-a72e7506-9e55-4f1c-8089-bd331cd68aa4.png#gh-dark-mode-only" alt="WK's dark logo" />
    <img src="https://user-images.githubusercontent.com/23109089/171970769-e07ffa29-5a19-4893-9610-8c1a08e8972e.png#gh-light-mode-only" alt="WK's light logo" />
</p>


<p align="center">
WK is a CLI tool to create, manage and access workspaces.
</p>

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
