#!/usr/bin/env python3
"""**S721 (ADR-282 D1) — fermer une session, d'un seul geste.**

Exécute, dans l'ordre, et s'arrête au premier échec **sans rien commettre** :

1. le script de fin de la session (la preuve, la liste, l'index, EN-COURS, le journal), s'il est donné ;
2. la compilation des essais (`cargo test --no-run`) ;
3. le rituel de fin (`outils/rituel.py fin …`) ;
4. le commit de l'étape, avec le message donné et la ligne d'attribution.

Deux fois (S702, S719), une chaîne de commandes séparées par `;` a commis un état partiel après l'échec d'un script de fin. Ici, un échec
arrête tout.

    python outils/fermer.py --session S721 --suivante "<texte>" --maillons 0 [--lot] [--script "<fin.py> [arguments]"] --message "<titre du commit>"
"""
import argparse
import shlex
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ATTRIBUTION = "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
CHEMINS = ["code", "docs", "notes", "REPRISE.md", "BOUSSOLE.md", "outils", "references"]


def etape(nom, cmd, **kw):
    print(f"— {nom}")
    r = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True, encoding="utf-8", errors="replace", **kw)
    sortie = (r.stdout or "") + (r.stderr or "")
    lignes = [l for l in sortie.splitlines() if l.strip()]
    for l in lignes[-4:]:
        print("  ", l)
    if r.returncode != 0:
        print(f"ÉCHEC : {nom} (code {r.returncode}) — rien n'est commis.")
        sys.exit(1)
    return sortie


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--session", required=True)
    ap.add_argument("--suivante", required=True)
    ap.add_argument("--maillons", default="0")
    ap.add_argument("--lot", action="store_true")
    ap.add_argument("--sans-banc", default=None)
    ap.add_argument("--script", default=None)
    ap.add_argument("--message", required=True)
    a = ap.parse_args()
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if a.script:
        # Le script et ses arguments : `--script "<fin.py> fin"`.
        etape("le script de fin", [sys.executable] + shlex.split(a.script, posix=False))
    etape("la compilation des essais", ["cargo", "test", "--manifest-path", "code/Cargo.toml", "--release", "--offline", "-p", "water-core",
                                        "--lib", "--no-run"])
    rituel = [sys.executable, "outils/rituel.py", "fin", "--session", a.session, "--suivante", a.suivante, "--maillons", a.maillons]
    if a.lot:
        rituel.append("--lot")
    if a.sans_banc:
        rituel += ["--sans-banc", a.sans_banc]
    sortie = etape("le rituel", rituel)
    if "MANQUE" in sortie or "etat_projet --check = 0" not in sortie:
        print("ÉCHEC : le rituel signale un manque — rien n'est commis.")
        sys.exit(1)
    etape("l'ajout", ["git", "add", "-A"] + [c for c in CHEMINS if (ROOT / c).exists()])
    message = ROOT / "calculs" / "message_commit.txt"
    message.write_text(f"{a.message}\n\n{ATTRIBUTION}\n", encoding="utf-8")
    etape("le commit", ["git", "commit", "-qF", str(message)])
    print(etape("le dernier commit", ["git", "log", "--oneline", "-1"]).strip())
    return 0


if __name__ == "__main__":
    sys.exit(main())
