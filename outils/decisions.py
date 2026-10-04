# -*- coding: utf-8 -*-
"""Les décisions en vigueur, ADR par ADR — S480 (BOUSSOLE : ne pas travailler dans le flou).

Lit l'en-tête de chaque `docs/adr/ADR-*.md` (le titre et ce qui précède la première section) ; écrit
`docs/registres/DECISIONS-EN-VIGUEUR.md` : le statut lu, la session, les ADR que l'en-tête nomme, et ceux dont l'en-tête nomme
celui-ci (qui le remplace, le précise ou le rétracte en partie). Rien ne s'y écrit à la main ; un ADR ne se réécrit jamais, donc
le statut d'un ADR ancien se lit aussi dans la colonne « nommé par » — c'est là que vivent ses suites.

Le statut se lit par mots, dans l'ordre : *rétract* → rétractée en partie ; *remplacée par*, *caduque*, *abrogée* → remplacée ;
*acté* → actée ; *proposé* → proposée ; sinon « ? ». Une note corrective en tête (« > **Note S39 — D2 et D4 sont RÉTRACTÉS** »)
compte : elle fait partie de l'en-tête.

    python outils/decisions.py              # le résumé
    python outils/decisions.py --ecrire     # réécrit le registre
    python outils/decisions.py --check      # échoue si le registre ne suit plus les ADR

`etat_projet.py --check` appelle `ecarts`.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRE = "docs/registres/DECISIONS-EN-VIGUEUR.md"
LIGNES_EN_TETE = 30


def en_tete(texte: str) -> tuple[str, str]:
    """(titre, en-tête) : le titre sans `# ADR-NNN — `, l'en-tête jusqu'à la première section ou au premier filet."""
    lignes = texte.lstrip("﻿\n").splitlines()
    titre = re.sub(r"^# ADR-\d+\s*[—–-]\s*", "", lignes[0]).strip() if lignes else ""
    corps = []
    for l in lignes[1:LIGNES_EN_TETE]:
        if l.startswith("## ") or l.strip() == "---":
            break
        corps.append(l)
    return titre, " ".join(" ".join(corps).split())


def statut(tete: str) -> str:
    t = tete.replace("*", "").lower()
    if "rétract" in t:
        return "rétractée en partie"
    if re.search(r"remplacée? par|caduque|abrogée", t):
        return "remplacée"
    if "acté" in t:
        return "actée"
    if "proposé" in t:
        return "proposée"
    return "?"


def lire(adrs: dict) -> list:
    """[(n, chemin, titre, statut, session, nomme)] trié par numéro ; `adrs` : chemin → texte."""
    out = []
    for chemin, texte in adrs.items():
        n = int(re.search(r"ADR-(\d+)", chemin).group(1))
        titre, tete = en_tete(texte)
        s = re.search(r"\bS(\d+)\b", tete)
        nomme = sorted({int(x) for x in re.findall(r"ADR-(\d+)", tete)} - {n})
        out.append((n, chemin, titre, statut(tete), f"S{s.group(1)}" if s else "", nomme))
    return sorted(out)


def rendre(adrs: dict) -> str:
    rangs = lire(adrs)
    fichier = {n: Path(c).name for n, c, *_ in rangs}
    nomme_par = {n: [] for n in fichier}
    for n, _, _, _, _, nomme in rangs:
        for m in nomme:
            if m in nomme_par and m < n:
                nomme_par[m].append(n)
    compte = {}
    for r in rangs:
        compte[r[3]] = compte.get(r[3], 0) + 1

    def lien(m: int) -> str:
        return f"[{m:03d}](../adr/{fichier[m]})" if m in fichier else f"{m:03d}"

    lignes = [
        "# Décisions en vigueur — ADR par ADR",
        "",
        "*Généré par `python outils/decisions.py --ecrire` (S480) — ne pas modifier à la main.* Lu dans l'en-tête de chaque ADR (le titre",
        "et ce qui précède sa première section). **Un ADR ne se réécrit jamais** : ses suites sont dans les ADR plus récents qui le",
        "nomment en tête — colonne « nommé par ». Le résumé des décisions qui gouvernent le travail aujourd'hui est dans la",
        "[boussole](../../BOUSSOLE.md) ; ce registre est le détail, pour vérifier qu'une décision n'a pas été remplacée.",
        "",
        f"**{len(rangs)} ADR** — " + ", ".join(f"{k} : {v}" for k, v in sorted(compte.items(), key=lambda kv: -kv[1])) + ".",
        "Le statut est lu par mots (voir l'outil) ; « proposée » vient souvent des premières sessions, avant que l'usage n'écrive",
        "« actée » : une proposée nommée par des ADR actés est en pratique appliquée.",
        "",
        "| ADR | titre | statut lu | session | nomme en tête | nommé par (plus récents) |",
        "|---|---|---|---|---|---|",
    ]
    for n, _, titre, st, s, nomme in rangs:
        titre = titre.replace("|", "\\|")
        lignes.append(f"| {lien(n)} | {titre} | {st} | {s} | {' '.join(f'{m:03d}' for m in nomme)} | "
                      f"{' '.join(f'{m:03d}' for m in nomme_par[n])} |")
    lignes.append("")
    return "\n".join(lignes)


def lire_adrs() -> dict:
    return {f"docs/adr/{p.name}": p.read_text(encoding="utf-8-sig") for p in sorted((ROOT / "docs/adr").glob("ADR-*.md"))}


def ecarts(adrs: dict, registre: str) -> list:
    if not registre:
        return [f"{REGISTRE} absent : python outils/decisions.py --ecrire"]
    if rendre(adrs) != registre:
        return [f"{REGISTRE} ne suit plus les ADR : python outils/decisions.py --ecrire"]
    return []


def main(argv) -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    adrs = lire_adrs()
    chemin = ROOT / REGISTRE
    existant = chemin.read_text(encoding="utf-8") if chemin.exists() else ""
    if "--check" in argv:
        e = ecarts(adrs, existant)
        for x in e:
            print(x)
        return 1 if e else 0
    texte = rendre(adrs)
    if "--ecrire" in argv:
        chemin.write_text(texte, encoding="utf-8", newline="\n")
    compte = {}
    for r in lire(adrs):
        compte[r[3]] = compte.get(r[3], 0) + 1
    print("DECISIONS " + " ".join(f"{k.replace(' ', '_')}={v}" for k, v in sorted(compte.items())))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
