<p align="center">
    <img src="https://user-images.githubusercontent.com/23109089/171970710-a72e7506-9e55-4f1c-8089-bd331cd68aa4.png#gh-dark-mode-only" alt="WK's dark logo" />
    <img src="https://user-images.githubusercontent.com/23109089/171970769-e07ffa29-5a19-4893-9610-8c1a08e8972e.png#gh-light-mode-only" alt="WK's light logo" />
</p>


<p align="center">
WK is a CLI tool to create, manager and access workspaces.
</p>

## How to use

### Go to workspace
It is worth to say that `wk` does not change the directory itself. Currently, it just “split out” a `cd` command with the path to the workspace informed, being necessary to use a shell script to get that output, execute the `cd` command and open the shell into the directory.

```wk go```

### Save a workspace
```wk --save go ~/Documents/Projects/Go```

### Show saved workspaces
> not implemented yet
```wk --list```

### Show usage
```wk --help```

### How to configurate
You can configurate your workspaces through a configuration file locate on `~/.wk.ron`.

```ron
(workspaces:[(name:"wk",path:"<PATH TO WK>")])
```
