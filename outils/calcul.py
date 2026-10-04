# -*- coding: utf-8 -*-
"""Les calculs longs, hors de la conversation — S480.

Un calcul de plus de dix minutes lancé depuis une conversation meurt avec elle, et sa sortie, écrite dans le dossier temporaire de
la conversation, disparaît avec lui (S479 : `b10_p24.txt` et `bulle_b.txt` n'existaient que là). Cet outil lance la commande dans
un processus détaché, écrit sa sortie dans `calculs/<id>/` (dans le dépôt, non versionné) et inscrit le calcul dans
`notes/CALCULS.md` (versionné) : la session suivante le retrouve par le registre, quelle que soit la conversation.

    python outils/calcul.py lancer <nom> [--session S480] [VAR=valeur …] -- <commande> [arguments…]
    python outils/calcul.py etat            # met à jour la colonne « état » du registre depuis calculs/, et l'affiche
    python outils/calcul.py garder <nom> <fichier>…   # met à l'abri des sorties déjà produites ailleurs

Dans `calculs/<id>/` : `commande.txt`, `sortie.log` (sortie et erreurs), `pid`, et `fin.txt` (code de sortie, heure) écrit par le
processus lui-même quand la commande se termine. Un calcul sans `fin.txt` dont le processus n'existe plus est « interrompu ».
La commande s'exécute à la racine du dépôt ; les `VAR=valeur` avant `--` lui sont données et s'inscrivent avec elle.
"""
import ctypes
import os
import shlex
import shutil
import subprocess
import sys
from datetime import datetime
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOSSIER = ROOT / "calculs"
REGISTRE = ROOT / "notes/CALCULS.md"
ENTETE = """# Calculs longs — le registre

*Tenu par `python outils/calcul.py` (S480) ; la colonne « état » se met à jour par `python outils/calcul.py etat`.* Un calcul de plus
de dix minutes se lance par l'outil : il survit à la conversation, sa sortie est dans `calculs/<id>/` (non versionné, dans le
dépôt). Ce qu'une session tire d'une sortie va dans sa preuve (`docs/validation/`) ; la sortie brute reste dans `calculs/`.

| id | session | nom | commande | état |
|---|---|---|---|---|
"""


def maintenant() -> str:
    return datetime.now().astimezone().strftime("%Y-%m-%d %H:%M")


def vivant(pid: int) -> bool:
    """Le processus existe-t-il encore ? (Windows : OpenProcess et GetExitCodeProcess ; ailleurs : kill 0.)"""
    if os.name != "nt":
        try:
            os.kill(pid, 0)
            return True
        except OSError:
            return False
    k = ctypes.windll.kernel32
    h = k.OpenProcess(0x1000, False, pid)  # PROCESS_QUERY_LIMITED_INFORMATION
    if not h:
        return False
    code = ctypes.c_ulong()
    ok = k.GetExitCodeProcess(h, ctypes.byref(code))
    k.CloseHandle(h)
    return bool(ok) and code.value == 259  # STILL_ACTIVE


def etat(d: Path) -> str:
    fin = d / "fin.txt"
    if fin.exists():
        code, quand = (fin.read_text(encoding="utf-8").split("\n") + [""])[:2]
        return ("terminé" if code.strip() == "0" else f"échec (code {code.strip()})") + f", {quand.strip()}"
    if (d / "garde.txt").exists():
        return "sorties gardées"
    pid = d / "pid"
    if pid.exists() and vivant(int(pid.read_text().strip())):
        return "en cours"
    return "interrompu (pas de fin.txt)"


def inscrire(ident: str, session: str, nom: str, commande: str, etat_: str) -> None:
    texte = REGISTRE.read_text(encoding="utf-8") if REGISTRE.exists() else ENTETE
    cmd = commande.replace("|", "\\|")
    texte = texte.rstrip("\n") + f"\n| `{ident}` | {session} | {nom} | `{cmd}` | {etat_} |\n"
    REGISTRE.write_text(texte, encoding="utf-8", newline="\n")


def mettre_a_jour() -> list:
    if not REGISTRE.exists():
        return []
    lignes, vus = [], []
    for l in REGISTRE.read_text(encoding="utf-8").splitlines():
        if l.startswith("| `"):
            cases = l.split(" | ")
            ident = cases[0][3:-1]
            d = DOSSIER / ident
            if d.exists():
                cases[-1] = etat(d) + " |"
                l = " | ".join(cases)
            vus.append(l)
        lignes.append(l)
    REGISTRE.write_text("\n".join(lignes) + "\n", encoding="utf-8", newline="\n")
    return vus


def lancer(nom: str, session: str, commande: list, env: dict | None = None) -> Path:
    ident = datetime.now().strftime("%Y%m%d-%H%M%S") + "-" + nom
    d = DOSSIER / ident
    d.mkdir(parents=True)
    texte = subprocess.list2cmdline(commande) if os.name == "nt" else shlex.join(commande)
    texte = " ".join(f"{k}={v}" for k, v in (env or {}).items()) + (" " if env else "") + texte
    (d / "commande.txt").write_text(texte + "\n", encoding="utf-8")
    sortie = open(d / "sortie.log", "wb")
    options = {}
    if os.name == "nt":
        options["creationflags"] = 0x00000008 | 0x00000200 | 0x08000000  # DETACHED, NEW_PROCESS_GROUP, NO_WINDOW
    else:
        options["start_new_session"] = True
    p = subprocess.Popen([sys.executable, str(Path(__file__).resolve()), "_executer", str(d), "--", *commande], cwd=ROOT,
                         stdin=subprocess.DEVNULL, stdout=sortie, stderr=subprocess.STDOUT, close_fds=True,
                         env=dict(os.environ, **(env or {})), **options)
    (d / "pid").write_text(str(p.pid), encoding="utf-8")
    inscrire(ident, session, nom, texte, f"lancé {maintenant()}")
    return d


def executer(d: Path, commande: list) -> int:
    """Dans le processus détaché : exécute la commande, puis écrit fin.txt — même si elle échoue."""
    (d / "pid").write_text(str(os.getpid()), encoding="utf-8")
    # Un programme donné par un chemin relatif au dépôt (S481 : `viewer/target/release/…` introuvable sous Windows sans shell).
    if commande and (ROOT / commande[0]).is_file():
        commande = [str(ROOT / commande[0])] + commande[1:]
    try:
        # les poignées explicites : un processus détaché n'a pas de console, la commande écrirait dans le vide (S480)
        env = dict(os.environ, PYTHONIOENCODING="utf-8")
        code = subprocess.call(commande, cwd=ROOT, stdin=subprocess.DEVNULL, stdout=sys.stdout, stderr=sys.stderr, env=env)
    except OSError as e:
        print(f"calcul.py : {e}", flush=True)
        code = 127
    (d / "fin.txt").write_text(f"{code}\n{maintenant()}\n", encoding="utf-8")
    return code


def garder(nom: str, session: str, fichiers: list) -> Path:
    ident = datetime.now().strftime("%Y%m%d-%H%M%S") + "-" + nom
    d = DOSSIER / ident
    d.mkdir(parents=True)
    for f in fichiers:
        shutil.copy2(f, d / Path(f).name)
    (d / "garde.txt").write_text("\n".join(str(Path(f)) for f in fichiers) + "\n", encoding="utf-8")
    inscrire(ident, session, nom, "garder " + " ".join(Path(f).name for f in fichiers), "sorties gardées")
    return d


def main(argv) -> int:
    sys.stdout.reconfigure(encoding="utf-8")
    if not argv or argv[0] in ("-h", "--help"):
        print(__doc__)
        return 0
    action, reste = argv[0], argv[1:]
    session = ""
    if "--session" in reste and ("--" not in reste or reste.index("--session") < reste.index("--")):
        i = reste.index("--session")
        session = reste[i + 1]
        reste = reste[:i] + reste[i + 2:]
    if action == "_executer":
        return executer(Path(reste[0]), reste[reste.index("--") + 1:])
    if action == "lancer":
        i = reste.index("--") if "--" in reste else -1
        env = dict(x.split("=", 1) for x in reste[1:i]) if i > 0 else {}
        if i < 1 or any("=" not in x for x in reste[1:i]):
            print("usage : calcul.py lancer <nom> [--session Snnn] [VAR=valeur …] -- <commande> …")
            return 2
        d = lancer(reste[0], session, reste[i + 1:], env)
        print(f"CALCUL lancé : {d.relative_to(ROOT)} (sortie.log, fin.txt à la fin)")
        return 0
    if action == "garder":
        d = garder(reste[0], session, reste[1:])
        print(f"CALCUL gardé : {d.relative_to(ROOT)}")
        return 0
    if action == "etat":
        for l in mettre_a_jour():
            print(l)
        return 0
    print(f"action inconnue : {action}")
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
