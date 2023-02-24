<h1 aling="center"><code>🦜: NuShell and FishShell</code></h1>
<h2 aling="center"><i> Scripting Tests </i></h2>

----
1. [Wah ?](#wah-)
2. [Dirs](#dirs)
3. [Special Commands](#special-commands)
   1. [Capturing the tabular output](#capturing-the-tabular-output)
4. [Regarding Loop with NU](#regarding-loop-with-nu)

----

# Wah ? 

1. Testing out fish and nushell scripts
2. Extremely important since there are the most popular shells beign used today 

# Dirs

- Dir description 

|           N           |                                               🏯                                               |
| :-------------------: | :-------------------------------------------------------------------------------------------: |
|  [`nu_cw`](./nu_cw/)  | Nushell script, CLI that takes arguments and loops through commands and writes output to text |
| [`nu_out`](./nu_out/) |                                   Redirecting output of com                                   |

# Special Commands 

## Capturing the tabular output 

Commmands 
```rs 
ls -f .. |
table |
ansi strip out+err> capt.txt 
```
- Above command output is as follows 

```rs
╭───┬──────────────┬─────────┬────────┬────────────────╮
│ # │     name     │  type   │  size  │    modified    │
├───┼──────────────┼─────────┼────────┼────────────────┤
│ 0 │ ../LICENSE   │ file    │ 1.2 KB │ 3 hours ago    │
│ 1 │ ../README.md │ file    │  576 B │ 33 minutes ago │
│ 2 │ ../g         │ symlink │    6 B │ 3 hours ago    │
│ 3 │ ../l         │ dir     │ 4.1 KB │ 2 hours ago    │
│ 4 │ ../nu_cw     │ dir     │ 4.1 KB │ 36 minutes ago │
│ 5 │ ../nu_out    │ dir     │ 4.1 KB │ 3 minutes ago  │
╰───┴──────────────┴─────────┴────────┴────────────────╯
```
- Clean with no ansi colors 


# Regarding Loop with NU 
1. This is not straight forward like in other shells
2. You will have to write a rust command just for this 
3. For quick work you cant use nushell , since its still under dev , and can be considered to be a comoiled language 

