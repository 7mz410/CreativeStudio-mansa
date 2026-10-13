#!/usr/bin/env python3
"""Apply the Creative Studio branding on top of upstream Craft Launcher (ArtCraft Launcher).

Only user-visible text is rebranded: crate names, bundle ids, asset file names, registry keys and
the ArtCraft feed's own labels keep their upstream names, so `git merge upstream/main` stays easy.
Idempotent: after merging upstream, resolve conflicts in favour of upstream and run it again.

    python3 studio/rebrand.py

Discord is removed from the launcher (ArtCraft's community), so its labels and rows go too.
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# Visible-text tokens, applied to string literals and to the two text columns of the catalogs.
TOKENS = [
    ('ArtCraft Launcher', 'Mansa'),
    ('Craft App Suite', 'Creative Studio suite'),
    ('getartcraft.com', 'studio.creative.ps'),
    ('ArtCraft', 'Creative Studio'),
]
# Labels of the removed Discord controls; their catalog rows go with them.
DISCORD = ('Discord', 'Join the Discord', 'Join the ArtCraft Discord', 'Join the ArtCraft community on Discord')
SOURCES = ['crates/ui-egui/src', 'crates/ui-egui/tests', 'apps/artcraft-launcher/src', 'apps/artcraft-launcher-web/src']
# Exact edits outside the literal rule: crates whose literals hold registry keys or file names.
RULES = [
    ('crates/native/src/backend.rs', 'ArtCraft Launcher is restarting', 'Mansa is restarting'),
    ('crates/native/src/backend.rs', 'this copy of ArtCraft Launcher', 'this copy of Mansa'),
    ('crates/native/src/backend.rs', "ArtCraft Launcher didn't restart", "Mansa didn't restart"),
    ('crates/native/src/self_update.rs', 'this copy of ArtCraft Launcher', 'this copy of Mansa'),
    ('crates/native/src/self_update.rs', 'ArtCraft Launcher {} has no', 'Mansa {} has no'),
    ('crates/core/src/status.rs', '"ArtCraft Launcher"', '"Mansa"'),
    ('crates/core/src/catalog.rs', '    "ArtCraft Launcher",\n    "Installs and updates the Craft App Suite",', '    "Mansa",\n    "Installs and updates the Creative Studio suite",'),
]
STRING = re.compile(r'"(?:[^"\\\n]|\\.)*"')
# Literals that name the feed or the registry, not the product.
KEEP = ('ArtCraft API', 'Learning Machines', 'storyteller')


def token(text: str) -> str:
    for a, b in TOKENS:
        text = text.replace(a, b)
    return text


def rename_literal(lit: str) -> str:
    if any(k in lit for k in KEEP):
        return lit
    return token(lit)


def apply_sources() -> list:
    changed = []
    for d in SOURCES:
        for p in sorted((ROOT / d).rglob('*.rs')):
            s = p.read_text(encoding='utf-8')
            o = STRING.sub(lambda m: rename_literal(m.group(0)), s)
            if o != s:
                p.write_text(o, encoding='utf-8')
                changed.append(p.relative_to(ROOT).as_posix())
    html = ROOT / 'apps/artcraft-launcher-web/index.html'
    s = html.read_text(encoding='utf-8')
    o = token(s)
    if o != s:
        html.write_text(o, encoding='utf-8')
        changed.append(html.relative_to(ROOT).as_posix())
    return changed


def apply_rules() -> list:
    changed = []
    for rel, a, b in RULES:
        p = ROOT / rel
        s = p.read_text(encoding='utf-8')
        o = s.replace(a, b)
        if o != s:
            p.write_text(o, encoding='utf-8')
            changed.append(rel)
    return changed


def apply_catalogs() -> list:
    changed = []
    for p in sorted((ROOT / 'crates/ui-egui/src/i18n').glob('*.tsv')):
        s = p.read_text(encoding='utf-8')
        rows = []
        for line in s.split('\n'):
            cells = line.split('\t')
            if len(cells) >= 3 and not line.startswith('#') and cells[1] in DISCORD:
                continue
            if len(cells) >= 3 and not line.startswith('#'):
                line = '\t'.join([cells[0], *(token(c) for c in cells[1:])])
            elif not line.startswith('#'):
                line = token(line) if line.count('\t') < 2 else line
            rows.append(line)
        o = '\n'.join(rows)
        if o != s:
            p.write_text(o, encoding='utf-8')
            changed.append(p.relative_to(ROOT).as_posix())
    return changed


def main():
    changed = apply_sources() + apply_rules() + apply_catalogs()
    for c in sorted(set(changed)):
        print('rebranded', c)


if __name__ == '__main__':
    main()
