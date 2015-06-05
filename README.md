#RSGNL

The start of a rust interface for libnl-genl and Generic Netlink. It will be built ontop of the rsnl library, in turn an interface for libnl.

The reasoning is to keep the original layout of libnl and libnl-genl. This means that you only link against genl if you're using RSGNL.


### Dependencies

* The rsnl interface for libnl

* nl-3
* nl-genl-3

#### License

MIT
