#!/usr/bin/env python3
"""Inventaire en lecture seule, Python standard + Git ; aucune estimation de productivité.

Lignes brutes, tests/commentaires inclus. L'activité suit la première parenté du HEAD,
sans compter une seconde fois les branches et leurs copies. Renommages comptés en retrait/ajout.
"""
from __future__ import annotations
import argparse
from collections import Counter
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parent.parent
ACTIVE = ("REPRISE.md", "README.md", "docs/00_INDEX.md", "notes/METHODE.md",
          "docs/FEUILLE-DE-ROUTE.md")


def git(*args: str) -> str:
    return subprocess.check_output(["git", "-C", str(ROOT), *args], encoding="utf-8")


def category(path: str) -> str:
    for prefix, label in (("code/water-core/src/", "coeur_src"),
                          ("code/water-harness/src/", "harnais_src"),
                          ("viewer/src/", "afficheur_src")):
        if path.startswith(prefix):
            return label
    if path.startswith("code/") and "/examples/" in path:
        return "exemples"
    return "markdown" if path.endswith(".md") else "autres"


def layer(path: str) -> str | None:
    """Repérage par noms, pas décompte de capacités ni jugement sur une modification."""
    if not path.startswith("code/water-core/src/") or not path.endswith(".rs"):
        return None
    name = PurePosixPath(path).stem
    if name.startswith("tests_") or name.endswith("_tests"):
        return None
    for label, pattern in (("B", r"^background"),
                           ("W", r"impact|wake|pressure|radial|modal|spectral|composition|prepared|mixed|wave_"),
                           ("delta", r"delta|shallow|dispersif|eponge|projection"),
                           ("V", r"hydro_geometry|hydro_snapshot|network|reseau|pipe|conduite")):
        if re.search(pattern, name):
            return label
    return None


def history(raw: str) -> list[dict]:
    commits = []
    for block in raw.split("\x1e"):
        lines = block.strip().splitlines()
        if not lines:
            continue
        commit, subject = lines[0].split("\t", 1)
        match = re.match(r"S(\d+)\s+P", subject)
        # Réinitialiser à CHAQUE commit, même non numéroté.
        changes = []
        for line in lines[1:]:
            parts = line.split("\t")
            if len(parts) == 3 and parts[0].isdigit() and parts[1].isdigit():
                changes.append((int(parts[0]), int(parts[1]), parts[2]))
        commits.append(dict(commit=commit, subject=subject,
                            session=int(match[1]) if match else None, changes=changes))
    return commits


def activity(commits: list[dict], since: int) -> dict:
    eras: dict[int, Counter] = {}
    for commit in commits:
        session = commit["session"]
        if session is None or session < since:
            continue
        counts = eras.setdefault(session // 10 * 10, Counter())
        for added, _removed, path in commit["changes"]:
            counts[category(path)] += added
    return {f"S{era}–S{era + 9}": dict(counts) for era, counts in sorted(eras.items())}


def missing_links(path: Path, contents: str) -> list[str]:
    """Chemins locaux Markdown seulement ; ancres et liens web non certifiés."""
    errors = []
    contents = re.sub(r"```.*?```", "", contents, flags=re.S)
    for target in re.findall(r"\]\(([^)]+)\)", contents):
        target = target.strip().strip("<>")
        if re.match(r"[a-zA-Z][\w+.-]*:", target) or target.startswith("#"):
            continue
        target = unquote(target.split("#", 1)[0])
        if target and not (path.parent / target).exists():
            errors.append(f"{path.relative_to(ROOT)} -> {target}")
    return errors


def inspect(since: int | None) -> dict:
    paths = sorted(filter(None, git("ls-files", "-z").split("\0")))
    commits = history(git("log", "--first-parent", "--no-renames", "--numstat",
                          "--format=%x1e%H%x09%s"))
    latest = max((c["session"] or 0 for c in commits), default=0)
    since = max(0, latest // 10 * 10 - 20) if since is None else since
    inventory: dict[str, Counter] = {}
    texts = {}
    for path in paths:
        if Path(path).suffix not in (".md", ".rs", ".wgsl", ".py", ".sh"):
            continue
        full = ROOT / path
        if not full.is_file():
            continue
        content = full.read_text(encoding="utf-8-sig")
        texts[path] = content
        stats = inventory.setdefault(category(path), Counter())
        stats["fichiers"] += 1
        stats["lignes_brutes"] += len(content.splitlines())
    layers = {}
    for name in ("B", "W", "delta", "V"):
        files = {p for p in texts if layer(p) == name}
        last = next((c for c in commits if any(p in files for _, _, p in c["changes"])), None)
        layers[name] = dict(fichiers_reperes=len(files),
                            derniere_modification=last["subject"] if last else None)
    questions = texts["docs/registres/QUESTIONS-OUVERTES.md"]
    active_questions = questions.split("## File active", 1)[1].split("## Traçabilité historique", 1)[0]
    errors = [e for p in ACTIVE for e in missing_links(ROOT / p, texts[p])]
    errors += missing_links(ROOT / "docs/registres/QUESTIONS-OUVERTES.md", active_questions)
    index = texts["docs/00_INDEX.md"]
    adrs = [p for p in paths if re.fullmatch(r"docs/adr/ADR-\d+[^/]*\.md", p)]
    errors += [f"ADR absent de l'index : {p}" for p in adrs if p.removeprefix("docs/") not in index]
    return dict(head=git("rev-parse", "--short", "HEAD").strip(),
                note="Fichiers suivis présents ; lignes brutes, tests/commentaires inclus. "
                     "Ajouts Git sans renommages ; ni temps, ni productivité, ni capacités. "
                     "Ancres Markdown non vérifiées.",
                modifications_locales=git("status", "--porcelain").splitlines(),
                inventaire=inventory,
                pages_actives={p: len(texts[p].splitlines()) for p in ACTIVE},
                file_active_lignes=len(active_questions.splitlines()),
                couches=layers, depuis_session=since,
                ajouts_par_ere=activity(commits, since), erreurs_navigation=errors)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--check", action="store_true", help="échouer si navigation active incomplète")
    parser.add_argument("--since-session", type=int)
    args = parser.parse_args()
    if args.since_session is not None and args.since_session < 0:
        parser.error("--since-session doit être positif ou nul")
    result = inspect(args.since_session)
    if args.json:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    else:
        print(f"HEAD {result['head']} — {result['note']}")
        for name, values in result["inventaire"].items():
            print(f"{name}: {values['fichiers']} fichiers, {values['lignes_brutes']} lignes")
        for path, count in result["pages_actives"].items():
            print(f"Lecture active {path}: {count} lignes")
        print(f"File active: {result['file_active_lignes']} lignes")
        for name, values in result["couches"].items():
            print(f"{name}: {values['fichiers_reperes']} fichiers repérés ; "
                  f"dernière modification: {values['derniere_modification']}")
        for era, values in result["ajouts_par_ere"].items():
            print(f"{era} (depuis S{result['depuis_session']}): {values}")
        print(f"Navigation active: {len(result['erreurs_navigation'])} erreur(s)")
        for error in result["erreurs_navigation"]:
            print(error)
    return int(args.check and bool(result["erreurs_navigation"]))


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    try:
        raise SystemExit(main())
    except (OSError, subprocess.CalledProcessError, UnicodeError) as error:
        print(f"Inventaire impossible : {error}", file=sys.stderr)
        raise SystemExit(2)
