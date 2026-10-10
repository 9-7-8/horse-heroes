# Distribution: installer and updater

How the game reaches a player's machine and stays current. This is an
implementation guide the owner handed over, turned into the rules to follow
when the work is done. **None of it has been run**: its scripts and commands
are templates, written with placeholder names, and are to be checked against
the real tools before use.

The recommended first design:

> An installer packages the game. A small separate launcher checks GitHub
> Releases, downloads a verified newer installer, exits, and lets that
> installer upgrade the existing installation. Shortcuts start the launcher,
> not the game.

## Principles

### What the player experiences

- **First install:** download one setup file from the releases page, run it,
  install for the current user, start from the Start menu or a desktop
  shortcut. No Rust, Cargo, Bevy or Visual Studio needed.
- **Every launch:** the launcher checks for an update with a short timeout.
  None, or no network: the game starts. An update: it is downloaded and
  verified, the launcher waits until the game is closed, and the upgrade
  runs.
- The player always gets progress, an explanation, and a **"play the current
  version"** choice.
- **An update never interrupts a running expedition.**

### Why a separate launcher

A running Windows program cannot reliably replace itself. Keep the update
check out of the game, and leave replacing files to an installer process
that runs after both launcher and game have exited.

Three deliverables: the game, the launcher, and a setup program containing
both plus the assets.

No background service. Updating while the game is closed is enough.

### Target

Windows 11 on x64 to begin with. Do not call it ARM support: an installer
that permits x64-compatible machines says nothing about whether the game or
its graphics run on ARM hardware. Test before promising. If earlier Windows
versions are to be blocked, that needs its own tested check.

### Building for release

- A release build for the MSVC target, with the lock file respected.
- **Never ship with Bevy's development dynamic-linking feature.**
- A staging directory holding exactly what is installed: the game, the
  launcher, a version file, the assets, the third-party notices.
- **Do not assume the executable alone is enough.** Copy every asset the
  game refers to, and any runtime library the real dependencies need.
- **Load assets relative to the executable**, or have the launcher set the
  working directory. Test from a shortcut and from an unrelated directory,
  not only from Cargo.

### Where things live

Installed files under the user's local programs folder. Everything the game
writes goes elsewhere, under the user's local application data: saves,
settings, logs, downloaded updates.

**Never put a save inside the assets or beside files the installer owns.**

### The installer

- **Per user, no administrator rights.** That also gives a per-user
  uninstall entry.
- **The application ID never changes between releases**, even if the visible
  name does. It is what identifies the existing installation. A separate ID
  only for a truly separate installation, such as a beta beside a stable.
- Do not rely on the installer forcing the game closed. The launcher makes
  sure the game has exited first.

### Releases

- A **public** repository, so the launcher needs no credentials. **Never
  embed an access token in a launcher.**
- Each release: a version tag, notes, published as stable (not draft or
  prerelease), the setup file under one exact name, and signed update
  metadata with its signature.
- Download the uploaded setup asset, never the automatic source archives.
- **Three-part semantic versions, compared by a real version parser.**
  "0.10.0" is newer than "0.9.0" and string comparison gets that wrong.

### The update check

- Ask the releases service for the latest release, identify the launcher in
  the request, and compare its tag with the installed version yourself. "The
  latest release" is the service's choice, not a version sort.
- **Match the asset by exact name.** Never "the first .exe" or a substring.
- Unauthenticated requests are rate-limited. Check at launch, with caching
  and back-off, and offer a manual "check for updates". Respect a rate-limit
  response.
- **On any failure** (offline, timeout, missing asset, bad response, unknown
  metadata, rate limit) show a message that is not fatal and let the
  existing game run. **Never delete the current game because the update
  service is unavailable.**

### Verify before executing

> An updater downloads executable code. Verification is a release
> requirement, not optional polish.

- A hash from the release service detects corruption. It does not prove who
  published the file if the account is compromised.
- So: **sign a small update manifest** with a release key; embed the public
  key in the launcher; verify the signature over the exact bytes downloaded
  before trusting anything in the manifest; then hash the installer and
  compare with the signed manifest.
- Check product, channel, platform, schema and version. **Refuse an
  installer that is not newer**, unless the player has deliberately asked
  for a repair. Remember the highest version accepted, to resist rollback.
- **The private key is never in the repository and never in the launcher.**
  Plan how to rotate the public key before it is needed.
- Sign the game, launcher and installer for Windows as well. That does not
  make warnings vanish at once: reputation accumulates. **Never tell players
  to turn off Windows protection.**

### Downloading and installing

    idle -> checking -> update available -> downloading -> verifying
      -> waiting for the game to exit -> starting the installer -> launcher exits

- HTTPS, certificate checks on, bounded redirects, timeouts, a size limit.
- Download to a uniquely named partial file; rename only after it verifies.
  **Never execute a partial or mismatched file.**
- A lock so two launchers cannot update at once. The launcher tracks the
  game process it started; do not find the game by matching a file name in
  the process list.
- Start the installer through a process call with separate arguments, never
  an unescaped shell command. **Never run anything taken from release
  notes.**
- The launcher exits so its own file can be replaced.
- Simplest first: open the normal installer wizard. Later, a silent install
  once the player has turned automatic updates on, and no automatic relaunch
  from a silent install, so there can be no update loop.

### Versions and paths

- Find the installation from where the launcher itself is. Never assume the
  default folder. Test custom paths, spaces and non-ASCII characters.
- The installed version is read from a file shipped with each release, and
  **is never advanced merely because an installer was downloaded**.
- **One source for the version**, from which the build, the installer, the
  version file, the tag and the manifest all take it.
- Installing over the top is simple and not perfectly transactional. Keep a
  repair installer and allow a manual reinstall.

### Saves and old files

- Save before updating. **Save files have their own version**, separate from
  the game's.
- Back up before a migration and replace the save only after it succeeds.
  **The installer never rewrites a save.**
- A rolled-back game may not read a migrated save. State the policy and keep
  the pre-migration backup.
- An install does not remove files that an earlier version shipped and a
  later one dropped. List known obsolete files for removal. **Never delete
  the install folder or user data wholesale during an update.**
- Uninstalling removes the program and **keeps saves by default**. Deleting
  personal data is a separate, clearly labelled choice.

### Publishing, in order

Freeze the version; build; stage; test the staged folder on a clean machine;
sign game and launcher; build the installer; sign the installer with a
timestamp; **hash the final signed installer, because signing changes the
bytes**; generate and sign the manifest; upload everything to a draft
release; check names, platform, versions and downloads; publish; test the
upgrade from the previous release through the real public feed.

Automation can do the build and upload. Signing needs protected secrets, and
the publishing workflow needs protecting.

### Test on a clean machine

With no developer tools installed: a first install as a standard user; the
shortcuts; assets and audio; graphics on supported hardware; launch with no
network; a slow or broken download; a corrupted installer; a bad manifest
signature; a missing release and a rate-limit response; two launchers; an
update arriving while the game runs; an upgrade that changes both launcher
and game; a custom install path; saves and settings preserved, with a
migration backup; no update loop; uninstall leaving saves; a manual repair
after an interrupted install.

### Smallest path

1. The release build and its assets run on a clean Windows 11.
2. The installer installs, launches, upgrades and uninstalls.
3. The launcher starts the installed game from the right directory.
4. The launcher checks a public release and compares versions.
5. The launcher downloads and verifies, then runs the normal wizard.
6. Reliable exit coordination and silent updates.
7. A temporary helper for automatic relaunch, only if that is needed.

Every update is a complete installer. **Delta patches, background services
and hot-reloading are not needed for this game.**

## For Horse Heroes

- The guide was written with a placeholder name and no repository. The game
  is **Horse Heroes** and its public repository already exists, which meets
  the "public, no credentials" requirement. The application ID, publisher
  name and file names are still to be chosen, and the ID is the one that can
  never change afterwards.
- **The target is Windows 11 and the machine it is built on runs Windows
  10.** `CLAUDE.md` says "Windows 10 desktop". Either Windows 10 is supported
  too, or the clean-machine testing needs a Windows 11 machine. That is the
  owner's call.
- **The game is GPL.** An installer must ship the licence and say where the
  source is; the third-party notices in the staging folder are part of the
  same duty. A public repository with tagged releases makes "where the
  source is" easy to answer.
- **Most of the asset risk is absent today**: the game's data is compiled
  into the executable. That changes the moment models, sounds or fonts are
  loaded from disk, which the loot shower's audio will be among the first to
  do.
- **There are no saves yet**, so the rule about where they live costs
  nothing to follow from the start: the first save should already be in the
  user's local application data, versioned, and backed up before migration.
- The "never interrupt a running expedition" rule fits the game's own: an
  expedition is resolved in order and committed, so the natural moment to
  update is in town or at launch.
- Signing certificates cost money and a signing key needs keeping safe.
  Neither is a technical decision.
- This is late work. Nothing here makes a drop feel better (pillar 1). But
  the three cheap habits are worth having from now: one source for the
  version, user data kept out of the install folder, and assets found
  relative to the executable.
- `procedures/INDEX.txt` already lists "release-a-build" as a procedure not
  yet written. The publishing order above is its draft.

## Source

An implementation guide for a Windows installer and a release-based updater
for a Bevy game, received from the owner 2026-10-09. It is the document her
design summary referred to as coming before it.
