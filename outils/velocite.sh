#!/bin/sh
# Compatibilité des anciens renvois ; une seule implémentation, portable (S227).
set -eu
cd "$(dirname "$0")/.."
exec python outils/etat_projet.py "$@"
