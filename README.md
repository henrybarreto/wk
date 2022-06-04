<p align="center">
    <img src="WK.svg" alt="WK's logo" />
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

