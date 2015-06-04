#RSGNL

This will be a rust interface for libnl-genl and Generic Netlink. It will be built ontop of the rsnl library, in turn an interface for libnl.

The reasoning is to keep the original layout of libnl and libnl-genl. This means that you only link against genl if you're using RSGNL.
