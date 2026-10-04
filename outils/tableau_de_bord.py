# -*- coding: utf-8 -*-
"""Le tableau de bord vers 100 % — S480 (ADR-218 : la liste validée à 100 %).

Lit la liste du projet fini (l'état de chaque point) et le plan de complétion (la campagne de chaque point) ; écrit
`docs/registres/TABLEAU-DE-BORD.md` : le décompte, chaque campagne avec ses points et leur état, l'historique du décompte (une ligne
par session qui l'a demandée). Rien ne s'y écrit à la main.

    python outils/tableau_de_bord.py                     # le résumé
    python outils/tableau_de_bord.py --ecrire [--session S480]   # réécrit le tableau ; avec --session, une ligne d'historique
    python outils/tableau_de_bord.py --check             # échoue si le tableau ne suit plus la liste ou le plan

`etat_projet.py --check` appelle `ecarts`.
"""
import re
import sys
from datetime import date
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LISTE = "docs/LISTE-PROJET-FINI.md"
PLAN = "docs/registres/PLAN-COMPLETION-S475.md"
TABLEAU = "docs/registres/TABLEAU-DE-BORD.md"
HORS = {"5.11"}
HIST_DEBUT, HIST_FIN = "<!-- historique -->", "<!-- fin de l'historique -->"


def etats(liste: str) -> dict:
    """Le point → validé, partiel ou absent (5.11 : hors)."""
    out = {}
    for bloc in re.split(r"\n(?=- \[[ x]\] \*\*)", liste):
        m = re.match(r"- \[([ x])\] \*\*(\d+\.\d+) ", bloc)
        if not m:
            continue
        texte = " ".join(bloc.split())
        if m.group(2) in HORS:
            out[m.group(2)] = "hors"
        elif m.group(1) == "x":
            out[m.group(2)] = "validé"
        elif "*absent*" in texte[:400]:
            out[m.group(2)] = "absent"
        else:
            out[m.group(2)] = "partiel"
    return out


def titres(liste: str) -> dict:
    return {m.group(1): m.group(2).strip() for m in re.finditer(r"- \[[ x]\] \*\*(\d+\.\d+) ([^*]+)\*\*", liste)}


def campagnes(plan: str) -> list:
    """[(K, intitulé, [points])] dans l'ordre du plan."""
    sec = plan[plan.index("## 2."):plan.index("## 3.")]
    out = []
    for l in sec.splitlines():
        m = re.match(r"\| \*\*(K\d+)\*\* \| \*\*([^*]+)\*\*[^|]*\| ([^|]*) \|", l)
        if not m:
            continue
        pts = []
        for tok in re.findall(r"\d+\.\d+(?:–\d+\.\d+)?", m.group(3)):
            if "–" in tok:
                a, b = tok.split("–")
                s0, i0 = a.split(".")
                _, i1 = b.split(".")
                pts += [f"{s0}.{i}" for i in range(int(i0), int(i1) + 1)]
            else:
                pts.append(tok)
        out.append((m.group(1), m.group(2).strip(), pts))
    return out


def historique(existant: str) -> str:
    if HIST_DEBUT in existant and HIST_FIN in existant:
        return existant.split(HIST_DEBUT, 1)[1].split(HIST_FIN, 1)[0].strip("\n")
    return "| session | date | validés | partiels | absents | périmètre |\n|---|---|---:|---:|---:|---:|"


def rendre(liste: str, plan: str, existant: str, session: str | None = None) -> str:
    e = etats(liste)
    t = titres(liste)
    camp = campagnes(plan)
    perimetre = [p for p in e if e[p] != "hors"]
    n = {k: sum(1 for p in perimetre if e[p] == k) for k in ("validé", "partiel", "absent")}
    hist = historique(existant)
    if session:
        ligne = f"| {session} | {date.today().isoformat()} | {n['validé']} | {n['partiel']} | {n['absent']} | {len(perimetre)} |"
        if f"| {session} |" not in hist:
            hist += "\n" + ligne
    rang = {"validé": "✅", "partiel": "◐", "absent": "·"}
    lignes = [
        "# Tableau de bord — la liste à 100 %",
        "",
        "*Généré par `python outils/tableau_de_bord.py --ecrire` (S480) — ne pas modifier à la main.* La liste du projet fini dit l'état de",
        "chaque point ; le plan de complétion, sa campagne ; ce tableau les croise. La fin du système de l'eau : tous les points validés",
        "([ADR-218](../adr/ADR-218-le-systeme-de-l-eau-complet.md)).",
        "",
        f"**Périmètre : {len(perimetre)} points** (5.11 hors). **Validés : {n['validé']}** ({100 * n['validé'] / len(perimetre):.1f} %) — "
        f"partiels : {n['partiel']} — absents : {n['absent']}.",
        "",
        "## Par campagne",
        "",
        "Légende : ✅ validé, ◐ partiel, · absent. L'ordre est celui du [plan de complétion](PLAN-COMPLETION-S475.md).",
        "",
        "| campagne | validés | partiels | absents | points |",
        "|---|---:|---:|---:|---|",
    ]
    vus = set()
    for k, nom, pts in camp:
        c = {s: sum(1 for p in pts if e.get(p) == s) for s in ("validé", "partiel", "absent")}
        vus.update(pts)
        detail = " ".join(f"{rang.get(e.get(p, ''), '?')}{p}" for p in pts)
        lignes.append(f"| **{k}** {nom} | {c['validé']} | {c['partiel']} | {c['absent']} | {detail} |")
    hors_plan = [p for p in perimetre if p not in vus and e[p] != "validé"]
    lignes += ["", f"**Points ouverts hors de toute campagne : {len(hors_plan)}**" + (f" — {', '.join(hors_plan)}." if hors_plan else "."),
               "", "## Les points ouverts, un par ligne", "", "| point | état | titre |", "|---|---|---|"]
    for p in sorted(perimetre, key=lambda x: tuple(map(int, x.split(".")))):
        if e[p] != "validé":
            lignes.append(f"| {p} | {e[p]} | {t.get(p, '')} |")
    lignes += ["", "## Historique du décompte", "", "Une ligne par session qui a écrit le tableau avec `--session`.", "", HIST_DEBUT, hist,
               HIST_FIN, ""]
    return "\n".join(lignes)


def ecarts(liste: str, plan: str, tableau: str) -> list:
    if not tableau:
        return [f"{TABLEAU} absent : python outils/tableau_de_bord.py --ecrire"]
    if rendre(liste, plan, tableau) != tableau:
        return [f"{TABLEAU} ne suit plus la liste ou le plan : python outils/tableau_de_bord.py --ecrire"]
    return []


def main(argv) -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    liste = (ROOT / LISTE).read_text(encoding="utf-8")
    plan = (ROOT / PLAN).read_text(encoding="utf-8")
    chemin = ROOT / TABLEAU
    existant = chemin.read_text(encoding="utf-8") if chemin.exists() else ""
    if "--check" in argv:
        e = ecarts(liste, plan, existant)
        for x in e:
            print(x)
        return 1 if e else 0
    session = argv[argv.index("--session") + 1] if "--session" in argv else None
    texte = rendre(liste, plan, existant, session)
    if "--ecrire" in argv:
        chemin.write_text(texte, encoding="utf-8", newline="\n")
    e = etats(liste)
    per = [p for p in e if e[p] != "hors"]
    print(f"TABLEAU_DE_BORD perimetre={len(per)} valides={sum(e[p] == 'validé' for p in per)} "
          f"partiels={sum(e[p] == 'partiel' for p in per)} absents={sum(e[p] == 'absent' for p in per)}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
