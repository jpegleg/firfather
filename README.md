![cdlogo](https://carefuldata.com/images/cdlogo.png)

# firfather 🌲

The firfather project is a CICD system for creating [serotinous cones](https://github.com/jpegleg/serotinous-cone/tree/main).

A firfather server is intended to use Alpine Linux, but could be adapted to another linux distribution.

Firfather is made of many components but has five software systems that work together within the project:

- 📦 `arrival`, a packaging system for making APK (Alpine Package Keeper) packages for subcomponents of serotinous-cone
- 🏔️ `winter`, an enforcement controller daemon
- 🌱 `cone`, a continous deployment system in a function that can be a scheduled job
- 🕸️ `hadronyche`, a continuous integration system
- 🕷️ `mygalomorphae`, a utility API for firfather observability and control

Within those components we find `kube-rs`, `actix-web`, `Open Tofu`, `Ansible`, and much more.

In addition to the five main components within firfather, there are also support scripts for installation of a new firfather server.

## Arrival

A CLI program that makes `apk`, essentialy a rusted shell wrapper for `alpine-sdk`.

## Winter 

A daemonized and detached kuberenetes and SSH controller that enforces the state, performing deployment and recovery actions
if cones are unhealthy.

## Cone 

A function that builds a new [serotinous cone](https://github.com/jpegleg/serotinous-cone/tree/main).

## Hydronyche 

<b>hydronyche</b> - (biology) <i>a genus</i> of venomous Australian <i>funnel web</i> spiders

Hydronyche is a very specific CI system for serotinous cones that includes HTML reports, SBOMs, and tests.

## Mygalomorphae 

<b>hydronyche</b> - (biology) <i>an infraorder</i> of many species of <i>trap door</i> spiders

Mygalomorphae is built from [morpho-web](https://github.com/jpegleg/morpho-web) (Actix-based) web API for collecting information on firfather.
