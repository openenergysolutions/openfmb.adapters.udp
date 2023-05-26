# UDP OpenFMB Adapter

This UDP OpenFMB Adapter is an plug-in that can translate UDP protocol into OpenFMB.  This is a sample plug-in that demonstrates how to write a plug-in that can be integrated to [OES OpenFMB adapters](https://openfmb.openenergysolutions.com/).
This sample plug-in uses a fictitious OES plug as an IoT-Ready Plug Control to:
- Read measurement data (Current, Voltage, and Power)
- Turn on/off

This plug-in is written in RUST programming language.  To know more about RUST, please go [here](https://www.rust-lang.org/).

## OpenFMB Profiles

The following profiles are supported by this plug-in:

- SwitchReadingProfile
- SwitchStatusProfile
- SwitchDiscreteControlProfile

## UDP Tags

The following tags are supported to be mapped from UDP to OpenFMB:

- OES.Plug.Current => mapped to "A.net.mag"
- OES.Plug.Voltage => mapped to "Phv.net.mag"
- OES.Plug.Power => mapped to "W.net.mag"
- OES.Plug.Status => mapped to "Pos.phs3.stVal" for SwitchDiscreteControlProfile
- OES.Plug.Command => mapped to "Pos.phs3.ctlVal" for SwitchDiscreteControlProfile

See `template.yaml` for mapping examples

## OES Plug Connection

Change the plug's IP address in `template.yaml`

## OES Specific Configurations

List of OES plugs that the adapter can listen to their HB messages:

```yaml
uncontrollable-plugs:
  - mac-address: 80c955645cd4
    mrid: 5f7436ec-7bf2-4f23-a5c4-fbf25d909ba2
    ip-address: 192.168.87.21
    port: 8556
```

List of OES plugs that the adapter can write commands to:

```yaml
controllable-plugs:
  - mac-address: 80c955645cd4
    mrid: 5f7436ec-7bf2-4f23-a5c4-fbf25d909ba2
    ip-address: 192.168.87.21
    port: 8556
```
Note the `mac-address` field that is used to identify the OES plug, and then mapped back to the mRID used by OpenFMB messages

## UDP Datagram for OES Plugs

Simple UDP implementation is done in `oes` sub-project

## OpenFMB Mappings
The mappings from UDP datagrams to OpenFMB data fields are done using a tree traversal and visitor pattern.  See `adapter-util` sub-project for more information.

## The Adapter
The adapter (main entry of the program) is done in `adapter` sub-project

## Adapter Configurations

There are configurations to support `switch` profiles.
- adapter.yaml
- template.yaml

## Compile and Run

From the project directory, do:

```bash
cargo build
```

```bash
cargo run -- -c adapter.yaml
```
## Docker Build and Run

The `make` command is a convenient way to build and push the docker image to a repository.  Modify the `Makefile` to point to the repository of your choice.  

To build and push, do:

```bash
make docker
```

Run the adapter image:

```bash
docker run -p 8555:8555/udp -v $PWD:/openfmb oesinc/openfmb.adapters.udp -c /openfmb/adapter.yaml
```

Note that in the `adapter.yaml`, the path of the referencing template needs to be modified when the adapter is running in docker container.

When the adapter is running as a stand-alone app:
```yaml
oes-plug:
    enabled: true
    thread-pool-size: 1
    session:
    - path: template.yaml
      session-name: Session
      overrides: []
```

When the adapter is running as a docker container where `/openfmb` is the mounted path in the container:
```yaml
oes-plug:
    enabled: true
    thread-pool-size: 1
    session:
    - path: /openfmb/template.yaml
      session-name: Session
      overrides: []
```

## Contributing

Contributing to the Adapter requires signing a CLA, please email cla@openenergysolutionsinc.com and
request it.