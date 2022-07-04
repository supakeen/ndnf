# ndnf

``ndnf`` or *nanodnf* is a small implementation of a D-Bus API and client to
perform DNF package related actions. Its main goal is to be small and nimble
while still providing the expected functionality.

It runs as a replacement for the standard [dnfdaemon](https://github.com/manatools/dnfdaemon) 
and [dnf](https://github.com/rpm-software-management/dnf) and provides the
**PackageKit** and **dnfdaemon** D-Bus APIs.

## Executables

### ndnf

`ndnf` is a command line frontend.

### ndnf-compat

`ndnf-compat` is a command line compatible `ndnf` frontend that can be aliased
to `dnf`.

### ndnf-daemon

`ndnf-daemon` provides `PackageKit` and `dnfdaemon` D-Bus APIs.

## Ecosystem

The packaging ecosystem is wide and diverse. How all the bits fit together
is difficult to figure out. I've written down what I've encountered and what I
think each of their roles is.

### RPM

RPM, or the *RPM Package Manager*. RPM makes packages which consist of a built
version of software plus some metadata or consist of the source plus
instructions on how to build the software.

These packages list all files to be installed and/or removed.

### yum

yum, or the *Yellowdog Updater, Modified*. This is a package manager that
handles fetching RPMs from repositories and their dependencies.

### DNF

DNF, or *Dandified YUM*. This is a package manager that handles fetching RPMs
from repositories and their dependencies.

[dnf](https://github.com/rpm-software-management/dnf)

### libdnf

[libdnf](https://github.com/rpm-software-management/libdnf)

### PackageKit

[PackageKit](https://github.com/PackageKit/PackageKit)

### rpm-ostree

[rpm-ostree](https://github.com/coreos/rpm-ostree)

### hawkey

### libsolv

### librepo

### libcomps
