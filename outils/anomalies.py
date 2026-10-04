# -*- coding: utf-8 -*-
"""Les anomalies ouvertes — S480 (BOUSSOLE : ne pas travailler dans le flou).

Lit `docs/registres/ANGLES-MORTS.md` : les entrées dont l'en-tête porte un statut, `**Annn — Snnn, date (sévérité N, statut).
Titre.**` (le format depuis ≈ S280), et leurs suites — `**Annn — note datée du … (Snnn). Verdict.**`, `**Annn — close en Snnn**`,
`*Note du …, Snnn, sur Annn*` ou `*Correction et clôture du …, Snnn*` (une note en italique sans « sur Annn » suit l'entrée sous
laquelle elle est écrite). Écrit `docs/registres/ANOMALIES-OUVERTES.md` : les ouvertes par sévérité, avec leur dernière suite,
puis les closes en une ligne. Rien ne s'y écrit à la main ; le registre source reste le seul où l'on écrit.

L'état se lit par mots sur le statut de l'en-tête puis sur chaque suite, la dernière qui tranche l'emporte : *close*, *fermée*,
*corrigée*, *résolu(e)*, *levée*, *supprimé*, *accepté*, *clôture* → close ; *rouverte* → ouverte. Une note en italique qui nomme une
autre anomalie ne tranche pas (en S436, « A324 corrigée » est écrit sous A320). Les entrées plus anciennes (le tableau général, sans
statut dans leur en-tête) ne sont pas lues.

    python outils/anomalies.py              # le résumé
    python outils/anomalies.py --ecrire     # réécrit le registre
    python outils/anomalies.py --check      # échoue si le registre ne suit plus ANGLES-MORTS

`etat_projet.py --check` appelle `ecarts`.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SOURCE = "docs/registres/ANGLES-MORTS.md"
REGISTRE = "docs/registres/ANOMALIES-OUVERTES.md"

ENTREE = re.compile(r"^\*\*A(\d+) — S(\d+), (\d{4}-\d\d-\d\d) \(sévérité (\d), ([^)]*)\)\.?\s*(.*)$")
SUITE_GRAS = re.compile(r"^\*\*A(\d+) — (?:note datée du [^(]*\(S(\d+)\)|close en S(\d+)[^*]*)(.*)$")
SUITE_ITALIQUE = re.compile(r"^\*(?:Note|Correction[^,]*) du [^,]*, (S\d+(?:–S\d+)?)(?:, sur A(\d+))?\*\s*(.*)$")


def gras(texte: str) -> str:
    """Le texte jusqu'à la fermeture du gras, sans balises — le titre ou le verdict ; un gras vide (`.** **Titre**`) est sauté."""
    t = texte.lstrip("* ")
    t = t.split("**", 1)[0] if "**" in t else t
    return " ".join(t.replace("*", "").split()).strip(" .:")


def extrait(texte: str, n: int = 160) -> str:
    """La première phrase, sans balises ni lien de preuve en tête, coupée à `n` caractères sur un mot."""
    t = re.sub(r"^\(\[[^\]]*\]\([^)]*\)[^)]*\)\s*:?\s*", "", texte.strip())
    t = " ".join(re.sub(r"\[([^\]]*)\]\([^)]*\)", r"", t).replace("*", "").replace("`", "").split())
    t = t.split(". ")[0].strip(" .:")
    return t if len(t) <= n else t[:n].rsplit(" ", 1)[0] + " …"


def tranche(texte: str):
    t = texte.lower().replace("*", "")
    if "rouvert" in t:
        return "ouverte"
    if re.search(r"\b(close|clos|fermée|corrigée?|résolue?|levée?|supprimée?|acceptée?|clôture)\b", t):
        return "close"
    if "ouverte" in t:
        return "ouverte"
    return None


def lire(source: str) -> dict:
    """n → {session, date, sévérité, statut, titre, suites : [(session, texte)], état}."""
    entrees, courant = {}, None
    lignes = source.splitlines()
    for i, l in enumerate(lignes):
        suite_ligne = " ".join(lignes[i:i + 3])
        m = ENTREE.match(l)
        if m:
            n = int(m.group(1))
            titre = gras(" ".join([m.group(6)] + lignes[i + 1:i + 3]))
            titre = titre if len(titre) <= 140 else extrait(titre, 140)
            e = dict(session=f"S{m.group(2)}", date=m.group(3), severite=int(m.group(4)), statut=m.group(5).replace("*", ""),
                     titre=titre, suites=[], ligne=i + 1)
            if n in entrees:  # un numéro donné deux fois (A303) : la seconde entrée garde le sien, suffixée
                n = f"{n} bis"
            entrees[n] = e
            courant = n
            continue
        m = SUITE_GRAS.match(l)
        if m and int(m.group(1)) in entrees:
            n = int(m.group(1))
            s = f"S{m.group(2) or m.group(3)}"
            if m.group(2):
                apres = suite_ligne.split("—", 1)[1].split(")", 1)[1]
                verdict = gras(apres) or extrait(apres.split("**", 1)[-1])
            else:
                verdict = "close"
            entrees[n]["suites"].append((s, verdict, True))
            courant = n
            continue
        m = SUITE_ITALIQUE.match(l)
        if m:
            n = int(m.group(2)) if m.group(2) else courant
            if n in entrees:
                corps = extrait(" ".join([m.group(3)] + lignes[i + 1:i + 3]))
                etiquette = l.split("*", 2)[1]
                autre = any(int(x) != n for x in re.findall(r"\bA(\d{3})\b", corps))
                entrees[n]["suites"].append((m.group(1), (etiquette.split(" du ")[0] + " : " if "lôture" in etiquette else "") + corps,
                                             not autre))
    for e in entrees.values():
        etat = tranche(e["statut"]) or "ouverte"
        for _, texte, decide in e["suites"]:
            etat = (tranche(texte) if decide else None) or etat
        e["etat"] = etat
    return entrees


def rang(n) -> tuple:
    return (int(str(n).split()[0]), str(n))


def rendre(source: str) -> str:
    entrees = lire(source)
    ouvertes = [(n, e) for n, e in entrees.items() if e["etat"] == "ouverte"]
    closes = [(n, e) for n, e in entrees.items() if e["etat"] == "close"]
    lignes = [
        "# Anomalies ouvertes",
        "",
        "*Généré par `python outils/anomalies.py --ecrire` (S480) — ne pas modifier à la main ; on écrit dans le",
        "[registre des angles morts](ANGLES-MORTS.md), seul.* Lu : les entrées dont l'en-tête porte un statut (depuis ≈ S280) et leurs",
        "suites datées ; l'état est lu par mots sur la dernière suite qui tranche (voir l'outil). Les entrées plus anciennes, au tableau",
        "général du registre, ne sont pas lues ici.",
        "",
        f"**{len(entrees)} entrées lues — {len(ouvertes)} ouvertes, {len(closes)} closes.** Sévérité : 1 = refonte d'architecture si découvert",
        "tard, 2 = refonte d'un sous-système, 3 = travail localisé.",
        "",
        "## Ouvertes",
        "",
        "| anomalie | sév. | ouverte | titre | dernière suite |",
        "|---|---:|---|---|---|",
    ]
    for n, e in sorted(ouvertes, key=lambda x: (x[1]["severite"], rang(x[0]))):
        der = f"{e['suites'][-1][0]} : {e['suites'][-1][1]}" if e["suites"] else "—"
        lignes.append(f"| A{n} | {e['severite']} | {e['session']} | {e['titre'].replace('|', '/')} | {der.replace('|', '/')} |")
    lignes += ["", "## Closes", "", "| anomalie | sév. | ouverte | close | titre |", "|---|---:|---|---|---|"]
    for n, e in sorted(closes, key=lambda x: rang(x[0])):
        quand = next((s for s, t, d in reversed(e["suites"]) if d and tranche(t) == "close"), e["session"])
        lignes.append(f"| A{n} | {e['severite']} | {e['session']} | {quand} | {e['titre'].replace('|', '/')} |")
    lignes.append("")
    return "\n".join(lignes)


def ecarts(source: str, registre: str) -> list:
    if not registre:
        return [f"{REGISTRE} absent : python outils/anomalies.py --ecrire"]
    if rendre(source) != registre:
        return [f"{REGISTRE} ne suit plus ANGLES-MORTS : python outils/anomalies.py --ecrire"]
    return []


def main(argv) -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    source = (ROOT / SOURCE).read_text(encoding="utf-8-sig")
    chemin = ROOT / REGISTRE
    existant = chemin.read_text(encoding="utf-8") if chemin.exists() else ""
    if "--check" in argv:
        e = ecarts(source, existant)
        for x in e:
            print(x)
        return 1 if e else 0
    texte = rendre(source)
    if "--ecrire" in argv:
        chemin.write_text(texte, encoding="utf-8", newline="\n")
    entrees = lire(source)
    print(f"ANOMALIES lues={len(entrees)} ouvertes={sum(e['etat'] == 'ouverte' for e in entrees.values())} "
          f"closes={sum(e['etat'] == 'close' for e in entrees.values())}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
