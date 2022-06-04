<p align="center">
    <img src="https://user-images.githubusercontent.com/23109089/171970560-b27bd17f-1d7f-4d8f-aa42-7d51e0081d7e.png#gh-dark-mode-only" alt="WK's logo" />
    <img src="https://user-images.githubusercontent.com/23109089/171970710-a72e7506-9e55-4f1c-8089-bd331cd68aa4.png#gh-light-mode-only" alt="WK's logo" />
</p>

<p align="center">
WK is a CLI tool to create aliases for directories ( workspaces ).
</p>

## How to use

### Go to workspace
```wk go```

### Save a workspace
```wk --save go ~/Documents/Projects/Go```

### Show saved workspaces
```wk --list```

### Show usage
```wk --help```

### How to configurate
You can configurate your workspaces through a configuration file locate on `~/.wk.ron`.

```ron
{
    "go": "~/Documents/Projects/Go",
    "rust": "~/Documents/Projects/Rust",
    "typescript": "~/Documents/Projects/Typescript",
    "java": "~/Documents/Projects/Java"
}

```

