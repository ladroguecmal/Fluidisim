# -*- coding: utf-8 -*-
"""La liste du projet fini rangée par dépendance — S352, ADR-190 D3.

Pour chaque point **non validé** de `docs/LISTE-PROJET-FINI.md` : son système (A haute mer, B volumique, C couplage,
H hors des trois), ce qu'une session peut en faire **maintenant**, les points qu'il **attend** pour son périmètre
final, une attente **extérieure** (ADR-190 D5) s'il en a une. Les **fronts** et la colonne « débloque » se
**calculent** : front 0 = rien d'autre qu'une session à attendre ; n = après le front n − 1 ; E = un fait ou une
action de l'utilisateur, directement ou par un point attendu.

    python outils/dependances_liste.py            # résumé
    python outils/dependances_liste.py --ecrire   # réécrit les tables du registre
    python outils/dependances_liste.py --check    # échoue si le registre ne suit plus la liste

`etat_projet.py --check` appelle `ecarts` : un point ouvert absent d'ici, une dépendance inconnue, un cycle, ou des
tables qui ne sont plus celles que ces données produisent. Quand un point change d'état ou d'attente, on corrige
ses données ici, puis `--ecrire`.
"""
import io
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LISTE = "docs/LISTE-PROJET-FINI.md"
REGISTRE = "docs/registres/DEPENDANCES-LISTE.md"
DEBUT, FIN = "<!-- tables : outils/dependances_liste.py --ecrire -->", "<!-- fin des tables -->"

# point : (système, ce qu'une session peut en faire maintenant, points attendus, attente extérieure ou None).
D = {
 "1.1": ("C", "—", ["3.9", "5.10", "4.8"], None),
 "1.3": ("H", "l'interface des solides de SPEC-004 §7, sur la paroi de δ existante", ["6.4", "2.7"], None),
 "1.4": ("H", "`WaterSystem`, qui porte l'ordonnanceur, l'oubli et l'estimateur de coût", ["9.9", "1.6"], None),
 "1.5": ("B", "la grille de référence et ses zones actives (ADR-006)", ["10.1"], None),
 "1.6": ("B", "—", ["1.5"], None),
 "1.7": ("H", "le déterminisme entre les chemins d'exécution de ce PC (ADR-219 D2)", [], None),
 "1.8": ("H", "les référentiels mobiles, puis la planète (ADR-002)", [], None),
 "2.1": ("A", "anisotropie, asymétrie des pentes, B1 complet", ["11.2"], None),
 "2.2": ("A", "marée, niveau moyen variable, adoption par défaut", ["2.8"], None),
 "2.3": ("A", "—", ["2.6", "2.7"], None),
 "2.4": ("A", "—", ["2.6", "2.7"], None),
 "2.5": ("A", "—", ["2.4"], None),
 "2.6": ("A", "le courant macroscopique, du vecteur au champ (ADR-011)", [], None),
 "2.7": ("A", "l'entrée dans B, isobathes droites, faite (ADR-196, S364) ; les chemins de B et Godot, puis la 2D et la marée ; hauts-fonds isolés", [], None),
 "2.8": ("A", "—", ["2.7", "3.6"], "la fin du projet : météo et son en dernier (ADR-197 D5)"),
 "2.9": ("A", "—", ["2.7"], None),
 "3.1": ("A", "l'eau peu profonde faite (S528) ; la gerbe, la profondeur variable", ["4.12"], None),
 "3.2": ("A", "durées longues ; C07 passe entier (S519–S527)", [], None),
 "3.3": ("A", "la source d'explosion de W, champ lointain", ["4.16", "7.4"], None),
 "3.4": ("A", "—", ["2.7", "3.6"], None),
 "3.5": ("A", "—", ["2.7", "3.6"], None),
 "3.6": ("A", "—", ["2.7"], None),
 "3.7": ("A", "—", ["10.1"], None),
 "3.8": ("A", "la saturation par la pression seule (A261)", [], None),
 "3.9": ("A", "W évalué au-dessus du plan moyen, comme B (ADR-154)", [], None),
 "4.1": ("B", "—", ["4.16"], None),
 "4.2": ("B", "plus de deux domaines ; l'ordonnanceur dans l'afficheur", [], None),
 "4.3": ("B", "—", ["1.6"], None),
 "4.4": ("B", "`nz` variable : le redimensionnement vertical", ["6.1"], None),
 "4.5": ("B", "une disparition progressive", [], "un verdict visuel des passages (A319)"),
 "4.6": ("C", "la houle progressive traversante sur une durée utile ; B4", ["3.9"], None),
 "4.7": ("C", "la réflexion d'un front oblique ; C05", [], None),
 "4.8": ("C", "A320 (les termes croisés sous forme de Bernoulli), puis l'ordre E, critère refondu (ADR-198)", [], None),
 "4.9": ("B", "—", ["1.5", "4.2"], None),
 "4.10": ("B", "—", ["4.3"], None),
 "4.11": ("B", "—", ["1.6", "4.20", "12.3"], None),
 "4.12": ("B", "le raccord particules ↔ colonnes (A316)", ["4.16", "7.4"], None),
 "4.13": ("B", "la coque en marche dans la production de δ", ["6.4", "4.16"], None),
 "4.14": ("B", "—", ["2.7", "3.5", "4.16", "6.5"], None),
 "4.15": ("B", "le couplage à B/W sur fond coupé ; un modèle de turbulence", [], None),
 "4.16": ("B", "le raccord (A316), puis APIC en 3D", [], None),
 "4.17": ("B", "C16 fait dans δ linéaire (S542–S543) ; Coriolis, la carte, C06", ["1.8"], None),
 "4.18": ("C", "le compteur sur la carte ; énergie et quantité de mouvement ; C09", [], None),
 "4.19": ("B", "d'autres scènes ; plusieurs domaines en direct ; un 99ᵉ centile en direct", [], None),
 "4.20": ("B", "—", ["4.16"], None),
 "4.21": ("C", "le mode relatif dans la production GPU ; W ; A320", [], None),
 "5.2": ("H", "la précision des grands volumes (A269) ; des formes courbes cuites", [], None),
 "5.3": ("H", "—", ["5.10"], None),
 "5.4": ("H", "le `C_d` selon l'ouverture ; pertes et énergie de la pompe", [], None),
 "5.5": ("H", "l'absorption par le sol ; la pluie hors contenant ; l'exposition calculée depuis les objets posés", ["2.8"], None),
 "5.6": ("H", "le seuil adaptatif", [], None),
 "5.7": ("H", "`liquid_id` (A17)", [], None),
 "5.8": ("H", "—", ["5.4"], None),
 "5.9": ("H", "C17 passé (S538), l'évent à débit limité (S547) ; la flottabilité de la poche, les brèches en jeu", ["5.2", "5.3", "7.5"], None),
 "5.10": ("H", "une dynamique visible (δ sur GPU, 5 à 10 cm) ; le bac tampon ; V qui déclenche δ", ["6.5"], None),
 "5.11": ("H", "—", [], "hors du périmètre par décision de l'utilisateur (ADR-197 D4) ; ne se rouvre que par lui"),
 "5.12": ("H", "le stockage durable", ["10.1"], None),
 "6.1": ("H", "W derrière la requête ; les autres degrés de liberté ; C11 ; B6", [], None),
 "6.2": ("H", "W derrière la requête", ["2.6", "4.15"], None),
 "6.3": ("H", "le corps en marche couplé à la source de sillage", [], None),
 "6.4": ("B", "la coque dans la production GPU de δ ; C23 sur le système", [], None),
 "6.5": ("B", "le décor dans la production GPU ; un décor qui perce la surface", [], None),
 "6.6": ("H", "—", ["6.1", "6.4", "4.13", "3.2", "5.9"], None),
 "6.7": ("B", "—", ["6.2"], None),
 "6.8": ("H", "—", ["4.12", "6.1"], None),
 "7.1": ("A", "sources de W, δ, vent ; demi-vies et transfert (B9)", [], None),
 "7.2": ("B", "—", ["4.16"], None),
 "7.3": ("B", "—", ["4.12"], None),
 "7.4": ("B", "—", ["4.16"], None),
 "7.5": ("B", "la poche comprimée d'un corps faite (S539) ; l'adiabatique, la poche qui s'échappe, le vide", ["7.4"], None),
 "7.6": ("H", "—", ["6.1"], None),
 "7.7": ("H", "la publication par tuiles depuis B, W et V (ADR-018)", [], None),
 "7.8": ("H", "les événements et paramètres publiés (ADR-016)", [], "la fin du projet, par Wwise, l'audio du jeu (ADR-197 D5, ADR-219)"),
 "8.1": ("H", "le cœur branché dans Godot, moteur du jeu entier (GDExtension ; ADR-197 D2)", [], None),
 "8.2": ("H", "le LOD du maillage ; déplacement ou normales selon la vue", [], None),
 "8.3": ("H", "le filtre des impacts ; le LOD temporel", [], None),
 "8.4": ("H", "l'écume (suspendue) ; le spray ; les bulles ; les gerbes de pluie (ADR-205, pièce 4) ; leurs niveaux de détail", ["7.1", "7.2", "7.3", "7.4"], None),
 "8.5": ("H", "particules, eaux chargées, caustiques sur les objets (transparence, réfraction : S359 ; caustiques du fond : S361)", [], None),
 "8.6": ("H", "la caméra à demi immergée (ADR-019 §6) ; bulles, rayons, turbidité ; le coût du profil immergé (B11)", ["8.5"], None),
 "8.7": ("C", "une frontière sans fondu ; la tolérance de pente", [], "le verdict de la frontière (R18 reçu, ADR-197 D8)"),
 "8.8": ("H", "un certificat d'absence d'alias", [], None),
 "8.9": ("H", "capillaires du vent ; queue des perturbations W ; coût (la queue de B par FFT dans Godot : S360 ; les rides de pluie, S379 : une texture à moments)", [], None),
 "8.10": ("H", "de nouvelles revues, préparées", [], "les verdicts de l'utilisateur : poses, animation, scénarios"),
 "9.1": ("H", "`W_urgence` ; le banc B8 ; `W_gameplay` par le jeu d'essai, puis DyingStar (ADR-219 D3)", [], None),
 "9.2": ("H", "le domaine qui précède la caméra : prédiction, orientation", [], None),
 "9.3": ("H", "un corps quelconque, le vent, l'entrée orientée consommée par δ", [], None),
 "9.4": ("H", "— ; les objets du jeu d'essai, puis de DyingStar (ADR-219 D3)", ["9.3"], None),
 "9.5": ("H", "—", ["9.6"], None),
 "9.6": ("H", "—", ["9.3"], None),
 "9.7": ("H", "la condensation hors caméra", [], None),
 "9.8": ("H", "la borne murale ; l'estimateur dans le cœur", [], None),
 "9.9": ("H", "rangs 3, 6 et 7 ; régulateur PI ; bande morte de l'échelle", ["7.2", "4.3"], "un verdict visuel du prix du rang 1 (A319)"),
 "9.10": ("H", "les profils de qualité (I-16), mesurés sur ce PC bridé (ADR-219 D2)", [], None),
 "9.11": ("H", "la scène représentative réunie, mesurée", ["4.19"], None),
 "9.12": ("H", "—", ["1.4"], None),
 "9.13": ("H", "la réserve d'événement d'ADR-012 §6", [], None),
 "10.1": ("H", "notre format de réplication, client et serveur locaux (ADR-219 D1)", [], None),
 "10.2": ("H", "un hôte serveur sans δ ni rendu (C18), un processus sans fenêtre sur ce PC (ADR-219 D2)", [], None),
 "10.3": ("H", "le déterminisme entre les chemins d'exécution de ce PC (ADR-219 D2)", [], None),
 "10.4": ("H", "—", ["10.1"], None),
 "10.5": ("H", "—", ["10.1"], None),
 "10.6": ("H", "C19 complet, en local", ["10.1", "5.12"], None),
 "10.8": ("H", "le bus `WaveEvent` et ses canaux (SPEC-006)", [], None),
 "10.9": ("H", "le jeu d'essai, puis DyingStar (ADR-219 D3)", [], None),
 "11.1": ("H", "—", ["1.8"], None),
 "11.2": ("A", "le descripteur de région de mer (I-09)", [], None),
 "11.3": ("H", "—", ["3.4", "6.6"], None),
 "11.4": ("H", "la généralisation, le LOD temporel", ["8.3"], None),
 "11.5": ("H", "ce PC, cible de livraison ; la seconde cible : le bridage de 9.10 (ADR-219 D2)", ["9.10"], None),
 "12.1": ("H", "la détection d'obsolescence", [], None),
 "12.2": ("H", "—", ["2.4"], None),
 "12.3": ("H", "—", ["2.8"], None),
 "12.4": ("H", "une carte de hauteurs qui imite les tuiles HEALPix de DyingStar (ADR-219 D4)", [], None),
 "12.5": ("H", "—", ["12.1"], None),
 "13.1": ("H", "les étages manquants de SPEC-003", [], None),
 "13.2": ("H", "chaque cas exécuté sur le système", ["4.7", "3.2", "4.18", "6.1", "7.4", "7.1", "7.6", "4.17", "5.9", "6.8", "5.10", "10.6"], None),
 "13.3": ("H", "chaque banc exécuté", ["2.1", "4.3", "6.1", "11.5", "9.1", "7.1", "4.12", "8.6"], None),
 "13.4": ("H", "lire les dépôts publics de DyingStar ; le jeu d'essai sur sa pile (ADR-219 D3)", ["11.1", "12.4", "10.1", "9.11", "13.2"], None),
}

SECTIONS = ["Socle et architecture", "Grandes masses d'eau et fond (B)", "Ondes propagatives (W)",
            "Simulation volumique locale (δ)", "Volumes finis et inondations (V)", "Solides et flottabilité",
            "Phénomènes secondaires", "Rendu et niveaux de détail visuels",
            "Activation, prédiction, budget et dégradation", "Multijoueur, autorité et persistance",
            "Grande échelle et très grands événements", "Outillage auteur et données cuites", "Validation du système"]


def cle(p: str) -> tuple:
    return tuple(map(int, p.split(".")))


def points(liste: str) -> tuple[dict, set]:
    """Titres de tous les points, et l'ensemble des points non validés."""
    corps = liste.partition("## Décompte")[0]
    trouves = re.findall(r"\n- \[([ x])\] \*\*(\d+\.\d+) ([^*]+)\*\*", corps)
    return {n: t.strip().rstrip(" :") for _, n, t in trouves}, {n for x, n, _ in trouves if x == " "}


def fronts(ouverts: set) -> tuple[dict, list]:
    """Front de chaque point ouvert : un entier, ou ("E", le point attendu qui l'y met). Rend aussi les cycles."""
    memo, cycles = {}, []

    def front(p, pile=()):
        if p in memo:
            return memo[p]
        if p in pile:
            cycles.append(" → ".join(pile + (p,)))
            return ("E", p)
        _, _, attend, ext = D[p]
        fs = [(a, front(a, pile + (p,))) for a in attend if a in ouverts and a in D]
        if ext is not None:
            r = ("E", None)
        elif any(isinstance(f, tuple) for _, f in fs):
            r = ("E", next(a for a, f in fs if isinstance(f, tuple)))
        else:
            r = 1 + max(f for _, f in fs) if fs else 0
        memo[p] = r
        return r

    return {p: front(p) for p in sorted(ouverts, key=cle) if p in D}, cycles


def tables(liste: str) -> str:
    """Le bloc de tables que le registre doit porter, calculé depuis les données et l'état de la liste."""
    titres, ouverts = points(liste)
    fr, _ = fronts(ouverts)
    debloque = {p: [] for p in D}
    for p, (_, _, attend, _) in D.items():
        if p in ouverts:
            for a in attend:
                if a in debloque:
                    debloque[a].append(p)
    lignes = ["| front | points | lesquels |", "|---|---:|---|"]
    par_front = {}
    for p, f in fr.items():
        par_front.setdefault("E" if isinstance(f, tuple) else str(f), []).append(p)
    for k in sorted(par_front, key=lambda k: (k == "E", int(k) if k != "E" else 0)):
        ps = sorted(par_front[k], key=cle)
        lignes.append(f"| **{k}** | {len(ps)} | {', '.join(ps)} |")
    lignes += ["", "**Ce qu'on demandera à l'utilisateur**, au moment où le point bloque (ADR-190 D5) :", "",
               "| point | attente extérieure |", "|---|---|"]
    lignes += [f"| **{p}** | {D[p][3]} |" for p in sorted(fr, key=cle) if D[p][3]]
    for i, nom in enumerate(SECTIONS, 1):
        ps = sorted([p for p in fr if p.split(".")[0] == str(i)], key=cle)
        lignes += ["", f"### {i}. {nom}", "", "| point | sys. | maintenant | attend | débloque | front |",
                   "|---|---|---|---|---|---|"]
        for p in ps:
            sysm, maint, attend, ext = D[p]
            f = fr[p]
            if not isinstance(f, tuple):
                cell = f"**{f}**"
            elif ext:
                cell = f"**E** — {ext}"
            else:
                cell = f"**E**, par {f[1]}"
            att = ", ".join(sorted((a for a in attend if a in ouverts), key=cle)) or "—"
            deb = ", ".join(sorted(debloque[p], key=cle)) or "—"
            lignes.append(f"| **{p}** {titres[p]} | {sysm} | {maint} | {att} | {deb} | {cell} |")
    return "\n".join(lignes)


def ecarts(liste: str, registre: str) -> list[str]:
    """Ce qui fait que le registre ne suit plus la liste. Né en S352 : aucun contre-exemple réel encore."""
    titres, ouverts = points(liste)
    found = [f"dépendances : point ouvert {p} absent du registre" for p in sorted(ouverts - set(D), key=cle)]
    found += [f"dépendances : {p} n'est pas un point de la liste" for p in sorted(set(D) - set(titres), key=cle)]
    found += [f"dépendances : {p} attend {a}, inconnu" for p, v in D.items() for a in v[2] if a not in titres]
    if not found:
        _, cycles = fronts(ouverts)
        found += [f"dépendances : cycle {c}" for c in cycles]
    if not found and registre:
        bloc = registre.partition(DEBUT)[2].partition(FIN)[0].strip()
        if bloc != tables(liste).strip():
            found.append("dépendances : les tables du registre ne sont plus celles des données — "
                         "python outils/dependances_liste.py --ecrire")
    return found


def main() -> int:
    liste = (ROOT / LISTE).read_text(encoding="utf-8")
    chemin = ROOT / REGISTRE
    registre = chemin.read_text(encoding="utf-8") if chemin.is_file() else ""
    if "--ecrire" in sys.argv:
        avant, trouve, reste = registre.partition(DEBUT)
        _, _, apres = reste.partition(FIN)
        if not trouve:
            print(f"{REGISTRE} : marqueurs absents", file=sys.stderr)
            return 1
        with io.open(chemin, "w", encoding="utf-8", newline="") as f:
            f.write(avant + DEBUT + "\n\n" + tables(liste) + "\n\n" + FIN + apres)
        registre = chemin.read_text(encoding="utf-8")
    titres, ouverts = points(liste)
    fr, _ = fronts(ouverts)
    compte = Counter("E" if isinstance(f, tuple) else str(f) for f in fr.values())
    print(f"points ouverts {len(ouverts)}, fronts {dict(sorted(compte.items()))}, "
          f"systèmes {dict(sorted(Counter(D[p][0] for p in fr).items()))}")
    anomalies = ecarts(liste, registre)
    for a in anomalies:
        print(a)
    return int("--check" in sys.argv and bool(anomalies))


if __name__ == "__main__":
    sys.stdout.reconfigure(encoding="utf-8")
    raise SystemExit(main())
