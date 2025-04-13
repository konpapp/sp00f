# sp00f

Capturing DNS queries.

## Linux

Run `ip a` to list network interfaces.

```bash
cargo build && sudo ./target/debug/sp00f
```

Generate DNS queries with `nslookup example.com @8.8.8.8`

## Windows

Run `ipconfig /all` to list network interfaces.

After doing the prerequisites listed in [libpnet readme](https://github.com/libpnet/libpnet), you should have a [WinPcap developers pack](https://www.winpcap.org/devel.htm) folder. Run from powershell as admin with:

```powershell
$Env:LIB="path_to_Packet.lib_subfolder"; cargo run
```

Generate DNS queries with `dig example.com @8.8.8.8`
