# DataCenterTopology

## Overview

This is an updated repo to store the various working parts of the Data Center Topology project for the Google Summer of Code, 2016.

The aim of this project is to programatically discover which machines in an existing Ceph deployment are behind the same switch, and likely on the same rack, using ARP requests. After gathering this information it generates a new crushmap.

It is broken down into two parts: a single controller and a series of subordinate nodes which gather information for said controller. Due to the limitations with Juju charms the only sane method for communication between multiple nodes is in a node/server relationship.

Currently the project is not fully functional. Ceph does not recognise the encoded crushmap as it stands, and will throw errors if it set in Ceph. The author was not yet able to determine if this was a fault of their code, the Crushtool library, or Ceph itself.

## Use

1. Create a config.yaml file with the following:

	> dct-controller:
  	>   num-units: n
  	
  	Where n is the number of Ceph OSDs present in the system.
2. Deploy the dct-controller charm to a machine running a Ceph Mon. **This is exremely important!**
3. Deploy the dct-node charm
4. Relate the dct-node charm to either your Ceph charm or your Ceph-osd charm
5. After the dct-node charms have finished deploying, relate the dct-controller to dct-node
6. Use `juju run-action dct-controller/0 begin-discovery` to start network discovery
7. After units report discovery is complete, use `juju run-action dct-controller/0 create-crushmap` to create a crushmap

The author strongly recommends having `juju debug-log` running to keep an eye on the controller charm. This charm is not without its bugs, and will sometimes break. In order to restart the network discovery you currently must remove and re-add the relation between the node and controller.

**_Please check the outputted crushmap before use!_ Use of these charms is at your own risk! The author cannot garuntee that any crushmap generated here will work for your unique Ceph deployment.**


## Limitations
Currently it only operates on an IPv4 network, as IPv6 does not have a 1 to 1 equivalent for ARP requests.

