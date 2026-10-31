# Reference

Comprehensive specifications, API schemas, and technical references for **Unframed**. These documents describe exact formats, options, and behaviors without tutorial narrative.

---

## Technical Specifications

| Reference | Contents |
| :--- | :--- |
| [CLI Options](cli.md) | Full command-line interface flags, positional arguments, standard input piping, and exit status. |
| [Command Surface](commands.md) | Built-in commands, arguments, return semantics, default aliases, and execution lifecycle. |
| [Configuration Format](configuration.md) | Structure of `imv`-compatible INI files, section definitions (`[options]`, `[aliases]`, `[binds]`), search precedence, and color schemas. |
| [IPC Protocol](ipc-protocol.md) | Wire protocol, UNIX domain socket paths, message framing, environment variable contracts, and client usage. |
| [Format Support](format-support.md) | Supported vector and raster formats, Netpbm family parsing rules, animated multi-frame specifications, and limitations. |

For architectural discussions and design rationale, consult the [Explanation](../explanation/index.md) section.
