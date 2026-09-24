#!/usr/bin/env python3
"""Inventaire en lecture seule, Python standard + Git ; aucune estimation de productivité.

Lignes brutes, tests/commentaires inclus. L'activité suit la première parenté du HEAD,
sans compter une seconde fois les branches et leurs copies. Renommages comptés en retrait/ajout.
"""
from __future__ import annotations
import argparse
from collections import Counter
from datetime import datetime, timezone
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
from urllib.parse import unquote

import dependances_liste  # S352 : le registre des dépendances suit la liste (ADR-190 D3)

ROOT = Path(__file__).resolve().parent.parent
ACTIVE = ("REPRISE.md", "README.md", "docs/00_INDEX.md", "notes/METHODE.md",
          "docs/FEUILLE-DE-ROUTE.md", "notes/EN-COURS.md")


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


# S294 (BILAN-GLOBAL-S293 M3) : les documents d'état redevenaient des journaux. Plafonds en mots,
# indépendants de la largeur des lignes : l'histoire va au journal et aux preuves.
QUEUE_ROW_WORDS = 90
MILESTONE_WORDS = 450


def oversized(active_questions: str, roadmap: str) -> list[str]:
    """Lignes de la file et sections de jalon au-delà de leur plafond de mots."""
    found = []
    for line in active_questions.splitlines():
        if line.startswith("| **"):
            words = len(line.replace("|", " ").split())
            if words > QUEUE_ROW_WORDS:
                title = line.split("**")[1]
                found.append(f"file active, « {title} » : {words} mots > {QUEUE_ROW_WORDS}")
    milestones = roadmap.split("## 2. Les jalons", 1)[-1].split("\n## ", 1)[0]
    for section in re.split(r"\n(?=### )", milestones):
        if section.startswith("### "):
            words = len(section.split())
            if words > MILESTONE_WORDS:
                title = section.splitlines()[0].removeprefix("### ")
                found.append(f"feuille de route, « {title} » : {words} mots > {MILESTONE_WORDS}")
    return found


def heartbeat(reprise: str, now: datetime | None = None) -> list[str]:
    """Le battement du jeton ne peut pas être **dans le futur** — contrôle ajouté en S309.

    **Pourquoi ce contrôle existe.** L237 demande de lire l'horloge dans un appel séparé et d'en
    reporter la valeur à chaque commit d'étape. S308 ne l'a lue qu'une fois et a **extrapolé** les
    battements suivants : le dernier valait 1 h 30 de plus que l'heure réelle. Un battement dans le
    futur est le pire des cas — AGENTS.md dit qu'un jeton `occupé` de moins de deux heures
    interdit la reprise, donc un horodatage avancé **bloque** la session suivante, et il le fait
    silencieusement parce que rien ne le relit.

    Une consigne d'exactitude sans contrôle n'est pas tenue : c'est **L349** appliquée à l'heure.
    Deux minutes de tolérance pour l'écart de lecture entre le `date` et l'écriture du fichier.
    """
    marge = 120.0
    m = re.search(r"^Battement\s*:\s*(\d{4}-\d{2}-\d{2} \d{2}:\d{2} [+-]\d{2}:?\d{2})\s*$",
                  reprise, re.M)
    if not m:
        # Un battement mal formé ne correspond pas au motif : il tombe ici, et c'est voulu — une
        # date illisible est aussi grave qu'une date absente, et un seul message suffit.
        return ["REPRISE.md : aucune ligne « Battement » lisible"]
    stamp = datetime.strptime(m.group(1), "%Y-%m-%d %H:%M %z")
    now = now or datetime.now(timezone.utc)
    avance = (stamp - now).total_seconds()
    if avance > marge:
        return [f"REPRISE.md : battement {m.group(1)} dans le futur de {avance / 60:.0f} min — "
                "L237, l'horloge se lit, elle ne s'extrapole pas"]
    return []


# S321 (ADR-187 D3, D5, D6) : les consignes qu'on oubliait deviennent des contrôles. Chacun porte le
# contre-exemple réel qui l'a fait naître ; ceux qui visent des écrits neufs ne jugent que S321 et après.
EN_COURS_LINES = 300
JOURNAL_ENTRY_LINES = 20
FROM_SESSION = 321
# Texte UTF-8 relu comme ANSI puis réécrit : une lettre accentuée devient deux caractères, un A
# tilde ou un A circonflexe suivi d'un octet de continuation. Écrit en échappements : ce fichier est
# lui-même contrôlé.
MOJIBAKE = re.compile("\u00c3[\u0080-\u00bf]|\u00e2\u20ac[\u0080-\u00bf\u0152\u0153\u0160\u0161"
                      "\u0178\u017d\u017e\u0192\u02c6\u02dc\u2013\u2014\u2018-\u201e\u2020-\u2022"
                      "\u2026\u2030\u2039\u203a\u2122]|\u00c2[\u00a0-\u00bf]|\ufffd")
PRODUCED = re.compile(r"(^|/)(__pycache__|target)/|\.py[co]$")


def en_cours(text: str) -> list[str]:
    """`EN-COURS` ne porte que la session en cours (ADR-187 D3).

    Contre-exemple réel : 1 686 lignes à la fin de S320, dont 1 560 de « Archive — notes de S30x »
    placées **avant** les notes de la session courante ; la reprise « de cinq minutes » lisait 115 Ko.
    """
    found = []
    lines = text.splitlines()
    if len(lines) > EN_COURS_LINES:
        found.append(f"EN-COURS : {len(lines)} lignes > {EN_COURS_LINES} — verser les notes "
                     "closes à leur preuve ou au journal")
    for line in lines:
        if re.match(r"#+ .*\barchive", line, re.I):
            found.append(f"EN-COURS : section d'archive « {line.lstrip('# ')} » — Git garde les "
                         "sessions closes")
    return found


def encoding(texts: dict[str, str]) -> list[str]:
    """Un fichier suivi relu en ANSI puis réécrit porte des séquences que le français n'a jamais.

    Contre-exemple réel : S301, `Get-Content -Raw | Set-Content` de Windows PowerShell 5.1 a
    ré-encodé `REPRISE.md` (commit `fff03d5`), restauré depuis `7ebeeb5`. Une ligne par fichier.
    """
    found = []
    for path, content in texts.items():
        for number, line in enumerate(content.splitlines(), 1):
            match = MOJIBAKE.search(line)
            if match:
                extrait = line[max(0, match.start() - 20):match.end() + 20]
                found.append(f"{path}:{number} : encodage abîmé près de « {extrait} »")
                break
    return found


def produced(paths: list[str]) -> list[str]:
    """Un fichier produit ne se versionne pas : un essai le réécrit, et la reprise à chaud prend le
    diff pour celui de l'étape interrompue. Contre-exemple réel : `outils/__pycache__/*.pyc`,
    versionnés jusqu'en S321."""
    return [f"fichier produit versionné : {p}" for p in paths if PRODUCED.search(p)]


def checklist(text: str) -> list[str]:
    """Le tableau « Décompte » de la liste égale le compte de ses points, section par section.

    Contre-exemple réel : S321 trouve 3 / 51 / 66 affichés pour 3 / 53 / 64 comptés — 4.8 (S316) et
    4.12 (S320) avaient changé de case sans que le tableau suive. REPRISE §6 le demandait (L349).
    """
    body, _, table = text.partition("## Décompte")
    counted: dict[int, Counter] = {}
    for item in re.split(r"\n(?=- \[[ x]\] \*\*\d+\.\d+)", body):
        head = re.match(r"- \[([ x])\] \*\*(\d+)\.\d+", item)
        if not head:
            continue
        state = re.search(r"\*(partiel|absent)\*", item[:400])
        key = "validés" if head[1] == "x" else (state[1] + "s" if state else "?")
        section = counted.setdefault(int(head[2]), Counter())
        section[key] += 1
        section["points"] += 1
    found = [f"liste : point de la section {s} sans état lisible" for s, c in counted.items() if c["?"]]
    columns = ("points", "validés", "partiels", "absents")
    total = sum(counted.values(), Counter())
    for row in table.splitlines():
        cells = [c.strip().strip("*") for c in row.strip().strip("|").split("|")]
        if len(cells) != 5 or not all(c.isdigit() for c in cells[1:]):
            continue
        shown = tuple(int(c) for c in cells[1:])
        name = cells[0]
        number = re.match(r"(\d+)\.", name)
        source = counted.get(int(number[1]), Counter()) if number else total if name == "total" else None
        if source is None:
            continue
        real = tuple(source[c] for c in columns)
        if shown != real:
            found.append(f"liste, décompte « {name} » : affiché {shown}, compté {real} "
                         "(points, validés, partiels, absents)")
    return found


def reproduce(first_session: dict[str, int], texts: dict[str, str]) -> list[str]:
    """Toute preuve ouverte à partir de S321 commence par « Reproduire » (ADR-187 D6)."""
    return [f"{path} : preuve ouverte en S{session} sans section « Reproduire »"
            for path, session in sorted(first_session.items())
            if path.startswith("docs/validation/") and path.endswith(".md")
            and session >= FROM_SESSION
            and not re.search(r"^(#+ .*|\*\*)Reproduire", texts.get(path, ""), re.M)]


def journal(text: str) -> list[str]:
    """Une entrée de journal tient en vingt lignes de texte, titre exclu, à partir de S321."""
    found = []
    for block in re.split(r"\n(?=## S\d+)", text):
        head = re.match(r"## S(\d+)", block)
        if not head or int(head[1]) < FROM_SESSION:
            continue
        lines = [line for line in block.splitlines()[1:] if line.strip()]
        if len(lines) > JOURNAL_ENTRY_LINES:
            found.append(f"journal, S{head[1]} : {len(lines)} lignes de texte > {JOURNAL_ENTRY_LINES}")
    return found


def first_sessions(commits: list[dict], paths: list[str], latest: int) -> dict[str, int]:
    """Session du premier commit qui touche chaque fichier ; un fichier pas encore committé est de
    la session en cours."""
    first: dict[str, int] = {}
    for commit in reversed(commits):
        for _added, _removed, path in commit["changes"]:
            first.setdefault(path, commit["session"] or 0)
    return {path: first.get(path, latest) for path in paths}


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
    sizes = oversized(active_questions, texts["docs/FEUILLE-DE-ROUTE.md"])
    clock = heartbeat(texts["REPRISE.md"])
    controls = (en_cours(texts["notes/EN-COURS.md"]) + encoding(texts) + produced(paths)
                + checklist(texts["docs/LISTE-PROJET-FINI.md"])
                + dependances_liste.ecarts(texts["docs/LISTE-PROJET-FINI.md"],
                                           texts.get(dependances_liste.REGISTRE, ""))
                + reproduce(first_sessions(commits, paths, latest), texts)
                + journal(texts["notes/JOURNAL.md"]))
    return dict(head=git("rev-parse", "--short", "HEAD").strip(),
                note="Fichiers suivis présents ; lignes brutes, tests/commentaires inclus. "
                     "Ajouts Git sans renommages ; ni temps, ni productivité, ni capacités. "
                     "Ancres Markdown non vérifiées.",
                modifications_locales=git("status", "--porcelain").splitlines(),
                inventaire=inventory,
                pages_actives={p: len(texts[p].splitlines()) for p in ACTIVE},
                file_active_lignes=len(active_questions.splitlines()),
                couches=layers, depuis_session=since,
                ajouts_par_ere=activity(commits, since), erreurs_navigation=errors,
                plafonds_depasses=sizes, battement=clock, controles=controls)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--check", action="store_true",
                        help="échouer si navigation active incomplète, plafond dépassé, battement "
                             "du jeton dans le futur, ou contrôle de S321 en défaut")
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
        print(f"Plafonds des documents d'état : {len(result['plafonds_depasses'])} dépassement(s)")
        for excess in result["plafonds_depasses"]:
            print(excess)
        print(f"Battement du jeton : {len(result['battement'])} anomalie(s)")
        for anomaly in result["battement"]:
            print(anomaly)
        print(f"Contrôles (EN-COURS, encodage, fichiers produits, liste, dépendances, preuves, journal) : "
              f"{len(result['controles'])} anomalie(s)")
        for anomaly in result["controles"]:
            print(anomaly)
    return int(args.check and bool(result["erreurs_navigation"] or result["plafonds_depasses"]
                                   or result["battement"] or result["controles"]))


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    try:
        raise SystemExit(main())
    except (OSError, subprocess.CalledProcessError, UnicodeError) as error:
        print(f"Inventaire impossible : {error}", file=sys.stderr)
        raise SystemExit(2)
