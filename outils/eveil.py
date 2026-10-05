# -*- coding: utf-8 -*-
"""Garder la machine éveillée pendant le travail autonome — S491 (demande de l'utilisateur : « mon ordinateur se met en veille et plus
possible d'avancer »).

Demande à Windows « système requis » (`SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED)`) — ce que fait un lecteur vidéo :
la mise en veille **sur inactivité** est retenue tant que ce processus vit ; l'écran peut s'éteindre ; rien n'est modifié dans les
réglages, et la demande disparaît avec le processus. Elle n'empêche ni la fermeture du capot ni une mise en veille demandée.

    python outils/calcul.py lancer eveil -- python outils/eveil.py <heures>     # hors de la session (WMI), le défaut : 12 h
    python outils/eveil.py --arret                                              # l'arrêter avant l'heure

Le fichier `calculs/eveil.actif` existe tant que la demande tient (son contenu : l'heure de fin) ; l'effacer l'arrête au plus tard une
minute après.
"""
import ctypes
import sys
import time
from datetime import datetime, timedelta
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DRAPEAU = ROOT / "calculs" / "eveil.actif"
ES_CONTINUOUS = 0x80000000
ES_SYSTEM_REQUIRED = 0x00000001


def main(argv) -> int:
    if "--arret" in argv:
        DRAPEAU.unlink(missing_ok=True)
        print("EVEIL : arrêt demandé")
        return 0
    heures = float(argv[0]) if argv else 12.
    fin = datetime.now() + timedelta(hours=heures)
    DRAPEAU.parent.mkdir(exist_ok=True)
    DRAPEAU.write_text(fin.strftime("%Y-%m-%d %H:%M") + "\n", encoding="utf-8")
    k = ctypes.windll.kernel32
    print(f"EVEIL : système requis jusqu'à {fin:%Y-%m-%d %H:%M}", flush=True)
    try:
        while datetime.now() < fin and DRAPEAU.exists():
            k.SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED)
            time.sleep(60)
    finally:
        k.SetThreadExecutionState(ES_CONTINUOUS)
        DRAPEAU.unlink(missing_ok=True)
    print("EVEIL : fin", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
