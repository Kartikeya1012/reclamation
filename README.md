# Reclamation

macOS storage cleanup tool. Moves junk files to a safe location (fully reversible).

## What It Does

Finds and moves junk files (`.tmp`, `.log`, `.DS_Store`, etc.) from a folder to `~/.reclamation/quarantine/`. Creates a manifest (JSON log) so you can restore everything.

**Quarantine** = files are moved, not deleted. You can restore them anytime.

## Installation

```bash
# Build binary
cargo build --release
# Binary will be at: ./target/release/reclamation

# Or install globally
cargo install --path .
# Then use: reclamation <command>
```

## Usage

```bash
# If installed globally
reclamation triage ~/Downloads

# If using built binary
./target/release/reclamation triage ~/Downloads

# Or use cargo run (development)
cargo run -- triage ~/Downloads
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

## Storage Locations

All data is stored in `~/.reclamation/`:

- **Quarantined files**: `~/.reclamation/quarantine/`
  - Files moved here during `clean` operations
  - Original filenames preserved
  
- **Manifests**: `~/.reclamation/manifests/`
  - JSON files named `{timestamp}.json`
  - One manifest per `clean` operation
  - Contains mapping of original → quarantine paths
  - Deleted automatically after successful `restore`

## Common Questions

**Q: Is it safe?**  
A: Yes. Files are moved, not deleted. Everything is reversible via manifests.

**Q: What if I lose a manifest?**  
A: Manifests are JSON files in `~/.reclamation/manifests/`. You can manually restore by moving files from `quarantine/` back to their original locations.

**Q: Can I customize what gets cleaned?**  
A: Yes. Edit the rules in `src/classify.rs` - the `AUTO_SAFE` and `DO_NOT_TOUCH` arrays.

**Q: Does it work recursively?**  
A: Not yet (M1). It only scans direct children of the folder you specify.
