#!/usr/bin/env python3
"""**S761 (ADR-295 D3) — commettre une étape de session, d'un seul geste.**

Exécute le script de l'étape, s'il est donné, puis commet avec le message et la ligne d'attribution ; s'arrête au premier échec **sans
rien commettre**. En S760, un script de plan en échec, suivi d'un `git add -A` sur une autre ligne, a commis un état étranger sous le titre
du plan.

    python outils/etape.py [--script "<script.py> [arguments]"] --message "S761 P2 — <description>" [--pousser]
"""
import argparse
import shlex
import sys

from fermer import ATTRIBUTION, CHEMINS, ROOT, etape


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--script", default=None)
    ap.add_argument("--message", required=True)
    ap.add_argument("--pousser", action="store_true")
    a = ap.parse_args()
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    if a.script:
        etape("le script de l'étape", [sys.executable] + shlex.split(a.script, posix=False))
    etape("l'ajout", ["git", "add", "-A"] + [c for c in CHEMINS if (ROOT / c).exists()])
    message = ROOT / "calculs" / "message_commit.txt"
    message.write_text(f"{a.message}\n\n{ATTRIBUTION}\n", encoding="utf-8")
    etape("le commit", ["git", "commit", "-qF", str(message)])
    print(etape("le dernier commit", ["git", "log", "--oneline", "-1"]).strip())
    if a.pousser:
        etape("la poussée", ["git", "push", "-q", "origin", "HEAD"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
