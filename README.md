## IP Inspect

ipinspect is a tool for inspecting information of given IP network.

If you often have following situations, this tool is for you.

- How many hosts can I use in this network?
- Is this address included in this network?
- What the netmask in other form is?

## How to use it?
- IPv4

```console
$ ipinspect 192.0.2.0/24
---
YOUR INPUT         192.0.2.0/24
NETWORK ADDRESS    192.0.2.0
HOST ADDRESS RANGE 192.0.2.1 ... 192.0.2.254 (COUNT: 254)
BROADCAST ADDRESS  192.0.2.255
NETMASK            255.255.255.0 (/24)
```
