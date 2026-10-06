# -*- coding: utf-8 -*-
"""Le rituel outillé — S480 (REPRISE §6 ; AGENTS : l'amorce).

Ce qui est mécanique dans l'amorce et le rituel de fin se fait ici ; ce qui demande un jugement (le journal, la suite, les lignes
de la liste) reste écrit par la session, et l'outil vérifie qu'il l'a été. Il ne committe jamais.

    python outils/rituel.py debut
        L'amorce d'AGENTS.md : copies de travail, branches, derniers commits, état ; le jeton et son âge, et ce qu'il commande
        (prendre, s'arrêter, reprise à chaud) ; la session en cours d'EN-COURS ; le tableau de bord ; les calculs longs.
    python outils/rituel.py fin --session S480 --suivante "<texte>" [--maillons "<texte>"] [--lot] [--sans-banc "<raison>"]
        Vérifie : toutes les cases d'EN-COURS cochées sauf la dernière (le rituel), une entrée `## S480` au journal (vingt lignes
        au plus). Puis : régénère le tableau de bord (avec sa ligne d'historique), les décisions et les anomalies ; met à jour
        les calculs ; libère le jeton (battement, dernière session tirée du journal, suivante, maillons ; avec --lot, la ligne
        Registres) ; coche le rituel ; lance `etat_projet.py --check`. Échoue, sans rien écrire, si une vérification manque.
        **S483 (ADR-222 D3)** : lance d'abord le banc de non-régression (`outils/non_regression.py`, ≈ 2 min) ; un échec arrête le
        rituel. `--sans-banc "<raison>"` le saute — la raison est imprimée, et va au journal.
"""
import re
import subprocess
import sys
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "outils"))
import anomalies  # noqa: E402
import calcul  # noqa: E402
import decisions  # noqa: E402
import tableau_de_bord  # noqa: E402

REPRISE, EN_COURS, JOURNAL = ROOT / "REPRISE.md", ROOT / "notes/EN-COURS.md", ROOT / "notes/JOURNAL.md"


def git(*args) -> str:
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, encoding="utf-8").stdout.rstrip()


def lire(p: Path) -> str:
    return p.read_text(encoding="utf-8")


def ecrire(p: Path, t: str) -> None:
    p.write_text(t, encoding="utf-8", newline="\n")


def jeton(reprise: str) -> dict:
    bloc = reprise.split("## Jeton de session", 1)[1].split("```", 2)[1]
    return {m.group(1).strip(): m.group(2).strip() for m in re.finditer(r"^([^:\n]+?)\s*: (.*)$", bloc, flags=re.M)}


def remplacer_champ(reprise: str, champ: str, valeur: str) -> str:
    motif = re.compile(rf"^({re.escape(champ)}\s*: ).*$", flags=re.M)
    assert motif.search(reprise), champ
    return motif.sub(lambda m: m.group(1) + valeur, reprise, count=1)


def maintenant() -> datetime:
    return datetime.now().astimezone()


def debut() -> int:
    print("== copies de travail\n" + git("worktree", "list"))
    print("== branches\n" + git("branch", "-a"))
    print("== derniers commits\n" + git("log", "--oneline", "-15"))
    print("== état\n" + (git("status", "--short") or "(propre)"))
    j = jeton(lire(REPRISE))
    print("== jeton")
    for k in ("JETON", "Battement", "Session en cours", "Dernière session", "Session suivante", "Maillons", "Registres"):
        print(f"{k:17}: {j.get(k, '?')}")
    etat = j.get("JETON", "?")
    try:
        age = (maintenant() - datetime.strptime(j["Battement"], "%Y-%m-%d %H:%M %z")).total_seconds() / 3600
    except (KeyError, ValueError):
        age = None
    if etat.startswith("libre"):
        avis = "libre → le prendre (Agent, plan), démarrage à froid : REPRISE §3"
    elif etat.startswith("archivé"):
        avis = "archivé → ne pas travailler ici ; rejoindre la branche désignée"
    elif etat.startswith("occupé") and age is not None and age < 2:
        avis = f"occupé, battement il y a {age:.1f} h → une autre session travaille : s'arrêter"
    else:
        avis = f"{etat}, battement il y a {age:.1f} h → reprise à chaud : notes/EN-COURS.md" if age is not None else \
            f"{etat} → reprise à chaud : notes/EN-COURS.md"
    print("→ " + avis)
    en = lire(EN_COURS).split("## Session en cours", 1)[-1]
    print("== EN-COURS\n" + next((l for l in en.splitlines() if l.startswith("Session :")), "?")[:200])
    for l in en.splitlines():
        if re.match(r"- \[[ x>]\] \*\*P", l):
            print(l[:160])
    tableau_de_bord.main([])
    print("== calculs longs")
    for l in calcul.mettre_a_jour():
        print(l[:200])
    return 0


def fin(argv) -> int:
    def option(nom, defaut=None):
        return argv[argv.index(nom) + 1] if nom in argv else defaut

    session, suivante = option("--session"), option("--suivante")
    if not session or not suivante:
        print("usage : rituel.py fin --session Snnn --suivante \"<texte>\" [--maillons \"<texte>\"] [--lot]")
        return 2
    n = int(session.lstrip("S"))
    manques = []
    en = lire(EN_COURS)
    plan = re.findall(r"^- \[([ x>])\] \*\*(P[\w-]+)\*\*", en.split("## Session en cours", 1)[-1], flags=re.M)
    if not plan:
        manques.append("EN-COURS : aucun plan lu")
    else:
        ouvertes = [p for c, p in plan[:-1] if c != "x"]
        if ouvertes:
            manques.append(f"EN-COURS : étapes non cochées avant le rituel : {', '.join(ouvertes)}")
    journal = lire(JOURNAL)
    entree = re.search(rf"^## {session} — [^\n]*\n(.*?)(?=^## S\d+|\Z)", journal, flags=re.M | re.S)
    if not entree:
        manques.append(f"journal : pas d'entrée « ## {session} — … »")
    else:
        lignes = [l for l in entree.group(1).splitlines() if l.strip()]
        if len(lignes) > 20:
            manques.append(f"journal : l'entrée {session} a {len(lignes)} lignes de texte (vingt au plus)")
    if not manques:
        if "--sans-banc" in argv:
            print(f"BANC sauté : {option('--sans-banc')} (à dire au journal)")
        else:
            banc = subprocess.run([sys.executable, str(ROOT / "outils/non_regression.py")], cwd=ROOT, capture_output=True, text=True,
                                  encoding="utf-8", errors="replace")
            for ligne in banc.stdout.splitlines()[-4:]:
                print(ligne)
            if banc.returncode != 0:
                manques.append("le banc de non-régression échoue (ci-dessus) : corriger, ou réinscrire les empreintes si le changement est voulu")
    if manques:
        for m in manques:
            print("MANQUE " + m)
        return 1
    titre = re.search(rf"^## {session} — \d{{4}}-\d\d-\d\d — (.*)$", journal, flags=re.M)
    # les registres générés
    liste, plan_k = lire(ROOT / tableau_de_bord.LISTE), lire(ROOT / tableau_de_bord.PLAN)
    t = ROOT / tableau_de_bord.TABLEAU
    ecrire(t, tableau_de_bord.rendre(liste, plan_k, lire(t) if t.exists() else "", session))
    ecrire(ROOT / decisions.REGISTRE, decisions.rendre(decisions.lire_adrs()))
    ecrire(ROOT / anomalies.REGISTRE, anomalies.rendre(lire(ROOT / anomalies.SOURCE)))
    calcul.mettre_a_jour()
    # le jeton
    r = lire(REPRISE)
    ancienne = jeton(r).get("Dernière session", "")
    avant = re.match(r"(S\d+)\s*—\s*([^(;]*)", ancienne)
    derniere = f"{session} — {titre.group(1) if titre else ''} ([journal](notes/JOURNAL.md))"
    if avant and avant.group(1) != session:
        derniere += f". Avant : {avant.group(1)} ({avant.group(2).strip().rstrip(' :—')})"
    r = remplacer_champ(r, "JETON", "libre")
    r = remplacer_champ(r, "Battement", maintenant().strftime("%Y-%m-%d %H:%M %z")[:-2] + ":" + maintenant().strftime("%z")[-2:])
    r = remplacer_champ(r, "Session en cours", "aucune")
    r = remplacer_champ(r, "Dernière session", derniere)
    r = remplacer_champ(r, "Session suivante", suivante)
    if option("--maillons"):
        r = remplacer_champ(r, "Maillons", option("--maillons"))
    if "--lot" in argv:
        r = remplacer_champ(r, "Registres", f"dernier lot {session} (ADR-213 D3) ; le prochain au plus tard en S{n + 3}")
    ecrire(REPRISE, r)
    # le rituel coché
    dernier = plan[-1][1]
    en = re.sub(rf"^- \[[ >]\] \*\*{re.escape(dernier)}\*\*", f"- [x] **{dernier}**", en, count=1, flags=re.M)
    en = re.sub(rf"^Session : {session} — \*\*en cours\*\*", f"Session : {session} — **terminée**", en, count=1, flags=re.M)
    ecrire(EN_COURS, en)
    lot = jeton(r).get("Registres", "")
    m = re.search(r"au plus tard en S(\d+)", lot)
    rappel = ""
    if m and int(m.group(1)) <= n and "--lot" not in argv:
        rappel = f"RAPPEL le lot des registres (REPRISE §6, points 6 à 8) était dû en S{m.group(1)} : le faire, puis --lot"
        print(rappel)
    controle = subprocess.run([sys.executable, str(ROOT / "outils/etat_projet.py"), "--check"], cwd=ROOT, capture_output=True,
                              text=True, encoding="utf-8")
    print("\n".join(controle.stdout.splitlines()[-4:]))
    print(f"RITUEL {session} : jeton libre, registres régénérés, rituel coché ; etat_projet --check = {controle.returncode}. "
          "Reste : le commit de l'étape, et fermer sa copie de travail si elle est isolée (AGENTS).")
    # S511 (ADR-231 D2) : le rappel aussi en dernière ligne — une lecture de la fin de la sortie ne le manque plus.
    if rappel:
        print(rappel)
    return controle.returncode


def main(argv) -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    if argv[:1] == ["debut"]:
        return debut()
    if argv[:1] == ["fin"]:
        return fin(argv[1:])
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
