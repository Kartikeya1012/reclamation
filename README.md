# Reclamation

macOS storage cleanup tool. Moves junk files to a safe location (fully reversible).

## What It Does

Finds and moves junk files (`.tmp`, `.log`, `.DS_Store`, etc.) from a folder to `~/.reclamation/quarantine/`. Creates a manifest (JSON log) so you can restore everything.

**Quarantine** = files are moved, not deleted. You can restore them anytime.

## Build

```bash
cargo build --release
```

## Usage

```bash
# See what would be cleaned (read-only)
reclamation triage ~/Downloads

# Move auto-safe junk files
reclamation clean ~/Downloads

# Restore moved files
reclamation restore [manifest-id]
```

## Commands

- `triage <path>` - Analyze folder, show what would be cleaned
- `clean <path>` - Move auto-safe items to quarantine
- `restore [id]` - Restore files (uses latest manifest if no ID)

## What Gets Cleaned

**Auto-safe** (moved automatically):
- `.tmp`, `.temp`, `.log`, `.DS_Store`

**Never touched**:
- `.app` bundles, system files (`/System/`, `/Library/Frameworks/`)

**Needs review** (everything else - not moved)

Edit rules in `src/classify.rs`.

## What Exactly Is a Manifest?

A manifest is a JSON file that records what was moved during a cleanup. Each `clean` operation creates one.

It contains:
- **ID**: Timestamp identifying this cleanup run
- **Entries**: List of (original path, quarantine path) pairs

Example:
```json
{
  "id": "1704067200",
  "entries": [
    ["/Users/me/Downloads/file.tmp", "/Users/me/.reclamation/quarantine/file.tmp"]
  ]
}
```

The manifest enables `restore` to move files back to their original locations. Manifests are stored in `~/.reclamation/manifests/` and deleted after successful restore.

## Where Files Go

- Moved files: `~/.reclamation/quarantine/`
- Manifests: `~/.reclamation/manifests/` (JSON files)
