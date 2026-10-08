#!/usr/bin/env python3
"""Lancer un essai long du cœur sans charger la discussion — S690 (ANALYSE-PHASES-S690 §4).

    python outils/essai.py <nom de l'essai> [--marque S690] [--ignore] [--sans-compiler]

1. compile les essais du cœur (`cargo test --release --offline -p water-core --lib --no-run`) ;
2. copie le binaire dans `calculs/` (ADR-265 D1 : un essai long tourne depuis une copie, la compilation suivante ne le gêne pas) ;
3. lance l'essai (`--ignored` si demandé), le journal complet dans `calculs/essai_<nom>.log` ;
4. n'affiche que les lignes utiles : celles qui portent la marque (par défaut, le préfixe `S<nnn>` tiré du nom), la progression, les
   échecs ; puis un résumé : la durée, le résultat.

Les essais longs affichent eux-mêmes leur progression (`eprintln!("S690 progression : …")`, ADR-274 D1).
"""
import argparse
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CODE = ROOT / "code"
CALCULS = ROOT / "calculs"


def binaire_des_essais() -> Path:
    sortie = subprocess.run(
        ["cargo", "test", "--manifest-path", str(CODE / "Cargo.toml"), "--release", "--offline", "-p", "water-core", "--lib",
         "--no-run", "--message-format=short"],
        capture_output=True, text=True, encoding="utf-8", errors="replace", cwd=ROOT)
    erreurs = [l for l in sortie.stderr.splitlines() if l.startswith("error")]
    if sortie.returncode != 0 or erreurs:
        print("\n".join(erreurs[:20]) or sortie.stderr[-2000:])
        sys.exit("la compilation a échoué")
    m = re.findall(r"Executable unittests src[\\/]lib\.rs \((.+?)\)", sortie.stderr)
    if not m:
        sys.exit("binaire des essais introuvable")
    return ROOT / m[-1] if not Path(m[-1]).is_absolute() else Path(m[-1])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("nom")
    ap.add_argument("--marque", default=None)
    ap.add_argument("--ignore", action="store_true", help="lancer un essai marqué #[ignore]")
    ap.add_argument("--sans-compiler", action="store_true")
    a = ap.parse_args()
    marque = a.marque or (re.search(r"s\d{3}", a.nom).group(0).upper() if re.search(r"s\d{3}", a.nom) else a.nom)
    CALCULS.mkdir(exist_ok=True)
    copie = CALCULS / f"essai_{a.nom}.exe"
    if not a.sans_compiler:
        shutil.copyfile(binaire_des_essais(), copie)
    journal = CALCULS / f"essai_{a.nom}.log"
    cmd = [str(copie), a.nom, "--nocapture", "--test-threads", "1"] + (["--ignored"] if a.ignore else [])
    debut = time.time()
    utiles = re.compile(rf"({re.escape(marque)}|progression|panicked|test result|critère|error)")
    resultat = None
    with open(journal, "w", encoding="utf-8") as j:
        proc = subprocess.Popen(cmd, cwd=CODE / "water-core", stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
                                encoding="utf-8", errors="replace")
        for ligne in proc.stdout:
            j.write(ligne)
            j.flush()
            if utiles.search(ligne):
                print(f"[{time.time() - debut:6.0f} s] {ligne.rstrip()}", flush=True)
            if ligne.startswith("test result"):
                resultat = ligne.strip()
        proc.wait()
    print(f"— {a.nom} : {resultat or 'aucun résultat'} ; {time.time() - debut:.0f} s ; journal {journal.relative_to(ROOT)}")
    sys.exit(0 if resultat and " 0 failed" in resultat else 1)


if __name__ == "__main__":
    main()
